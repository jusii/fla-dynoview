//! Tauri command handlers.

use crate::db;
use crate::error::CommandError;
use crate::model::{
    fmt_date, sha256_hex, DecodedRunDto, ImageSummary, ImportReport, Paths, ResetReport, RunEntry,
    RunIndexEntry, RunRecord, ShopInfo,
};
use fladyno_core::{decode_erg, parse_cfg, DirEntry, Fat12};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
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

fn file_name(path: &str) -> String {
    Path::new(path)
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string())
}

/// The app's base folder: `<OS data dir>/fla-dynoview` — a plain name rather
/// than the reverse-DNS bundle id. Holds `db/` and `settings.json`.
pub fn app_root(app: &tauri::AppHandle) -> Result<PathBuf, CommandError> {
    let base = app
        .path()
        .data_dir()
        .map_err(|e| CommandError::Other(e.to_string()))?;
    Ok(base.join("fla-dynoview"))
}

fn db_dir(app: &tauri::AppHandle) -> Result<PathBuf, CommandError> {
    Ok(app_root(app)?.join("db"))
}

/// Build a persistable record from raw `.ERG` bytes.
fn build_record(
    bytes: &[u8],
    source_image: Option<String>,
    source_entry: Option<String>,
    was_deleted: bool,
) -> Result<RunRecord, CommandError> {
    let dto = decode_dto(bytes)?;
    Ok(RunRecord {
        schema_version: 1,
        id: dto.sha256.clone(),
        sha256: dto.sha256.clone(),
        source_image,
        source_entry,
        was_deleted_entry: was_deleted,
        run_date: dto.date.clone(),
        imported_at: chrono::Utc::now().to_rfc3339(),
        description: String::new(),
        results: dto.results,
        channels: dto.channels,
    })
}

// ---------------------------------------------------------------------------
// Reading images / runs
// ---------------------------------------------------------------------------

