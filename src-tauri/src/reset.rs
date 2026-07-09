//! Disk-reset: wipe only the dyno runs (`\DAT\*.ERG`) from a floppy image while
//! keeping settings/calibration (`FLA.CFG`), language tables and fonts intact.
//!
//! Uses the `fatfs` crate for the clean write path (our own reader is read-only).
//! Safety measures, in order:
//!   1. copy the original to a timestamped backup;
//!   2. do the deletions on a working copy, not the original;
//!   3. re-open the result with our reader and assert `FLA.CFG` survived and no
//!      live `.ERG` remains — otherwise abort and leave the original untouched;
//!   4. only then atomically replace the original.

use crate::error::CommandError;
use crate::model::ResetReport;
use fladyno_core::Fat12;
use std::fs;
use std::path::Path;

fn io<E: std::fmt::Display>(ctx: &str, e: E) -> CommandError {
    CommandError::Io(format!("{ctx}: {e}"))
}

/// Reset `image_path`, writing a backup into `backups_dir` tagged with `timestamp`.
pub fn reset_image(
    image_path: &str,
    backups_dir: &Path,
    timestamp: &str,
) -> Result<ResetReport, CommandError> {
    let orig = Path::new(image_path);
    let stem = orig
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "image.img".into());

    // 1. backup
    fs::create_dir_all(backups_dir).map_err(|e| io("create backups dir", e))?;
    let backup = backups_dir.join(format!("{stem}.{timestamp}.bak.img"));
    fs::copy(orig, &backup).map_err(|e| io("backup image", e))?;

    // 2. working copy
    let tmp = orig.with_file_name(format!("{stem}.reset.tmp"));
    fs::copy(orig, &tmp).map_err(|e| io("make working copy", e))?;

    // 3. delete the runs on the copy
    let deleted = match delete_dat_ergs(&tmp) {
        Ok(d) => d,
        Err(e) => {
            let _ = fs::remove_file(&tmp);
            return Err(e);
        }
    };

    // 4. sanity-check the result before touching the original
    let data = fs::read(&tmp).map_err(|e| io("read working copy", e))?;
    let check = Fat12::new(data).map_err(|e| CommandError::Core(e.to_string()))?;
    if check.find("FLA/FLA.CFG").is_none() {
        let _ = fs::remove_file(&tmp);
        return Err(CommandError::Other(
            "reset aborted: FLA.CFG missing after wipe; original left unchanged (backup kept)".into(),
        ));
    }
    if check.list_dat_ergs().iter().any(|e| !e.deleted) {
        let _ = fs::remove_file(&tmp);
        return Err(CommandError::Other(
            "reset aborted: a live .ERG remained after wipe; original left unchanged".into(),
        ));
    }

    // 5. replace the original
    fs::rename(&tmp, orig).map_err(|e| io("replace original", e))?;

    Ok(ResetReport {
        backup_path: backup.to_string_lossy().into_owned(),
        deleted,
    })
}

/// Remove every live `*.ERG` from `\DAT` in the image at `path` (in place).
fn delete_dat_ergs(path: &Path) -> Result<Vec<String>, CommandError> {
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .map_err(|e| io("open image read/write", e))?;
    let stream = fscommon::BufStream::new(file);
    let fs_ = fatfs::FileSystem::new(stream, fatfs::FsOptions::new())
        .map_err(|e| io("mount FAT12", e))?;

    let deleted = {
        let root = fs_.root_dir();
        match root.open_dir("DAT") {
            Ok(dat) => {
                let names: Vec<String> = dat
                    .iter()
                    .filter_map(|r| r.ok())
                    .filter(|e| !e.is_dir())
                    .map(|e| e.file_name())
                    .filter(|n| n.to_uppercase().ends_with(".ERG"))
                    .collect();
                let mut removed = Vec::new();
                for n in &names {
                    dat.remove(n).map_err(|e| io(&format!("remove {n}"), e))?;
                    removed.push(n.clone());
                }
                removed
            }
            Err(_) => Vec::new(), // no \DAT directory: nothing to do
        }
    };

    fs_.unmount().map_err(|e| io("flush/unmount FAT12", e))?;
    Ok(deleted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn corpus() -> Option<PathBuf> {
        let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../Data/DSKA0000.img");
        p.exists().then_some(p)
    }

    #[test]
    fn reset_keeps_settings_wipes_runs() {
        let Some(src) = corpus() else {
            eprintln!("SKIP reset_keeps_settings_wipes_runs: corpus absent");
            return;
        };
        let tmpdir = std::env::temp_dir().join(format!("fladyno-reset-{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmpdir);
        fs::create_dir_all(&tmpdir).unwrap();
        let work = tmpdir.join("DSKA0000.img");
        fs::copy(&src, &work).unwrap();

        // Baseline: capture FLA.CFG bytes and confirm there is a live run.
        let before = Fat12::new(fs::read(&work).unwrap()).unwrap();
        let cfg_before = before.read_entry(&before.find("FLA/FLA.CFG").unwrap());
        assert!(before.list_dat_ergs().iter().filter(|e| !e.deleted).count() >= 1);

        let report =
            reset_image(work.to_str().unwrap(), &tmpdir.join("backups"), "TESTSTAMP").unwrap();
        assert!(!report.deleted.is_empty(), "removed at least one run");
        assert!(Path::new(&report.backup_path).exists(), "backup written");

        // After: FLA.CFG byte-identical, runs gone, languages/fonts kept.
        let after = Fat12::new(fs::read(&work).unwrap()).unwrap();
        let cfg_after = after.read_entry(&after.find("FLA/FLA.CFG").expect("FLA.CFG kept"));
        assert_eq!(cfg_after, cfg_before, "calibration/shop name preserved verbatim");
        assert_eq!(
            after.list_dat_ergs().iter().filter(|e| !e.deleted).count(),
            0,
            "all live runs wiped"
        );
        assert!(after.find("TXT/FINNISCH.TXT").is_some(), "language tables kept");

        let _ = fs::remove_dir_all(&tmpdir);
    }
}
