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
    /// Whether this run (by sha256) is already saved in the local library.
    pub in_library: bool,
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
    pub k_din: Option<f32>,
    pub pmax_kw: Option<f32>,
    pub rpm_at_pmax: Option<i32>,
    pub ppyora_kw: Option<f32>,
    pub phavio_kw: Option<f32>,
    pub mmax_nm: Option<f32>,
    pub rpm_at_mmax: Option<i32>,
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

/// Result of resetting (wiping the dyno runs from) an image.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetReport {
    /// Where the pre-reset backup of the image was written.
    pub backup_path: String,
    /// The `.ERG` run files that were removed.
    pub deleted: Vec<String>,
}

/// Resolved on-disk locations (for a "reveal database" affordance).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paths {
    /// The base `fla-dynoview` folder (for a "reveal data folder" action).
    pub root: String,
    pub db_dir: String,
    pub backups_dir: String,
}

/// Per-run display overrides for editable readings. Display-only: they replace
/// the shown number but do **not** recompute the curves / DIN factor. Defaulted
/// for records written before this field existed.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueOverrides {
    pub temp_c: Option<i16>,
    pub pressure_hpa: Option<i16>,
}

/// A full run record persisted to the library (`db/runs/<yyyy>/<date>/<id>.json`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunRecord {
    pub schema_version: u32,
    pub id: String,
    pub sha256: String,
    pub source_image: Option<String>,
    pub source_entry: Option<String>,
    pub was_deleted_entry: bool,
    /// Shop/owner name from the source disk's FLA.CFG (kept so the printout shows
    /// it even when viewing a library run with no disk open). Defaulted for
    /// records written before this field existed.
    #[serde(default)]
    pub shop_name: Option<String>,
    pub run_date: Option<String>,
    pub imported_at: String,
    pub description: String,
    /// User overrides for editable readings (display-only).
    #[serde(default)]
    pub value_overrides: ValueOverrides,
    pub results: ResultsDto,
    pub channels: ChannelsDto,
}

/// A lightweight index row for fast library browsing (no channel arrays).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunIndexEntry {
    pub id: String,
    pub sha256: String,
    pub run_date: Option<String>,
    pub source_image: Option<String>,
    pub pnim_kw: Option<i16>,
    pub description: String,
    pub imported_at: String,
    /// Path to the full record, relative to the `db/` directory.
    pub path: String,
}

/// Result of an import operation. Values are the run display names.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportReport {
    pub added: Vec<String>,
    pub skipped: Vec<String>,
    pub overwritten: Vec<String>,
    pub failed: Vec<String>,
}

impl From<&Results> for ResultsDto {
    fn from(r: &Results) -> Self {
        ResultsDto {
            pnim_kw: r.pnim_kw,
            pressure_hpa: r.pressure_hpa,
            temp_c: r.temp_c,
            k_din: r.k_din,
            pmax_kw: r.pmax_kw,
            rpm_at_pmax: r.rpm_at_pmax,
            ppyora_kw: r.ppyora_kw,
            phavio_kw: r.phavio_kw,
            mmax_nm: r.mmax_nm,
            rpm_at_mmax: r.rpm_at_mmax,
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
