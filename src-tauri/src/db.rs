//! The local run library: human-readable JSON on disk under the app-data dir.
//!
//! ```text
//! db/
//!   index.json                          lightweight list for fast browsing
//!   runs/<yyyy>/<yyyy-mm-dd>/<id>.json   full RunRecord (id = sha256)
//!   runs/<yyyy>/<yyyy-mm-dd>/<id>.erg    raw 8704-byte original (provenance)
//!   backups/images/…                     reset-image backups
//! ```
//!
//! De-duplication is by the run's SHA-256 (also its `id`): re-importing a disk
//! adds only new runs unless `overwrite` is set.

use crate::error::CommandError;
use crate::model::{RunIndexEntry, RunRecord};
use std::fs;
use std::path::{Path, PathBuf};

fn io<E: std::fmt::Display>(ctx: &str, e: E) -> CommandError {
    CommandError::Io(format!("{ctx}: {e}"))
}

fn json_err<E: std::fmt::Display>(e: E) -> CommandError {
    CommandError::Other(format!("json: {e}"))
}

fn index_path(db: &Path) -> PathBuf {
    db.join("index.json")
}

/// Load the library index (empty if it doesn't exist yet).
pub fn load_index(db: &Path) -> Result<Vec<RunIndexEntry>, CommandError> {
    let p = index_path(db);
    if !p.exists() {
        return Ok(Vec::new());
    }
    let s = fs::read_to_string(&p).map_err(|e| io("read index", e))?;
    serde_json::from_str(&s).map_err(json_err)
}

fn save_index(db: &Path, idx: &[RunIndexEntry]) -> Result<(), CommandError> {
    fs::create_dir_all(db).map_err(|e| io("create db dir", e))?;
    let s = serde_json::to_string_pretty(idx).map_err(json_err)?;
    fs::write(index_path(db), s).map_err(|e| io("write index", e))
}

/// `runs/<year>/<date>/<id>.json`, relative to `db/`.
fn record_rel(rec: &RunRecord) -> String {
    let (year, day) = match rec.run_date.as_deref() {
        Some(d) if d.len() >= 4 => (&d[..4], d),
        _ => ("unknown", "unknown"),
    };
    format!("runs/{year}/{day}/{}.json", rec.id)
}

fn index_row(rec: &RunRecord, rel: String) -> RunIndexEntry {
    RunIndexEntry {
        id: rec.id.clone(),
        sha256: rec.sha256.clone(),
        run_date: rec.run_date.clone(),
        source_image: rec.source_image.clone(),
        pnim_kw: rec.results.pnim_kw,
        description: rec.description.clone(),
        imported_at: rec.imported_at.clone(),
        path: rel,
    }
}

fn write_record(db: &Path, rec: &RunRecord, erg: &[u8]) -> Result<String, CommandError> {
    let rel = record_rel(rec);
    let full = db.join(&rel);
    if let Some(parent) = full.parent() {
        fs::create_dir_all(parent).map_err(|e| io("create run dir", e))?;
    }
    let json = serde_json::to_string_pretty(rec).map_err(json_err)?;
    fs::write(&full, json).map_err(|e| io("write record", e))?;
    fs::write(full.with_extension("erg"), erg).map_err(|e| io("write erg sidecar", e))?;
    Ok(rel)
}

/// Outcome of importing a single run.
pub enum Outcome {
    Added,
    Skipped,
    Overwritten,
}

/// Import one run. `erg` is the raw `.ERG` bytes; `rec` the decoded record
/// (whose `id`/`sha256` must equal the digest of `erg`).
pub fn import_one(
    db: &Path,
    erg: &[u8],
    rec: &RunRecord,
    overwrite: bool,
) -> Result<Outcome, CommandError> {
    let mut idx = load_index(db)?;
    if let Some(pos) = idx.iter().position(|e| e.id == rec.id) {
        if !overwrite {
            return Ok(Outcome::Skipped);
        }
        let rel = write_record(db, rec, erg)?;
        idx[pos] = index_row(rec, rel);
        save_index(db, &idx)?;
        Ok(Outcome::Overwritten)
    } else {
        let rel = write_record(db, rec, erg)?;
        idx.push(index_row(rec, rel));
        save_index(db, &idx)?;
        Ok(Outcome::Added)
    }
}

