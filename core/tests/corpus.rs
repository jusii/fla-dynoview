//! Integration tests against the real Bosch corpus living in the parent
//! `Bosch-FLA-203/` tree. These are **gated**: if the corpus isn't present
//! (e.g. in CI, where the copyrighted disk images are not committed) they print
//! a skip notice and pass, so the suite stays green everywhere.

use fladyno_core::{decode_erg, parse_cfg, Fat12};
use std::path::PathBuf;

/// Resolve `rel` against the parent project tree (two levels up from this crate).
fn corpus(rel: &str) -> Option<PathBuf> {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..").join(rel);
    p.exists().then_some(p)
}

#[test]
fn dska0000_live_run_and_shop_name() {
    let Some(img) = corpus("Data/DSKA0000.img") else {
        eprintln!("SKIP dska0000_live_run_and_shop_name: corpus absent");
        return;
    };
    let data = std::fs::read(&img).expect("read image");
    let fs = Fat12::new(data).expect("valid 1.44MB image");

    // Shop name from FLA.CFG.
    let cfg_ent = fs.find("FLA/FLA.CFG").expect("FLA.CFG present");
    let cfg = parse_cfg(&fs.read_entry(&cfg_ent));
    assert_eq!(cfg.name, "Pellinen Motorsport");

    // The single live run DAT/1.ERG.
    let ent = fs.find("DAT/1.ERG").expect("DAT/1.ERG present");
    assert_eq!(ent.size, 8704, "full run is 8704 bytes");

    let run = decode_erg(&fs.read_entry(&ent)).expect("decode 1.ERG");
    assert_eq!(run.num_channels, 4);

    let d = run.date.expect("run carries a date");
    assert_eq!((d.year, d.month, d.day), (2021, 7, 1), "trailer date");

    // The three trailer offsets confirmed against the on-screen info box.
    assert_eq!(run.results.pnim_kw, Some(200), "Pnim");
    assert_eq!(run.results.pressure_hpa, Some(975), "Paine");
    assert_eq!(run.results.temp_c, Some(21), "Lämp");

    // The trailer date should agree with the FAT directory timestamp.
    if let Some(ts) = ent.mtime {
        assert_eq!((ts.year, ts.month, ts.day), (d.year, d.month, d.day));
    }
}

#[test]
fn dska0000_has_deleted_ergs() {
    let Some(img) = corpus("Data/DSKA0000.img") else {
        eprintln!("SKIP dska0000_has_deleted_ergs: corpus absent");
        return;
    };
    let fs = Fat12::new(std::fs::read(&img).unwrap()).unwrap();
    let ergs = fs.list_dat_ergs();
    let live = ergs.iter().filter(|e| !e.deleted).count();
    let deleted = ergs.iter().filter(|e| e.deleted).count();
    println!("DAT/*.ERG: {live} live, {deleted} deleted");
    assert!(live >= 1, "at least the one live run");
    assert!(deleted >= 1, "deleted runs are enumerated for carving");
}

#[test]
fn mystery_corpus_decodes_cleanly() {
    let Some(dir) = corpus("04-analysis/dyno-results-mystery") else {
        eprintln!("SKIP mystery_corpus_decodes_cleanly: corpus absent");
        return;
    };
    let mut full = 0usize;
    let mut dated = 0usize;
    let mut total = 0usize;
    for entry in std::fs::read_dir(&dir).unwrap() {
        let p = entry.unwrap().path();
        let is_erg = p
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.eq_ignore_ascii_case("erg"))
            .unwrap_or(false);
        if !is_erg {
            continue;
        }
        let bytes = std::fs::read(&p).unwrap();
        if bytes.len() < 512 {
            continue; // empty/aborted saves
        }
        total += 1;
        let run = decode_erg(&bytes).unwrap();
        if run.size == 8704 {
            full += 1;
        }
        if let Some(d) = run.date {
            if (2000..=2025).contains(&d.year) {
                dated += 1;
            }
        }
    }
    println!("mystery corpus: total={total} full={full} plausibly-dated={dated}");
    assert!(full > 40, "most runs are full 8704-byte saves");
    assert!(dated > 40, "most runs decode a plausible year");
}
