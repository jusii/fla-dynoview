# FLA Dynoview — Installazione e uso (Italiano)

Leggi e archivia le prove al banco Bosch FLA 203 su un PC moderno, e stampa o esporta un PDF.

## Installazione

### Opzione A — scarica una build pronta
Scarica il file adatto al tuo sistema dalla [pagina Releases](https://github.com/jusii/fla-dynoview/releases):

- **Windows** — `FLA.Dynoview_x64-setup.exe` (programma di installazione)
- **macOS** — `FLA.Dynoview_aarch64.dmg` (Apple Silicon) oppure `FLA.Dynoview_x64.dmg` (Intel)
- **Linux** — `FLA.Dynoview_amd64.AppImage` (portabile) oppure `..._amd64.deb`

Le build **non sono firmate**, quindi il sistema operativo mostra un avviso al primo avvio:

- **macOS:** fai clic destro sull'app → **Apri** (una volta), oppure esegui `xattr -cr "/Applications/FLA Dynoview.app"`.
- **Windows:** SmartScreen → **Ulteriori informazioni → Esegui comunque**.
- **Linux (AppImage):** `chmod +x FLA.Dynoview_*.AppImage` e poi eseguilo.

### Opzione B — compila dai sorgenti
Richiede Rust, Node.js 20+ e i prerequisiti di Tauri. Su Debian/Ubuntu:

```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev \
  libjavascriptcoregtk-4.1-dev librsvg2-dev patchelf build-essential
npm install
npm run tauri dev      # run it
npm run tauri build    # build installers
```

## Uso

1. **Apri un disco o una prova.** Fai clic su **Apri .img…** per aprire un'immagine
   floppy Bosch, oppure **Apri .ERG…** per un singolo file di prova. Puoi anche avviarlo
   dalla riga di comando indicando il percorso di un file.
2. **Sfoglia le prove.** La scheda **Disco** elenca tutte le prove presenti
   sull'immagine; le prove eliminate sono mostrate barrate. Fai clic su una prova per
   vederne i grafici di **potenza** e **coppia** (kW / Nm) e i relativi valori
   (Pmass, Pruota, Ppersa, Mmax, k, …).
3. **Importa nella tua libreria.** **Importa nuovi** aggiunge le prove non ancora
   salvate; **Importa tutti…** reimporta tutto e sovrascrive le copie esistenti (con un
   avviso). Le prove importate vengono deduplicate in base al contenuto, quindi
   reimportare un disco aggiunge solo ciò che è nuovo.
4. **Ritrova le prove in seguito.** La scheda **Libreria** elenca le prove salvate per
   data. Aggiungi una **descrizione** (cliente, veicolo, note) e cercala.
5. **Stampa o esporta in PDF.** Fai clic su **Stampa / PDF…** e scegli la stampante,
   oppure seleziona "Salva come PDF" / "Stampa su file" nella finestra di stampa.
6. **Reimposta un disco.** **Reimposta disco…** elimina dall'immagine solo le prove al
   banco, mantenendo le impostazioni e la calibrazione della macchina (`FLA.CFG`), le
   lingue e i font. Viene prima salvato un backup dell'immagine con data e ora.
7. **Impostazioni (⚙).** Scegli la **lingua** e il **sistema di unità** (metrico kW/Nm/°C
   oppure imperiale bhp/lb·ft/°F). La tua scelta viene ricordata.

## Dove vengono salvati i dati

Le prove importate e le impostazioni si trovano in una cartella `fla-dynoview`:

- **Windows:** `%APPDATA%\fla-dynoview\`
- **macOS:** `~/Library/Application Support/fla-dynoview/`
- **Linux:** `~/.local/share/fla-dynoview/`

Contiene `settings.json`, la libreria delle prove `db/` (JSON leggibile, organizzato per
data, con il file `.ERG` originale accanto a ogni prova) e i backup del disco in
`db/backups/`.

> La lingua della macchina è un'impostazione di runtime che non viene salvata sul disco,
> quindi l'app non può rilevarla automaticamente — scegli la tua lingua nelle
> Impostazioni.
