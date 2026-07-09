# FLA Dynoview — Install & Usage (English)

Read and archive Bosch FLA 203 dyno runs on a modern PC, and print or export a PDF.

## Install

### Option A — download a ready-made build
Grab the file for your system from the [Releases page](https://github.com/jusii/fla-dynoview/releases):

- **Windows** — `FLA.Dynoview_x64-setup.exe` (installer)
- **macOS** — `FLA.Dynoview_aarch64.dmg` (Apple Silicon) or `FLA.Dynoview_x64.dmg` (Intel)
- **Linux** — `FLA.Dynoview_amd64.AppImage` (portable) or `..._amd64.deb`

The builds are **unsigned**, so the OS warns on first launch:

- **macOS:** right-click the app → **Open** (once), or run `xattr -cr "/Applications/FLA Dynoview.app"`.
- **Windows:** SmartScreen → **More info → Run anyway**.
- **Linux (AppImage):** `chmod +x FLA.Dynoview_*.AppImage` then run it.

### Option B — build from source
Requires Rust, Node.js 20+, and the Tauri prerequisites. On Debian/Ubuntu:

```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev \
  libjavascriptcoregtk-4.1-dev librsvg2-dev patchelf build-essential
npm install
npm run tauri dev      # run it
npm run tauri build    # build installers
```

## Usage

1. **Open a disk or a run.** Click **Open .img…** to open a Bosch floppy image, or
   **Open .ERG…** for a single run file. You can also start it from the command line
   with a file path.
2. **Browse runs.** The **Disk** tab lists every run on the image; deleted runs are
   shown struck-through. Click a run to see its **power** and **torque** charts
   (kW / Nm) and its values (Pmax, Ppyörä, Phäviö, Mmax, k, …).
3. **Import to your library.** **Import new** adds runs not yet saved; **Import all…**
   re-imports everything and overwrites existing copies (with a warning). Imported runs
   are de-duplicated by content, so re-importing a disk only adds what's new.
4. **Find runs later.** The **Library** tab lists your saved runs by date. Add a
   **description** (customer, vehicle, notes) and search it.
5. **Print or export PDF.** Click **Print / PDF…** and choose your printer, or pick
   "Save as PDF" / "Print to File" in the print dialog.
6. **Reset a disk.** **Reset disk…** deletes only the dyno runs from the image while
   keeping the machine's settings and calibration (`FLA.CFG`), languages and fonts. A
   timestamped backup of the image is saved first.
7. **Settings (⚙).** Choose the **language** and the **unit system** (metric kW/Nm/°C
   or imperial bhp/lb·ft/°F). Your choice is remembered.

## Where your data is stored

Imported runs and settings live in a `fla-dynoview` folder:

- **Windows:** `%APPDATA%\fla-dynoview\`
- **macOS:** `~/Library/Application Support/fla-dynoview/`
- **Linux:** `~/.local/share/fla-dynoview/`

It contains `settings.json`, the `db/` run library (human-readable JSON, organised by
date, with the original `.ERG` alongside each run), and `db/backups/` disk backups.

> The machine's own language is a runtime setting that isn't saved on the disk, so the
> app can't detect it automatically — pick your language in Settings.
