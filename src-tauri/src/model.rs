//! Serializable DTOs handed to the frontend (camelCase JSON), plus conversions
//! from the pure-`std` [`fladyno_core`] types.

use fladyno_core::{DecodedRun, Results, RunDate};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// SHA-256 hex digest of raw bytes (run identity / dedup key).
pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    let d = h.finalize();
    let mut s = String::with_capacity(64);
    for b in d {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

/// Format a run date as `YYYY-MM-DD`.
pub fn fmt_date(d: &RunDate) -> String {
    format!("{:04}-{:02}-{:02}", d.year, d.month, d.day)
}

/// One `.ERG` entry as listed from an image's `\DAT` directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunEntry {
    pub name: String,
    pub path: String,
    pub size: u32,
    pub deleted: bool,
    pub sha256: String,
    pub date: Option<String>,
}

/// Summary of an opened floppy image.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageSummary {
    pub image_path: String,
    pub shop_name: String,
    pub runs: Vec<RunEntry>,
}

/// Shop/owner header from `FLA.CFG`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShopInfo {
    pub name: String,
}

/// The four measurement-curve channels (raw `i16` samples).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelsDto {
    pub ch0: Vec<i16>,
    pub ch1: Vec<i16>,
    pub ch2: Vec<i16>,
    pub ch3: Vec<i16>,
}

/// Decoded trailer scalars.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultsDto {
    pub pnim_kw: Option<i16>,
    pub pressure_hpa: Option<i16>,
    pub temp_c: Option<i16>,
    pub rpm_raw: Option<i16>,
    /// `(offset, value)` of every non-zero even-offset i16 in the trailer.
    pub trailer_scan: Vec<(u16, i16)>,
}

/// A fully decoded run for charting/detail views.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecodedRunDto {
    pub size: usize,
    pub num_channels: usize,
    pub date: Option<String>,
    pub sha256: String,
    pub channels: ChannelsDto,
    pub results: ResultsDto,
}

/// Resolved on-disk locations (for a "reveal database" affordance).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paths {
    pub db_dir: String,
    pub backups_dir: String,
}

impl From<&Results> for ResultsDto {
    fn from(r: &Results) -> Self {
        ResultsDto {
            pnim_kw: r.pnim_kw,
            pressure_hpa: r.pressure_hpa,
            temp_c: r.temp_c,
            rpm_raw: r.rpm_raw,
            trailer_scan: r.trailer_scan.clone(),
        }
    }
}

impl DecodedRunDto {
    pub fn from_core(r: &DecodedRun, sha256: String) -> Self {
        DecodedRunDto {
            size: r.size,
            num_channels: r.num_channels,
            date: r.date.as_ref().map(fmt_date),
            sha256,
            channels: ChannelsDto {
                ch0: r.channels.ch0.clone(),
                ch1: r.channels.ch1.clone(),
                ch2: r.channels.ch2.clone(),
                ch3: r.channels.ch3.clone(),
            },
            results: ResultsDto::from(&r.results),
        }
    }
}
