# FLA Dynoview — Installatie & Gebruik (Nederlands)

Lees en archiveer Bosch FLA 203 vermogensmetingen op een moderne pc, en druk ze af of exporteer een PDF.

## Installatie

### Optie A — download een kant-en-klare build
Pak het bestand voor jouw systeem van de [Releases-pagina](../../releases):

- **Windows** — `FLA.Dynoview_x64-setup.exe` (installatieprogramma)
- **macOS** — `FLA.Dynoview_aarch64.dmg` (Apple Silicon) of `FLA.Dynoview_x64.dmg` (Intel)
- **Linux** — `FLA.Dynoview_amd64.AppImage` (draagbaar) of `..._amd64.deb`

De builds zijn **niet ondertekend**, dus het besturingssysteem waarschuwt bij de eerste keer starten:

- **macOS:** klik met de rechtermuisknop op de app → **Open** (eenmalig), of voer `xattr -cr "/Applications/FLA Dynoview.app"` uit.
- **Windows:** SmartScreen → **Meer info → Toch uitvoeren**.
- **Linux (AppImage):** `chmod +x FLA.Dynoview_*.AppImage` en voer het daarna uit.

### Optie B — bouwen vanuit de broncode
Vereist Rust, Node.js 20+ en de Tauri-vereisten. Op Debian/Ubuntu:

```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev \
  libjavascriptcoregtk-4.1-dev librsvg2-dev patchelf build-essential
npm install
npm run tauri dev      # run it
npm run tauri build    # build installers
```

## Gebruik

1. **Open een diskette of een meting.** Klik op **.img openen…** om een Bosch diskette-image te openen, of op **.ERG openen…** voor een los meetbestand. Je kunt de app ook vanaf de opdrachtregel starten met een bestandspad.
2. **Metingen bekijken.** Het tabblad **Diskette** toont elke meting op de image; verwijderde metingen worden doorgestreept weergegeven. Klik op een meting om de grafieken voor **vermogen** en **koppel** (kW / Nm) en de bijbehorende waarden (Pmax, Pwiel, Pverl, Mmax, k, …) te bekijken.
3. **Importeren naar je bibliotheek.** **Nieuwe importeren** voegt metingen toe die nog niet zijn opgeslagen; **Alles importeren…** importeert alles opnieuw en overschrijft bestaande kopieën (met een waarschuwing). Geïmporteerde metingen worden op inhoud ontdubbeld, dus bij het opnieuw importeren van een diskette wordt alleen toegevoegd wat nieuw is.
4. **Metingen later terugvinden.** Het tabblad **Bibliotheek** toont je opgeslagen metingen op datum. Voeg een **omschrijving** toe (klant, voertuig, notities) en doorzoek deze.
5. **Afdrukken of PDF exporteren.** Klik op **Afdrukken / PDF…** en kies je printer, of selecteer "Opslaan als PDF" / "Afdrukken naar bestand" in het afdrukvenster.
6. **Een diskette resetten.** **Diskette resetten…** verwijdert alleen de vermogensmetingen van de image en behoudt de instellingen en kalibratie van het apparaat (`FLA.CFG`), talen en lettertypen. Er wordt eerst een back-up van de image met tijdstempel opgeslagen.
7. **Instellingen (⚙).** Kies de **taal** en het **eenhedenstelsel** (metrisch kW/Nm/°C of imperiaal bhp/lb·ft/°F). Je keuze wordt onthouden.

## Waar je gegevens worden opgeslagen

Geïmporteerde metingen en instellingen bevinden zich in een map `fla-dynoview`:

- **Windows:** `%APPDATA%\fla-dynoview\`
- **macOS:** `~/Library/Application Support/fla-dynoview/`
- **Linux:** `~/.local/share/fla-dynoview/`

Deze map bevat `settings.json`, de metingenbibliotheek `db/` (leesbare JSON, geordend op datum, met het originele `.ERG`-bestand naast elke meting) en de schijfback-ups in `db/backups/`.

> De eigen taal van het apparaat is een runtime-instelling die niet op de diskette wordt opgeslagen, dus de app kan deze niet automatisch detecteren — kies je taal bij Instellingen.
