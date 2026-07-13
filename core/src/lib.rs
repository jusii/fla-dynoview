//! # fladyno-core
//!
//! Parser core for the Bosch FLA 203 chassis dyno's data floppies.
//!
//! Three read-only building blocks, all pure-`std` (no external crates) so the
//! parser can be unit-tested offline and later wired into the Tauri backend:
//!
//! - [`fat12`] — reads files out of a raw 1.44 MB FAT12 floppy image
//!   (`\DAT\*.ERG`, `\FLA\FLA.CFG`, …), including deleted directory entries.
//!   A faithful Rust port of the project's `tools/fat12.py`.
//! - [`erg`] — decodes a `.ERG` ("Ergebnis"/result) run file into its four
//!   measurement-curve channels plus the trailer scalar results and run date.
//! - [`config`] — extracts the shop/owner name from `FLA.CFG`.
//!
//! The write path (deleting `\DAT\*.ERG` for the disk-reset feature) is handled
//! separately in the Tauri crate via the `fatfs` crate; this crate stays
//! read-only and dependency-free.

pub mod cp437;
pub mod fat12;
pub mod erg;
pub mod config;

pub use config::{parse_cfg, ShopInfo};
pub use erg::{
    decode as decode_erg, detect_format, Channels, DecodedRun, ErgFormat, Results, RunDate,
};
pub use fat12::{DirEntry, Fat12, FatDateTime, IMG_SIZE};

/// Errors produced by the parser core.
///
/// Deliberately dependency-free (no `thiserror`) so the core builds and tests
/// offline. The Tauri layer maps these into its serializable command error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreError {
    /// The image is not exactly [`IMG_SIZE`] bytes (a standard 1.44 MB floppy).
    BadImageSize(usize),
    /// A `.ERG` payload is too small to contain even the 512-byte trailer.
    BadErgSize(usize),
    /// A requested path was not found on the filesystem.
    NotFound(String),
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreError::BadImageSize(n) => {
                write!(f, "unexpected image size {n} bytes (expected {IMG_SIZE})")
            }
            CoreError::BadErgSize(n) => write!(f, "ERG file too small: {n} bytes (need >= 512)"),
            CoreError::NotFound(p) => write!(f, "not found on image: {p}"),
        }
    }
}

impl std::error::Error for CoreError {}
