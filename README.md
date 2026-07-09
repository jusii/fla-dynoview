# FLA Dynoview

A cross-platform desktop app (Windows, macOS, Linux) for the **Bosch FLA 203**
chassis dyno. The original machine runs 1990s DOS software that can only print
results to an obsolete dot-matrix printer; FLA Dynoview reads the machine's data
floppy on a modern PC so you can **view, archive, and print/PDF** your dyno runs.

> Not affiliated with or endorsed by Robert Bosch GmbH. "Bosch" and "FLA" are
> used only to describe hardware compatibility.

## Install & usage guides

The app is localized into every language the FLA 203 supports. Install & usage:
[Deutsch](docs/de/install-and-usage.md) ·
[English](docs/en/install-and-usage.md) ·
[Español](docs/es/install-and-usage.md) ·
[Français](docs/fr/install-and-usage.md) ·
[Italiano](docs/it/install-and-usage.md) ·
[Nederlands](docs/nl/install-and-usage.md) ·
[Português](docs/pt/install-and-usage.md) ·
[Suomi](docs/fi/install-and-usage.md) ·
[Dansk](docs/da/install-and-usage.md) ·
[日本語](docs/ja/install-and-usage.md)

## Features

- **Open a floppy image** (`.img`, e.g. captured from a Gotek) or a single
  **`.ERG`** run file.
- **Two dyno charts** reproducing the machine's own views — power (kW vs km/h)
  and torque (Nm vs rpm) — rendered as crisp vector SVG.
- **Local run library** — imported runs are saved as human-readable JSON,
  organised by date, each with a free-text description so you can find a
  customer or a specific pull later.
- **De-duplicated import** — a SHA-256 of each run means re-importing a disk only
  adds new runs by default; an explicit "import all / overwrite" is available.
- **Print & PDF** — print to any printer, or export a clean one-page report.
- **Reset a disk** — wipe only the dyno runs (`\DAT\*.ERG`) from an image while
  keeping the machine's settings and calibration (`FLA.CFG`), always with a
  backup.

## Install

Pre-built binaries are attached to each [GitHub Release](../../releases).

These builds are **unsigned**, so the OS will warn on first launch:

- **macOS** — right-click the app → **Open** (once), or run
  `xattr -cr "/Applications/FLA Dynoview.app"`.
- **Windows** — SmartScreen: **More info → Run anyway**.

## Build from source

Requires [Rust](https://rustup.rs), [Node.js](https://nodejs.org) 20+, and the
Tauri prerequisites for your OS. On Debian/Ubuntu:

```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev \
  libjavascriptcoregtk-4.1-dev librsvg2-dev patchelf build-essential

npm install
npm run tauri dev      # run in development
npm run tauri build    # produce a release bundle
```

## Project layout

```
core/        Rust parser library (FAT12 image + .ERG + FLA.CFG) — pure std, unit-tested
src-tauri/   Tauri v2 backend (commands, run database, PDF, reset)
src/         Svelte 5 frontend (run browser, SVG charts, report view)
```

The `core/` crate has no external dependencies and its tests run offline
(`cargo test -p fladyno-core`); it also validates against the real disk corpus
when present.

## Data format

The `.ERG` ("Ergebnis") layout, the FAT12 disk structure, and the trailer
scalar offsets were reverse-engineered from real disks and cross-checked against
the machine's on-screen values. See `core/src/erg.rs` for the documented layout.

## License

[MIT](LICENSE) © Jussi Alanärä
