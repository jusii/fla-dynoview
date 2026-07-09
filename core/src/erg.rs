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
//! ### Channel quadrants (each 0x800 = 1024 × i16 little-endian, zero-padded)
//!
//! Confirmed by correlating a known run against the machine's on-screen values:
//!
//! | quadrant | meaning                              |
//! |----------|--------------------------------------|
//! | ch0      | **wheel power** ×10  [kW]            |
//! | ch1      | **wheel-loss power** ×10 [kW] (≤0)   |
//! | ch2      | secondary curve (drag/torque aux)    |
//! | ch3      | **engine rpm**                       |
//!
//! Derived quantities (see [`Curves`]):
//! - DIN correction `k = (1013 / Paine_hPa) · √((273 + Lämp_°C) / 293)`
//! - `engine_kw = (ch0 − ch1) / 10 · k`   (wheel + |loss|, corrected)
//! - `torque_nm = engine_kw · 9549.296 / rpm`   (60000 / 2π)
//!
//! ### Trailer (512 B) — offsets relative to the start of the trailer
//!
//! | off  | field                       | note                              |
//! |------|-----------------------------|-----------------------------------|
//! | 0xAA | day (u8)                    | matches the FAT directory date    |
//! | 0xAB | month (u8)                  |                                   |
//! | 0xB2 | year (u16 LE)               |                                   |
//! | 0x3A | Pnim — rated power [kW]     | confirmed                         |
//! | 0xD2 | Paine — ambient press [hPa] | confirmed (feeds DIN `k`)         |
//! | 0xDA | Lämp — air temp [°C]        | confirmed (feeds DIN `k`)         |
//!
//! The peak scalars the machine shows (Pmax, Ppyörä, Phäviö, Mmax, …) are not
//! stored — they are *computed* from the channels, which is what [`decode`] does.

use crate::CoreError;

/// Bytes per channel quadrant (2 KB = 1024 × i16).
const QUAD: usize = 0x800;
/// Trailer length in bytes.
const TRAILER: usize = 0x200;
/// 60000 / (2π): converts kW and rpm to N·m.
const TORQUE_CONST: f32 = 9549.296;

/// The run date carried inside the trailer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RunDate {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

/// The (up to four) raw measurement-curve channels (`i16` samples, trailing
/// zero padding trimmed).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Channels {
    pub ch0: Vec<i16>,
    pub ch1: Vec<i16>,
    pub ch2: Vec<i16>,
    pub ch3: Vec<i16>,
}

/// Physically-calibrated curves derived from the raw channels (present only for
/// full four-channel runs).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Curves {
    /// Engine rpm (ch3), per sample.
    pub rpm: Vec<i32>,
    /// Wheel power [kW].
    pub wheel_kw: Vec<f32>,
    /// Wheel-loss power [kW] (≤ 0).
    pub loss_kw: Vec<f32>,
    /// Engine power, DIN-corrected [kW].
    pub engine_kw: Vec<f32>,
    /// Torque [N·m].
    pub torque_nm: Vec<f32>,
}

/// Scalar results: a few are stored in the trailer, the peaks are computed.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Results {
    // --- stored in the trailer ---
    /// Pnim — rated/nominal power [kW] (trailer +0x3A).
    pub pnim_kw: Option<i16>,
    /// Paine — ambient pressure [hPa] (trailer +0xD2).
    pub pressure_hpa: Option<i16>,
    /// Lämp — air/intake temperature [°C] (trailer +0xDA).
    pub temp_c: Option<i16>,
    // --- computed from the channels ---
    /// DIN correction factor.
    pub k_din: Option<f32>,
    /// Pmax — peak engine power [kW].
    pub pmax_kw: Option<f32>,
    /// Engine rpm at Pmax.
    pub rpm_at_pmax: Option<i32>,
    /// Ppyörä — peak wheel power [kW].
    pub ppyora_kw: Option<f32>,
    /// Phäviö — drivetrain loss [kW] at the Pmax point.
    pub phavio_kw: Option<f32>,
    /// Mmax — peak torque [N·m].
    pub mmax_nm: Option<f32>,
    /// Engine rpm at Mmax.
    pub rpm_at_mmax: Option<i32>,
    /// Every non-zero `i16` at an even offset in the trailer (oracle aid).
    pub trailer_scan: Vec<(u16, i16)>,
}

