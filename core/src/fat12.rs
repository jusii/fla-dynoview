//! Read-only FAT12 reader for the Bosch FLA 203 1.44 MB floppy images.
//!
//! Direct port of `tools/fat12.py`. Assumes the standard 1.44 MB geometry used
//! by every disk in this project: 512 B/sector, 1 sector/cluster, 2 FATs × 9
//! sectors, 224 root entries, 2880 sectors. Enumerates deleted entries too, so
//! the app can offer to recover carved-out `.ERG` runs (the `fatfs` crate used
//! for the write path cannot see deleted entries).

use crate::cp437::cp437_to_string;
use crate::CoreError;
use std::collections::HashSet;

const SECT: usize = 512;
const FAT0_LBA: usize = 1;
const FAT_SECTS: usize = 9;
const ROOT_LBA: usize = 19;
const ROOT_SECTS: usize = 14;
const DATA_LBA: usize = 33; // cluster 2 starts here

/// Exact byte length of a standard 1.44 MB floppy image (2880 × 512).
pub const IMG_SIZE: usize = 2880 * SECT; // 1_474_560

/// A decoded FAT directory date/time (local time, as the DOS entry stores it).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FatDateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
}

/// A single directory entry (file or subdirectory), live or deleted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirEntry {
    /// Full path from the root, e.g. `"DAT/1.ERG"`. For deleted entries the
    /// lost first filename character shows as `?` (e.g. `"DAT/?2.ERG"`).
    pub path: String,
    /// Bare `NAME.EXT`.
    pub name: String,
    pub attr: u8,
    pub start: u16,
    pub size: u32,
    pub deleted: bool,
    pub is_dir: bool,
    pub is_vol: bool,
    pub mtime: Option<FatDateTime>,
}

/// An in-memory FAT12 floppy image.
pub struct Fat12 {
    data: Vec<u8>,
}

impl Fat12 {
    /// Wrap a raw image. Fails unless it is exactly [`IMG_SIZE`] bytes.
    pub fn new(data: Vec<u8>) -> Result<Self, CoreError> {
        if data.len() != IMG_SIZE {
            return Err(CoreError::BadImageSize(data.len()));
        }
        Ok(Self { data })
    }

    fn fat(&self) -> &[u8] {
        &self.data[FAT0_LBA * SECT..(FAT0_LBA + FAT_SECTS) * SECT]
    }

    /// The 12-bit FAT entry for cluster `n` (the classic nibble trick).
    fn fat_entry(fat: &[u8], n: usize) -> u16 {
        let off = n * 3 / 2;
        let v = (fat[off] as u16) | ((fat[off + 1] as u16) << 8);
        if n & 1 == 1 {
            v >> 4
        } else {
            v & 0x0FFF
        }
    }

    /// Follow a cluster chain from `start`, stopping on EOF/loops.
    fn chain(fat: &[u8], start: u16) -> Vec<u16> {
        let mut out = Vec::new();
        let mut seen: HashSet<u16> = HashSet::new();
        let mut c = start;
        while (2..0xFF0).contains(&c) {
            if seen.contains(&c) || out.len() > 2849 {
                break;
            }
            seen.insert(c);
            out.push(c);
            c = Self::fat_entry(fat, c as usize);
        }
        out
    }

    fn cluster_lba(c: u16) -> usize {
        DATA_LBA + (c as usize - 2)
    }

    fn sector(&self, lba: usize) -> &[u8] {
        &self.data[lba * SECT..(lba + 1) * SECT]
    }

    fn parse_dir(&self, sectors: &[usize]) -> Vec<DirEntry> {
        let mut raw = Vec::with_capacity(sectors.len() * SECT);
        for &l in sectors {
            raw.extend_from_slice(self.sector(l));
        }
        let mut entries = Vec::new();
        let mut i = 0;
        while i + 32 <= raw.len() {
            let e = &raw[i..i + 32];
            i += 32;
            if e[0] == 0x00 {
                break; // end of directory
            }
            let attr = e[11];
            if attr == 0x0F {
                continue; // long-filename entry (not expected on these disks)
            }
            let deleted = e[0] == 0xE5;
            let mut name_bytes = e[0..8].to_vec();
            if deleted {
                name_bytes[0] = b'?';
            }
            let base = cp437_to_string(&name_bytes).trim_end().to_string();
            let ext = cp437_to_string(&e[8..11]).trim_end().to_string();
            let name = if ext.is_empty() {
                base
            } else {
                format!("{base}.{ext}")
            };
            let t = u16::from_le_bytes([e[22], e[23]]);
            let d = u16::from_le_bytes([e[24], e[25]]);
            let start = u16::from_le_bytes([e[26], e[27]]);
            let size = u32::from_le_bytes([e[28], e[29], e[30], e[31]]);
            entries.push(DirEntry {
                path: name.clone(),
                name,
                attr,
                start,
                size,
                deleted,
                is_dir: attr & 0x10 != 0,
                is_vol: attr & 0x08 != 0,
                mtime: decode_dt(d, t),
            });
        }
        entries
    }

    /// The root directory entries.
    pub fn root(&self) -> Vec<DirEntry> {
        let sectors: Vec<usize> = (ROOT_LBA..ROOT_LBA + ROOT_SECTS).collect();
        self.parse_dir(&sectors)
    }

