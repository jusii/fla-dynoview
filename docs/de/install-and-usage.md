# FLA Dynoview – Installation & Bedienung (Deutsch)

Bosch FLA 203 Messläufe auf einem modernen PC lesen und archivieren sowie drucken oder als PDF exportieren.

## Installation

### Option A – Fertiges Build herunterladen
Laden Sie die Datei für Ihr System von der [Releases-Seite](https://github.com/jusii/fla-dynoview/releases) herunter:

- **Windows** – `FLA.Dynoview_x64-setup.exe` (Installer)
- **macOS** – `FLA.Dynoview_aarch64.dmg` (Apple Silicon) oder `FLA.Dynoview_x64.dmg` (Intel)
- **Linux** – `FLA.Dynoview_amd64.AppImage` (portabel) oder `..._amd64.deb`

Die Builds sind **nicht signiert**, daher warnt das Betriebssystem beim ersten Start:

- **macOS:** Rechtsklick auf die App → **Öffnen** (einmalig), oder führen Sie `xattr -cr "/Applications/FLA Dynoview.app"` aus.
- **Windows:** SmartScreen → **Weitere Informationen → Trotzdem ausführen**.
- **Linux (AppImage):** `chmod +x FLA.Dynoview_*.AppImage`, dann ausführen.

### Option B – Aus dem Quellcode bauen
Erfordert Rust, Node.js 20+ und die Tauri-Voraussetzungen. Unter Debian/Ubuntu:

```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev \
  libjavascriptcoregtk-4.1-dev librsvg2-dev patchelf build-essential
npm install
npm run tauri dev      # run it
npm run tauri build    # build installers
```

## Bedienung

1. **Diskette oder Messlauf öffnen.** Klicken Sie auf **.img öffnen…**, um ein Bosch-Diskettenabbild
   zu öffnen, oder auf **.ERG öffnen…** für eine einzelne Messlauf-Datei. Sie können die App auch über
   die Kommandozeile mit einem Dateipfad starten.
2. **Messläufe durchsuchen.** Die Registerkarte **Diskette** listet jeden Messlauf auf dem Abbild auf;
   gelöschte Messläufe werden durchgestrichen dargestellt. Klicken Sie auf einen Messlauf, um seine
   **Leistungs-** und **Drehmoment**-Diagramme (kW / Nm) sowie seine Werte
   (Pmax, Ppyörä, Phäviö, Mmax, k, …) anzuzeigen.
3. **In Ihre Bibliothek importieren.** **Neue importieren** fügt noch nicht gespeicherte Messläufe hinzu;
   **Alle importieren…** importiert alles erneut und überschreibt vorhandene Kopien (mit Warnung).
   Importierte Messläufe werden anhand ihres Inhalts entdoppelt, sodass beim erneuten Importieren einer
   Diskette nur das Neue hinzugefügt wird.
4. **Messläufe später wiederfinden.** Die Registerkarte **Bibliothek** listet Ihre gespeicherten Messläufe
   nach Datum auf. Fügen Sie eine **Beschreibung** hinzu (Kunde, Fahrzeug, Notizen) und durchsuchen Sie sie.
5. **Drucken oder als PDF exportieren.** Klicken Sie auf **Drucken / PDF…** und wählen Sie Ihren Drucker,
   oder wählen Sie im Druckdialog „Als PDF speichern“ / „In Datei drucken“.
6. **Diskette zurücksetzen.** **Diskette zurücksetzen…** löscht nur die Messläufe vom Abbild und behält
   dabei die Einstellungen und Kalibrierung des Geräts (`FLA.CFG`), Sprachen und Schriftarten. Zuvor wird
   eine mit Zeitstempel versehene Sicherung des Abbilds gespeichert.
7. **Einstellungen (⚙).** Wählen Sie die **Sprache** und das **Einheitensystem** (metrisch kW/Nm/°C oder
   imperial bhp/lb·ft/°F). Ihre Wahl wird gespeichert.

## Wo Ihre Daten gespeichert werden

Importierte Messläufe und Einstellungen befinden sich in einem `fla-dynoview`-Ordner:

- **Windows:** `%APPDATA%\fla-dynoview\`
- **macOS:** `~/Library/Application Support/fla-dynoview/`
- **Linux:** `~/.local/share/fla-dynoview/`

Er enthält `settings.json`, die Messlauf-Bibliothek `db/` (menschenlesbares JSON, nach Datum geordnet, mit
der originalen `.ERG` neben jedem Messlauf) sowie die Disketten-Sicherungen unter `db/backups/`.

> Die Sprache des Geräts selbst ist eine Laufzeiteinstellung, die nicht auf der Diskette gespeichert wird,
> sodass die App sie nicht automatisch erkennen kann – wählen Sie Ihre Sprache in den Einstellungen.