/// A fully decoded run.
#[derive(Debug, Clone)]
pub struct DecodedRun {
    pub size: usize,
    pub num_channels: usize,
    pub date: Option<RunDate>,
    pub channels: Channels,
    pub curves: Curves,
    pub results: Results,
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

/// DIN 70020 correction factor from ambient pressure/temperature.
fn din_k(pressure_hpa: Option<i16>, temp_c: Option<i16>) -> Option<f32> {
    match (pressure_hpa, temp_c) {
        (Some(p), Some(t)) if p > 0 => {
            Some((1013.0 / p as f32) * ((273.0 + t as f32) / 293.0).sqrt())
        }
        _ => None,
    }
}

fn argmax(v: &[f32]) -> Option<usize> {
    v.iter()
        .enumerate()
        .fold(None, |acc, (i, &x)| match acc {
            Some((_, best)) if x <= best => acc,
            _ => Some((i, x)),
        })
        .map(|(i, _)| i)
}

/// Compute the calibrated curves from the raw channels and DIN factor.
fn compute_curves(ch: &Channels, k: f32) -> Curves {
    let n = ch.ch0.len().min(ch.ch1.len()).min(ch.ch3.len());
    let mut c = Curves::default();
    for i in 0..n {
        let wheel = ch.ch0[i] as f32 / 10.0;
        let loss = ch.ch1[i] as f32 / 10.0;
        let rpm = ch.ch3[i] as i32;
        let engine = (ch.ch0[i] as f32 - ch.ch1[i] as f32) / 10.0 * k;
        let torque = if rpm > 0 {
            engine * TORQUE_CONST / rpm as f32
        } else {
            0.0
        };
        c.rpm.push(rpm);
        c.wheel_kw.push(wheel);
        c.loss_kw.push(loss);
        c.engine_kw.push(engine);
        c.torque_nm.push(torque);
    }
    c
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

    let pressure_hpa = nonzero_i16(trailer, 0xD2);
    let temp_c = nonzero_i16(trailer, 0xDA);
    let k_din = din_k(pressure_hpa, temp_c);

    let curves = match k_din {
        Some(k) if num_channels >= 4 => compute_curves(&channels, k),
        _ => Curves::default(),
    };

    // Computed peak scalars.
    let (pmax_kw, rpm_at_pmax, ppyora_kw, phavio_kw) = match argmax(&curves.engine_kw) {
        Some(pi) => (
            Some(curves.engine_kw[pi]),
            Some(curves.rpm[pi]),
            curves.wheel_kw.iter().cloned().fold(None, |a: Option<f32>, x| {
                Some(a.map_or(x, |m| m.max(x)))
            }),
            Some(-curves.loss_kw[pi]),
        ),
        None => (None, None, None, None),
    };
    let (mmax_nm, rpm_at_mmax) = match argmax(&curves.torque_nm) {
        Some(mi) => (Some(curves.torque_nm[mi]), Some(curves.rpm[mi])),
        None => (None, None),
    };

    let results = Results {
        pnim_kw: nonzero_i16(trailer, 0x3A),
        pressure_hpa,
        temp_c,
        k_din,
        pmax_kw,
        rpm_at_pmax,
        ppyora_kw,
        phavio_kw,
        mmax_nm,
        rpm_at_mmax,
        trailer_scan,
    };

    Ok(DecodedRun {
        size,
        num_channels,
        date,
        channels,
        curves,
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
        b[t + 0xAA] = 5;
        b[t + 0xAB] = 5;
        put_i16(&mut b, t + 0xB2, 2000);
        put_i16(&mut b, t + 0x3A, 150);
        put_i16(&mut b, t + 0xD2, 1013);
        put_i16(&mut b, t + 0xDA, 20);
        put_i16(&mut b, 0, 123);
        put_i16(&mut b, 2, 456);

        let r = decode(&b).unwrap();
        assert_eq!(r.num_channels, 4);
        assert_eq!(r.date.unwrap(), RunDate { year: 2000, month: 5, day: 5 });
        assert_eq!(r.results.pnim_kw, Some(150));
        assert_eq!(r.results.pressure_hpa, Some(1013));
        // 1013 hPa, 20 °C -> k ~ 1.0 (reference conditions)
        let k = r.results.k_din.unwrap();
        assert!((k - 1.0).abs() < 0.01, "k={k}");
        assert_eq!(r.channels.ch0, vec![123, 456]);
    }

    #[test]
    fn din_k_reference() {
        // 975 hPa, 21 °C -> 1.041 (matches the machine's readout)
        let k = din_k(Some(975), Some(21)).unwrap();
        assert!((k - 1.041).abs() < 0.002, "k={k}");
        // 974 hPa, 38 °C -> 1.072
        let k2 = din_k(Some(974), Some(38)).unwrap();
        assert!((k2 - 1.072).abs() < 0.002, "k2={k2}");
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
        assert!(r.curves.engine_kw.is_empty());
    }

    #[test]
    fn too_small_errors() {
        assert_eq!(decode(&[0u8; 100]).unwrap_err(), CoreError::BadErgSize(100));
    }
}