    /// Every entry (files + dirs, including deleted) with full paths, recursing
    /// into live subdirectories only.
    pub fn list_all(&self) -> Vec<DirEntry> {
        let mut out = Vec::new();
        let root = self.root();
        self.walk(&root, "", &mut out);
        out
    }

    fn walk(&self, entries: &[DirEntry], prefix: &str, out: &mut Vec<DirEntry>) {
        let fat = self.fat();
        for e in entries {
            if e.is_vol || e.name == "." || e.name == ".." {
                continue;
            }
            let path = format!("{prefix}{}", e.name);
            let mut with_path = e.clone();
            with_path.path = path.clone();
            let (deleted, is_dir, start) = (with_path.deleted, with_path.is_dir, with_path.start);
            out.push(with_path);
            if deleted {
                continue; // don't recurse into deleted dirs (chain may be reused)
            }
            if is_dir {
                let sub_lbas: Vec<usize> = Self::chain(fat, start)
                    .iter()
                    .map(|&c| Self::cluster_lba(c))
                    .collect();
                let sub = self.parse_dir(&sub_lbas);
                self.walk(&sub, &format!("{path}/"), out);
            }
        }
    }

    /// Read a file's bytes (truncated to its recorded size).
    pub fn read_entry(&self, e: &DirEntry) -> Vec<u8> {
        let fat = self.fat();
        let mut out = Vec::new();
        for c in Self::chain(fat, e.start) {
            out.extend_from_slice(self.sector(Self::cluster_lba(c)));
            if !e.is_dir && out.len() >= e.size as usize {
                break;
            }
        }
        if !e.is_dir {
            out.truncate(e.size as usize);
        }
        out
    }

    /// Find a **live** entry by case-insensitive path such as `"FLA/FLA.CFG"`.
    pub fn find(&self, path: &str) -> Option<DirEntry> {
        let parts: Vec<String> = path.split('/').map(|p| p.to_uppercase()).collect();
        self.find_in(&self.root(), &parts, "")
    }

    fn find_in(&self, entries: &[DirEntry], parts: &[String], prefix: &str) -> Option<DirEntry> {
        let fat = self.fat();
        for e in entries {
            if e.deleted || e.is_vol {
                continue;
            }
            if e.name.to_uppercase() == parts[0] {
                let path = format!("{prefix}{}", e.name);
                if parts.len() == 1 {
                    let mut hit = e.clone();
                    hit.path = path;
                    return Some(hit);
                }
                if e.is_dir {
                    let lbas: Vec<usize> = Self::chain(fat, e.start)
                        .iter()
                        .map(|&c| Self::cluster_lba(c))
                        .collect();
                    let sub = self.parse_dir(&lbas);
                    return self.find_in(&sub, &parts[1..], &format!("{path}/"));
                }
            }
        }
        None
    }

    /// All `\DAT\*.ERG` entries (live and deleted), in directory order.
    pub fn list_dat_ergs(&self) -> Vec<DirEntry> {
        self.list_all()
            .into_iter()
            .filter(|e| {
                !e.is_dir && {
                    let up = e.path.to_uppercase();
                    up.starts_with("DAT/") && up.ends_with(".ERG")
                }
            })
            .collect()
    }
}

/// Decode a DOS FAT date/time word pair into a [`FatDateTime`], or `None` if the
/// packed fields are out of range.
fn decode_dt(d: u16, t: u16) -> Option<FatDateTime> {
    let year = 1980 + (d >> 9);
    let month = ((d >> 5) & 0xF) as u8;
    let day = (d & 0x1F) as u8;
    let hour = (t >> 11) as u8;
    let min = ((t >> 5) & 0x3F) as u8;
    let sec = ((t & 0x1F) * 2) as u8;
    if (1..=12).contains(&month) && (1..=31).contains(&day) && hour < 24 && min < 60 && sec < 60 {
        Some(FatDateTime {
            year,
            month,
            day,
            hour,
            min,
            sec,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fat12_entry_nibbles() {
        // Two 12-bit entries packed into 3 bytes: 0x123, 0x456 -> 23 61 45
        let fat = [0x23u8, 0x61, 0x45];
        assert_eq!(Fat12::fat_entry(&fat, 0), 0x123);
        assert_eq!(Fat12::fat_entry(&fat, 1), 0x456);
    }

    #[test]
    fn decode_dt_known() {
        // 2000-05-05 15:45:00  (from the project's 01.ERG FAT entry)
        // date = (2000-1980)<<9 | 5<<5 | 5 ; time = 15<<11 | 45<<5 | 0
        let d = (20u16 << 9) | (5 << 5) | 5;
        let t = (15u16 << 11) | (45 << 5);
        let dt = decode_dt(d, t).unwrap();
        assert_eq!(
            (dt.year, dt.month, dt.day, dt.hour, dt.min),
            (2000, 5, 5, 15, 45)
        );
    }

    #[test]
    fn rejects_wrong_size() {
        match Fat12::new(vec![0; 1000]) {
            Err(e) => assert_eq!(e, CoreError::BadImageSize(1000)),
            Ok(_) => panic!("should reject a non-1.44MB image"),
        }
    }
}
