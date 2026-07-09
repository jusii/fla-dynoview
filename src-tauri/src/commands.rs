//! Tauri command handlers.

use crate::error::CommandError;
use crate::model::{
    fmt_date, sha256_hex, DecodedRunDto, ImageSummary, Paths, RunEntry, ShopInfo,
};
use fladyno_core::{decode_erg, parse_cfg, Fat12};
use tauri::Manager;

fn read_bytes(path: &str) -> Result<Vec<u8>, CommandError> {
    std::fs::read(path).map_err(|e| CommandError::Io(format!("{path}: {e}")))
}

fn open_fat12(path: &str) -> Result<Fat12, CommandError> {
    Ok(Fat12::new(read_bytes(path)?)?)
}

fn decode_dto(bytes: &[u8]) -> Result<DecodedRunDto, CommandError> {
    let run = decode_erg(bytes)?;
    Ok(DecodedRunDto::from_core(&run, sha256_hex(bytes)))
}

/// Open a floppy image: return the shop name and the list of `\DAT\*.ERG` runs.
#[tauri::command]
pub async fn open_image(path: String) -> Result<ImageSummary, CommandError> {
    let fs = open_fat12(&path)?;
    let shop_name = fs
        .find("FLA/FLA.CFG")
        .map(|e| parse_cfg(&fs.read_entry(&e)).name)
        .unwrap_or_default();

    let mut runs = Vec::new();
    for e in fs.list_dat_ergs() {
        let bytes = fs.read_entry(&e);
        let date = decode_erg(&bytes)
            .ok()
            .and_then(|r| r.date)
            .map(|d| fmt_date(&d));
        runs.push(RunEntry {
            name: e.name.clone(),
            path: e.path.clone(),
            size: e.size,
            deleted: e.deleted,
            sha256: sha256_hex(&bytes),
            date,
        });
    }
    Ok(ImageSummary {
        image_path: path,
        shop_name,
        runs,
    })
}

/// Read and decode one `.ERG` entry (live or deleted) out of an image.
#[tauri::command]
pub async fn read_erg_from_image(
    path: String,
    entry_path: String,
) -> Result<DecodedRunDto, CommandError> {
    let fs = open_fat12(&path)?;
    let ent = fs
        .list_dat_ergs()
        .into_iter()
        .find(|e| e.path == entry_path)
        .ok_or_else(|| CommandError::Other(format!("run not found on image: {entry_path}")))?;
    decode_dto(&fs.read_entry(&ent))
}

/// Open and decode a bare `.ERG` file.
#[tauri::command]
pub async fn open_erg_file(path: String) -> Result<DecodedRunDto, CommandError> {
    decode_dto(&read_bytes(&path)?)
}

/// Decode a `.ERG` payload already in memory (pure, no I/O).
#[tauri::command]
pub fn parse_erg(bytes: Vec<u8>) -> Result<DecodedRunDto, CommandError> {
    decode_dto(&bytes)
}

/// Parse just the shop/owner header from an image's `FLA.CFG`.
#[tauri::command]
pub async fn get_shop_header(path: String) -> Result<ShopInfo, CommandError> {
    let fs = open_fat12(&path)?;
    let name = fs
        .find("FLA/FLA.CFG")
        .map(|e| parse_cfg(&fs.read_entry(&e)).name)
        .unwrap_or_default();
    Ok(ShopInfo { name })
}

/// A file path passed on the command line (for "open with" / CLI use), if it
/// names an existing file. The frontend calls this on startup to auto-open it.
#[tauri::command]
pub fn initial_path() -> Option<String> {
    std::env::args()
        .nth(1)
        .filter(|p| std::path::Path::new(p).is_file())
}

/// Resolved database/backup locations under the OS app-data dir.
#[tauri::command]
pub fn app_paths(app: tauri::AppHandle) -> Result<Paths, CommandError> {
    let base = app
        .path()
        .app_data_dir()
        .map_err(|e| CommandError::Other(e.to_string()))?;
    let db = base.join("db");
    Ok(Paths {
        db_dir: db.to_string_lossy().into_owned(),
        backups_dir: db.join("backups").to_string_lossy().into_owned(),
    })
}