/// Read one full record by id.
pub fn get_record(db: &Path, id: &str) -> Result<RunRecord, CommandError> {
    let idx = load_index(db)?;
    let entry = idx
        .iter()
        .find(|e| e.id == id)
        .ok_or_else(|| CommandError::Other(format!("run not in library: {id}")))?;
    let s = fs::read_to_string(db.join(&entry.path)).map_err(|e| io("read record", e))?;
    serde_json::from_str(&s).map_err(json_err)
}

/// Update a run's description (record + index row).
pub fn update_description(db: &Path, id: &str, description: &str) -> Result<(), CommandError> {
    let mut idx = load_index(db)?;
    let pos = idx
        .iter()
        .position(|e| e.id == id)
        .ok_or_else(|| CommandError::Other(format!("run not in library: {id}")))?;
    let rel = idx[pos].path.clone();
    let full = db.join(&rel);
    let mut rec: RunRecord =
        serde_json::from_str(&fs::read_to_string(&full).map_err(|e| io("read record", e))?)
            .map_err(json_err)?;
    rec.description = description.to_string();
    let json = serde_json::to_string_pretty(&rec).map_err(json_err)?;
    fs::write(&full, json).map_err(|e| io("write record", e))?;
    idx[pos].description = description.to_string();
    save_index(db, &idx)
}

/// Delete a run (record + sidecar + index row).
pub fn delete_record(db: &Path, id: &str) -> Result<(), CommandError> {
    let mut idx = load_index(db)?;
    let pos = idx
        .iter()
        .position(|e| e.id == id)
        .ok_or_else(|| CommandError::Other(format!("run not in library: {id}")))?;
    let full = db.join(&idx[pos].path);
    let _ = fs::remove_file(&full);
    let _ = fs::remove_file(full.with_extension("erg"));
    idx.remove(pos);
    save_index(db, &idx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ChannelsDto, ResultsDto};

    fn rec(id: &str, date: &str, desc: &str) -> RunRecord {
        RunRecord {
            schema_version: 1,
            id: id.to_string(),
            sha256: id.to_string(),
            source_image: Some("DSKA0000.img".into()),
            source_entry: Some("1.ERG".into()),
            was_deleted_entry: false,
            run_date: Some(date.to_string()),
            imported_at: "2026-07-09T10:00:00Z".into(),
            description: desc.to_string(),
            results: ResultsDto {
                pnim_kw: Some(200),
                pressure_hpa: Some(975),
                temp_c: Some(21),
                rpm_raw: None,
                trailer_scan: vec![],
            },
            channels: ChannelsDto {
                ch0: vec![1, 2, 3],
                ch1: vec![],
                ch2: vec![],
                ch3: vec![],
            },
        }
    }

    #[test]
    fn import_dedup_and_overwrite() {
        let tmp = std::env::temp_dir().join(format!("fladyno-db-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        let db = tmp.join("db");

        let r = rec("aaa", "2021-07-01", "first");
        assert!(matches!(import_one(&db, b"ERGDATA", &r, false).unwrap(), Outcome::Added));
        // second identical import is skipped by default
        assert!(matches!(import_one(&db, b"ERGDATA", &r, false).unwrap(), Outcome::Skipped));
        assert_eq!(load_index(&db).unwrap().len(), 1);

        // overwrite updates in place
        let r2 = rec("aaa", "2021-07-01", "updated");
        assert!(matches!(import_one(&db, b"ERGDATA", &r2, true).unwrap(), Outcome::Overwritten));
        assert_eq!(load_index(&db).unwrap().len(), 1);
        assert_eq!(get_record(&db, "aaa").unwrap().description, "updated");

        // a different run is added alongside
        let r3 = rec("bbb", "2019-05-02", "other");
        assert!(matches!(import_one(&db, b"ZZ", &r3, false).unwrap(), Outcome::Added));
        assert_eq!(load_index(&db).unwrap().len(), 2);

        // description edit persists to record + index
        update_description(&db, "bbb", "renamed").unwrap();
        assert_eq!(get_record(&db, "bbb").unwrap().description, "renamed");
        assert!(load_index(&db).unwrap().iter().any(|e| e.description == "renamed"));

        // date-organised path
        assert!(db.join("runs/2021/2021-07-01/aaa.json").exists());
        assert!(db.join("runs/2021/2021-07-01/aaa.erg").exists());

        // delete removes record + sidecar + index row
        delete_record(&db, "aaa").unwrap();
        assert_eq!(load_index(&db).unwrap().len(), 1);
        assert!(!db.join("runs/2021/2021-07-01/aaa.json").exists());

        let _ = fs::remove_dir_all(&tmp);
    }
}
