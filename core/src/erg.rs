//! Decoder for Bosch FLA 203 `.ERG` ("Ergebnis" = result) run files.
//!
//! ## Layout (reverse-engineered, validated against the live 4.1 G disk)
//!
//! A `.ERG` is size-tolerant. The **trailer is always the last 512 bytes**;
//! everything before it is a whole number of 2 KB channel quadrants:
//!
//! ```text
//! full run   8704 B = four 0x800 channel quadrants + 512 B trailer
//! partial    4608 B = two  0x800 channel quadrants + 512 B trailer
//! trailer    512  B = trailer only (aborted/summary save)
//! ```
//!
//! Each quadrant is an array of little-endian `i16` curve samples
//! (power / engine-power / torque / rpm — exact semantics still being pinned
//! down via the emulator oracle), zero-padded at the tail.
//!
//! ### Trailer (512 B) — offsets relative to the start of the trailer
//!
//! | off  | field                       | confidence | note                                   |
//! |------|-----------------------------|------------|----------------------------------------|
//! | 0xAA | day (u8)                    | confirmed  | matches the FAT directory timestamp    |
//! | 0xAB | month (u8)                  | confirmed  |                                        |
//! | 0xB2 | year (u16 LE)               | confirmed  |                                        |
//! | 0x3A | Pnim — rated power [kW]     | confirmed  | screenshot showed 200; byte = 200      |
//! | 0xD2 | Paine — ambient press [hPa] | confirmed  | screenshot showed 975; word = 975      |
//! | 0xDA | Lämp — air temp [°C]        | confirmed  | screenshot showed 21; word = 21        |
//! | 0x72 | rpm-ish                     | tentative  | 1170 vs on-screen 1130 — needs oracle  |
//!
//! The remaining `ERGEB_*` result fields (Pmax, Ppyörä, Phäviö, ΔP, Mmax, k, …)
//! occupy other trailer offsets not yet individually confirmed; use
//! [`Results::trailer_scan`] as the oracle aid to finish the map.

use crate::CoreError;

/// Bytes per channel quadrant (2 KB = 1024 × i16).
const QUAD: usize = 0x800;
/// Trailer length in bytes.
const TRAILER: usize = 0x200;

/// The run date carried inside the trailer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RunDate {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

/// The (up to four) measurement-curve channels, each a raw `i16` sample array
/// with trailing zero padding trimmed. Semantics are tentative pending the
/// emulator-oracle confirmation; kept as `ch0..ch3` to avoid over-claiming.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Channels {
    pub ch0: Vec<i16>,
    pub ch1: Vec<i16>,
    pub ch2: Vec<i16>,
    pub ch3: Vec<i16>,
}

/// Scalar results decoded from the 512-byte trailer.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Results {
    /// Pnim — rated/nominal power [kW] (trailer +0x3A). Confirmed.
    pub pnim_kw: Option<i16>,
    /// Paine — ambient pressure [hPa] (trailer +0xD2). Confirmed.
    pub pressure_hpa: Option<i16>,
    /// Lämp — air/intake temperature [°C] (trailer +0xDA). Confirmed.
    pub temp_c: Option<i16>,
    /// rpm-ish value at trailer +0x72. Tentative — needs the oracle.
    pub rpm_raw: Option<i16>,
    /// Every non-zero `i16` at an even offset in the trailer, as
    /// `(offset, value)`. The raw material for finishing the offset map.
    pub trailer_scan: Vec<(u16, i16)>,
}

/// A fully decoded run.
#[derive(Debug, Clone)]
pub struct DecodedRun {
    /// Original `.ERG` byte length.
    pub size: usize,
    /// Number of channel quadrants present (0, 2, or 4 in practice).
    pub num_channels: usize,
    pub date: Option<RunDate>,
    pub channels: Channels,
    pub results: Results,
    /// The raw 512-byte trailer, retained for re-decoding / oracle work.
    pub raw_trailer: Vec<u8>,
}

fn i16_at(b: &[u8], off: usize) -> Option<i16> {
    if off + 2 <= b.len() {
        Some(i16::from_le_bytes([b[off], b[off + 1]]))
    } else {
        None
    }
}

fn nonzero_i16(b: &[u8], off: usize) -> Option<i16> {
    i16_at(b, off).filter(|&v| v != 0)
}

/// Read one channel quadrant starting at `off`, trimming trailing zeros.
fn read_channel(b: &[u8], off: usize) -> Vec<i16> {
    let end = (off + QUAD).min(b.len());
    if off >= end {
        return Vec::new();
    }
    let mut v: Vec<i16> = b[off..end]
        .chunks_exact(2)
        .map(|c| i16::from_le_bytes([c[0], c[1]]))
        .collect();
    while matches!(v.last(), Some(0)) {
        v.pop();
    }
    v
}

