# FLA Dynoview — Instalación y uso (Español)

Lea y archive mediciones de banco Bosch FLA 203 en un PC moderno, e imprima o exporte un PDF.

## Instalación

### Opción A — descargar una versión ya compilada
Descargue el archivo correspondiente a su sistema desde la [página de versiones](../../releases):

- **Windows** — `FLA.Dynoview_x64-setup.exe` (instalador)
- **macOS** — `FLA.Dynoview_aarch64.dmg` (Apple Silicon) o `FLA.Dynoview_x64.dmg` (Intel)
- **Linux** — `FLA.Dynoview_amd64.AppImage` (portable) o `..._amd64.deb`

Las versiones están **sin firmar**, por lo que el sistema operativo muestra una advertencia en el primer arranque:

- **macOS:** haga clic derecho en la aplicación → **Abrir** (una vez), o ejecute `xattr -cr "/Applications/FLA Dynoview.app"`.
- **Windows:** SmartScreen → **Más información → Ejecutar de todos modos**.
- **Linux (AppImage):** `chmod +x FLA.Dynoview_*.AppImage` y luego ejecútelo.

### Opción B — compilar desde el código fuente
Requiere Rust, Node.js 20+ y los requisitos previos de Tauri. En Debian/Ubuntu:

```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev \
  libjavascriptcoregtk-4.1-dev librsvg2-dev patchelf build-essential
npm install
npm run tauri dev      # run it
npm run tauri build    # build installers
```

## Uso

1. **Abra un disco o una medición.** Haga clic en **Abrir .img…** para abrir una imagen
   de disquete Bosch, o en **Abrir .ERG…** para un único archivo de medición. También
   puede iniciarla desde la línea de comandos con una ruta de archivo.
2. **Explore las mediciones.** La pestaña **Disco** enumera todas las mediciones de la
   imagen; las mediciones eliminadas se muestran tachadas. Haga clic en una medición para
   ver sus gráficos de **potencia** y **par** (kW / Nm) y sus valores (Pmax, Ppyörä,
   Phäviö, Mmax, k, …).
3. **Importe a su biblioteca.** **Importar nuevas** añade las mediciones que aún no se han
   guardado; **Importar todas…** reimporta todo y sobrescribe las copias existentes (con
   una advertencia). Las mediciones importadas se desduplican por contenido, de modo que
   reimportar un disco solo añade lo nuevo.
4. **Encuentre mediciones más tarde.** La pestaña **Biblioteca** enumera sus mediciones
   guardadas por fecha. Añada una **descripción** (cliente, vehículo, notas) y búsquela.
5. **Imprima o exporte un PDF.** Haga clic en **Imprimir / PDF…** y elija su impresora, o
   seleccione «Guardar como PDF» / «Imprimir a un archivo» en el cuadro de diálogo de
   impresión.
6. **Restablezca un disco.** **Restablecer disco…** elimina únicamente las mediciones de
   banco de la imagen, conservando los ajustes y la calibración de la máquina (`FLA.CFG`),
   los idiomas y las fuentes. Primero se guarda una copia de seguridad de la imagen con
   marca de tiempo.
7. **Ajustes (⚙).** Elija el **idioma** y el **sistema de unidades** (métrico kW/Nm/°C o
   imperial bhp/lb·ft/°F). Se recuerda su elección.

## Dónde se almacenan sus datos

Las mediciones importadas y los ajustes se guardan en una carpeta `fla-dynoview`:

- **Windows:** `%APPDATA%\fla-dynoview\`
- **macOS:** `~/Library/Application Support/fla-dynoview/`
- **Linux:** `~/.local/share/fla-dynoview/`

Contiene `settings.json`, la biblioteca de mediciones `db/` (JSON legible por humanos,
organizado por fecha, con el `.ERG` original junto a cada medición) y las copias de
seguridad de disco `db/backups/`.

> El idioma de la propia máquina es un ajuste de tiempo de ejecución que no se guarda en el
> disco, por lo que la aplicación no puede detectarlo automáticamente: elija su idioma en
> Ajustes.
