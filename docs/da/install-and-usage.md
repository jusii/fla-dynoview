# FLA Dynoview — Installation og brug (Dansk)

Læs og arkivér Bosch FLA 203-dynokørsler på en moderne pc, og udskriv eller eksportér en PDF.

## Installation

### Mulighed A — hent en færdig build
Hent filen til dit system fra [Releases-siden](https://github.com/jusii/fla-dynoview/releases):

- **Windows** — `FLA.Dynoview_x64-setup.exe` (installationsprogram)
- **macOS** — `FLA.Dynoview_aarch64.dmg` (Apple Silicon) eller `FLA.Dynoview_x64.dmg` (Intel)
- **Linux** — `FLA.Dynoview_amd64.AppImage` (bærbar) eller `..._amd64.deb`

Disse builds er **usignerede**, så operativsystemet advarer ved første start:

- **macOS:** højreklik på appen → **Åbn** (én gang), eller kør `xattr -cr "/Applications/FLA Dynoview.app"`.
- **Windows:** SmartScreen → **Flere oplysninger → Kør alligevel**.
- **Linux (AppImage):** `chmod +x FLA.Dynoview_*.AppImage`, og kør den derefter.

### Mulighed B — byg fra kildekode
Kræver Rust, Node.js 20+ og Tauri-forudsætningerne. På Debian/Ubuntu:

```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev \
  libjavascriptcoregtk-4.1-dev librsvg2-dev patchelf build-essential
npm install
npm run tauri dev      # run it
npm run tauri build    # build installers
```

## Brug

1. **Åbn en diskette eller en kørsel.** Klik på **Åbn .img…** for at åbne et Bosch-diskettebillede,
   eller **Åbn .ERG…** for en enkelt kørselsfil. Du kan også starte den fra kommandolinjen
   med en filsti.
2. **Gennemse kørsler.** Fanen **Diskette** viser alle kørsler på billedet; slettede kørsler
   vises overstreget. Klik på en kørsel for at se dens **effekt**- og **moment**-kurver
   (kW / Nm) og dens værdier (Pmax, Phjul, Ptab, Mmax, k, …).
3. **Importér til dit bibliotek.** **Importér nye** tilføjer kørsler, der endnu ikke er gemt;
   **Importér alle…** genimporterer alt og overskriver eksisterende kopier (med en advarsel).
   Importerede kørsler dublet-fjernes efter indhold, så genimport af en diskette kun tilføjer det nye.
4. **Find kørsler senere.** Fanen **Bibliotek** viser dine gemte kørsler efter dato. Tilføj en
   **beskrivelse** (kunde, køretøj, noter), og søg i den.
5. **Udskriv eller eksportér PDF.** Klik på **Udskriv / PDF…** og vælg din printer, eller vælg
   "Gem som PDF" / "Udskriv til fil" i udskriftsdialogen.
6. **Nulstil en diskette.** **Nulstil diskette…** sletter kun dynokørslerne fra billedet, mens
   maskinens indstillinger og kalibrering (`FLA.CFG`), sprog og skrifttyper bevares. Der gemmes
   først en tidsstemplet sikkerhedskopi af billedet.
7. **Indstillinger (⚙).** Vælg **sprog** og **enhedssystem** (metrisk kW/Nm/°C eller
   imperisk bhp/lb·ft/°F). Dit valg huskes.

## Hvor dine data gemmes

Importerede kørsler og indstillinger ligger i en `fla-dynoview`-mappe:

- **Windows:** `%APPDATA%\fla-dynoview\`
- **macOS:** `~/Library/Application Support/fla-dynoview/`
- **Linux:** `~/.local/share/fla-dynoview/`

Den indeholder `settings.json`, `db/`-kørselsbiblioteket (læsbar JSON, organiseret efter
dato, med den originale `.ERG` ved siden af hver kørsel) og `db/backups/`-diskettesikkerhedskopier.

> Maskinens eget sprog er en runtime-indstilling, der ikke gemmes på disketten, så appen
> kan ikke registrere det automatisk — vælg dit sprog under Indstillinger.