/// Open a floppy image: shop name + the list of `\DAT\*.ERG` runs, each flagged
/// with whether it is already in the local library.
#[tauri::command]
pub async fn open_image(
    app: tauri::AppHandle,
    path: String,
) -> Result<ImageSummary, CommandError> {
    let fs = open_fat12(&path)?;
    let shop_name = fs
        .find("FLA/FLA.CFG")
        .map(|e| parse_cfg(&fs.read_entry(&e)).name)
        .unwrap_or_default();

    let known: HashSet<String> = db::load_index(&db_dir(&app)?)
        .unwrap_or_default()
        .into_iter()
        .map(|e| e.sha256)
        .collect();

    let mut runs = Vec::new();
    for e in fs.list_dat_ergs() {
        let bytes = fs.read_entry(&e);
        let date = decode_erg(&bytes).ok().and_then(|r| r.date).map(|d| fmt_date(&d));
        let sha = sha256_hex(&bytes);
        runs.push(RunEntry {
            name: e.name.clone(),
            path: e.path.clone(),
            size: e.size,
            deleted: e.deleted,
            in_library: known.contains(&sha),
            sha256: sha,
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

// ---------------------------------------------------------------------------
// Library / import
// ---------------------------------------------------------------------------

fn do_import(
    db: &Path,
    fs: &Fat12,
    img_name: &str,
    entries: &[DirEntry],
    overwrite: bool,
) -> ImportReport {
    let mut report = ImportReport::default();
    for ent in entries {
        let bytes = fs.read_entry(ent);
        let label = ent.name.clone();
        match build_record(&bytes, Some(img_name.to_string()), Some(ent.name.clone()), ent.deleted)
            .and_then(|rec| db::import_one(db, &bytes, &rec, overwrite))
        {
            Ok(db::Outcome::Added) => report.added.push(label),
            Ok(db::Outcome::Skipped) => report.skipped.push(label),
            Ok(db::Outcome::Overwritten) => report.overwritten.push(label),
            Err(_) => report.failed.push(label),
        }
    }
    report
}

/// Import selected runs from an image into the library.
#[tauri::command]
pub async fn import_runs(
    app: tauri::AppHandle,
    image_path: String,
    entry_paths: Vec<String>,
    overwrite: bool,
) -> Result<ImportReport, CommandError> {
    let db = db_dir(&app)?;
    let fs = open_fat12(&image_path)?;
    let img_name = file_name(&image_path);
    let wanted: HashSet<String> = entry_paths.into_iter().collect();
    let entries: Vec<DirEntry> = fs
        .list_dat_ergs()
        .into_iter()
        .filter(|e| wanted.contains(&e.path))
        .collect();
    Ok(do_import(&db, &fs, &img_name, &entries, overwrite))
}

/// Import every run from an image (optionally including deleted/carved ones).
#[tauri::command]
pub async fn import_all(
    app: tauri::AppHandle,
    image_path: String,
    overwrite: bool,
    include_deleted: bool,
) -> Result<ImportReport, CommandError> {
    let db = db_dir(&app)?;
    let fs = open_fat12(&image_path)?;
    let img_name = file_name(&image_path);
    let entries: Vec<DirEntry> = fs
        .list_dat_ergs()
        .into_iter()
        .filter(|e| include_deleted || !e.deleted)
        .collect();
    Ok(do_import(&db, &fs, &img_name, &entries, overwrite))
}

/// List library runs (newest first), optionally filtered by a search string.
#[tauri::command]
pub async fn list_db_runs(
    app: tauri::AppHandle,
    query: Option<String>,
) -> Result<Vec<RunIndexEntry>, CommandError> {
    let mut idx = db::load_index(&db_dir(&app)?)?;
    if let Some(q) = query.map(|q| q.trim().to_lowercase()).filter(|q| !q.is_empty()) {
        idx.retain(|e| {
            e.description.to_lowercase().contains(&q)
                || e.run_date.as_deref().unwrap_or("").contains(&q)
                || e.source_image.as_deref().unwrap_or("").to_lowercase().contains(&q)
        });
    }
    idx.sort_by(|a, b| {
        b.run_date
            .cmp(&a.run_date)
            .then(b.imported_at.cmp(&a.imported_at))
    });
    Ok(idx)
}

/// Fetch one full library record (with channel arrays) for charting.
#[tauri::command]
pub async fn get_db_run(app: tauri::AppHandle, id: String) -> Result<RunRecord, CommandError> {
    db::get_record(&db_dir(&app)?, &id)
}

/// Set a run's free-text description.
#[tauri::command]
pub async fn update_run_description(
    app: tauri::AppHandle,
    id: String,
    description: String,
) -> Result<(), CommandError> {
    db::update_description(&db_dir(&app)?, &id, &description)
}

/// Remove a run from the library.
#[tauri::command]
pub async fn delete_db_run(app: tauri::AppHandle, id: String) -> Result<(), CommandError> {
    db::delete_record(&db_dir(&app)?, &id)
}

// ---------------------------------------------------------------------------
// Reset
// ---------------------------------------------------------------------------

/// Wipe only the dyno runs (`\DAT\*.ERG`) from an image, keeping settings. A
/// timestamped backup is written first; refuses unless `confirm` is `true`.
#[tauri::command]
pub async fn reset_image(
    app: tauri::AppHandle,
    image_path: String,
    confirm: bool,
) -> Result<ResetReport, CommandError> {
    if !confirm {
        return Err(CommandError::Other("reset requires confirmation".into()));
    }
    let backups = db_dir(&app)?.join("backups").join("images");
    let ts = chrono::Local::now().format("%Y%m%dT%H%M%S").to_string();
    crate::reset::reset_image(&image_path, &backups, &ts)
}

// ---------------------------------------------------------------------------
// Misc
// ---------------------------------------------------------------------------

/// A file path passed on the command line (for "open with" / CLI use), if it
/// names an existing file. The frontend calls this on startup to auto-open it.
#[tauri::command]
pub fn initial_path() -> Option<String> {
    std::env::args()
        .nth(1)
        .filter(|p| Path::new(p).is_file())
}

/// Resolved data-folder locations.
#[tauri::command]
pub fn app_paths(app: tauri::AppHandle) -> Result<Paths, CommandError> {
    let root = app_root(&app)?;
    let db = root.join("db");
    Ok(Paths {
        root: root.to_string_lossy().into_owned(),
        db_dir: db.to_string_lossy().into_owned(),
        backups_dir: db.join("backups").to_string_lossy().into_owned(),
    })
}

/// Load persisted settings (language, unit system).
#[tauri::command]
pub fn get_settings(app: tauri::AppHandle) -> Result<crate::settings::Settings, CommandError> {
    Ok(crate::settings::load(&app_root(&app)?))
}

/// Persist settings.
#[tauri::command]
pub fn set_settings(
    app: tauri::AppHandle,
    settings: crate::settings::Settings,
) -> Result<(), CommandError> {
    crate::settings::save(&app_root(&app)?, &settings)
}