/// Decode a `.ERG` payload.
pub fn decode(bytes: &[u8]) -> Result<DecodedRun, CoreError> {
    let size = bytes.len();
    if size < TRAILER {
        return Err(CoreError::BadErgSize(size));
    }
    let trailer_off = size - TRAILER;
    let num_channels = (trailer_off / QUAD).min(4);

    let mut channels = Channels::default();
    if num_channels > 0 {
        channels.ch0 = read_channel(bytes, 0);
    }
    if num_channels > 1 {
        channels.ch1 = read_channel(bytes, QUAD);
    }
    if num_channels > 2 {
        channels.ch2 = read_channel(bytes, 2 * QUAD);
    }
    if num_channels > 3 {
        channels.ch3 = read_channel(bytes, 3 * QUAD);
    }

    let trailer = &bytes[trailer_off..trailer_off + TRAILER];

    let date = {
        let day = trailer.get(0xAA).copied();
        let month = trailer.get(0xAB).copied();
        let year = i16_at(trailer, 0xB2).map(|y| y as u16);
        match (day, month, year) {
            (Some(d), Some(m), Some(y))
                if (1..=31).contains(&d) && (1..=12).contains(&m) && (1980..=2100).contains(&y) =>
            {
                Some(RunDate {
                    year: y,
                    month: m,
                    day: d,
                })
            }
            _ => None,
        }
    };

    let mut trailer_scan = Vec::new();
    let mut o = 0;
    while o + 2 <= TRAILER {
        let v = i16::from_le_bytes([trailer[o], trailer[o + 1]]);
        if v != 0 {
            trailer_scan.push((o as u16, v));
        }
        o += 2;
    }

    let results = Results {
        pnim_kw: nonzero_i16(trailer, 0x3A),
        pressure_hpa: nonzero_i16(trailer, 0xD2),
        temp_c: nonzero_i16(trailer, 0xDA),
        rpm_raw: nonzero_i16(trailer, 0x72),
        trailer_scan,
    };

    Ok(DecodedRun {
        size,
        num_channels,
        date,
        channels,
        results,
        raw_trailer: trailer.to_vec(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn put_i16(b: &mut [u8], off: usize, v: i16) {
        b[off..off + 2].copy_from_slice(&v.to_le_bytes());
    }

    #[test]
    fn full_run_synthetic() {
        let mut b = vec![0u8; 8704];
        let t = 8704 - TRAILER;
        b[t + 0xAA] = 5; // day
        b[t + 0xAB] = 5; // month
        put_i16(&mut b, t + 0xB2, 2000); // year
        put_i16(&mut b, t + 0x3A, 150); // pnim
        put_i16(&mut b, t + 0xD2, 999); // pressure
        put_i16(&mut b, t + 0xDA, 40); // temp
        put_i16(&mut b, t + 0x72, 6260); // rpm-ish
        put_i16(&mut b, 0, 123); // one ch0 sample
        put_i16(&mut b, 2, 456);

        let r = decode(&b).unwrap();
        assert_eq!(r.num_channels, 4);
        assert_eq!(r.date.unwrap(), RunDate { year: 2000, month: 5, day: 5 });
        assert_eq!(r.results.pnim_kw, Some(150));
        assert_eq!(r.results.pressure_hpa, Some(999));
        assert_eq!(r.results.temp_c, Some(40));
        assert_eq!(r.results.rpm_raw, Some(6260));
        assert_eq!(r.channels.ch0, vec![123, 456]);
        assert!(r.channels.ch3.is_empty()); // no data written there
    }

    #[test]
    fn partial_4608_two_channels() {
        let mut b = vec![0u8; 4608];
        let t = 4608 - TRAILER;
        put_i16(&mut b, t + 0xB2, 2015);
        b[t + 0xAA] = 12;
        b[t + 0xAB] = 3;
        let r = decode(&b).unwrap();
        assert_eq!(r.num_channels, 2);
        assert_eq!(r.date.unwrap(), RunDate { year: 2015, month: 3, day: 12 });
    }

    #[test]
    fn trailer_only_512() {
        let mut b = vec![0u8; 512];
        b[0xAA] = 1;
        b[0xAB] = 1;
        put_i16(&mut b, 0xB2, 2010);
        let r = decode(&b).unwrap();
        assert_eq!(r.num_channels, 0);
        assert_eq!(r.date.unwrap().year, 2010);
    }

    #[test]
    fn too_small_errors() {
        assert_eq!(decode(&[0u8; 100]).unwrap_err(), CoreError::BadErgSize(100));
    }

    #[test]
    fn missing_date_is_none() {
        let b = vec![0u8; 8704]; // all zeros -> invalid month/day
        assert!(decode(&b).unwrap().date.is_none());
    }
}
