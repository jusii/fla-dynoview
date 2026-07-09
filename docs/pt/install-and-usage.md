# FLA Dynoview — Instalação e utilização (Português)

Leia e arquive ensaios dinamométricos do Bosch FLA 203 num PC moderno, e imprima ou exporte um PDF.

## Instalação

### Opção A — transferir uma versão pronta a usar
Obtenha o ficheiro para o seu sistema na [página de Releases](https://github.com/jusii/fla-dynoview/releases):

- **Windows** — `FLA.Dynoview_x64-setup.exe` (instalador)
- **macOS** — `FLA.Dynoview_aarch64.dmg` (Apple Silicon) ou `FLA.Dynoview_x64.dmg` (Intel)
- **Linux** — `FLA.Dynoview_amd64.AppImage` (portátil) ou `..._amd64.deb`

As versões (builds) **não estão assinadas**, por isso o sistema operativo avisa no primeiro arranque:

- **macOS:** clique com o botão direito na aplicação → **Abrir** (uma vez), ou execute `xattr -cr "/Applications/FLA Dynoview.app"`.
- **Windows:** SmartScreen → **Mais informações → Executar mesmo assim**.
- **Linux (AppImage):** `chmod +x FLA.Dynoview_*.AppImage` e depois execute-o.

### Opção B — compilar a partir do código-fonte
Requer Rust, Node.js 20+ e os pré-requisitos do Tauri. No Debian/Ubuntu:

```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev \
  libjavascriptcoregtk-4.1-dev librsvg2-dev patchelf build-essential
npm install
npm run tauri dev      # run it
npm run tauri build    # build installers
```

## Utilização

1. **Abra uma disquete ou um ensaio.** Clique em **Abrir .img…** para abrir uma imagem
   de disquete Bosch, ou **Abrir .ERG…** para um ficheiro de ensaio individual. Também
   pode iniciá-lo a partir da linha de comandos com um caminho de ficheiro.
2. **Percorra os ensaios.** O separador **Disquete** lista todos os ensaios da imagem;
   os ensaios eliminados aparecem rasurados. Clique num ensaio para ver os gráficos de
   **potência** e de **torque** (kW / Nm) e os respetivos valores (Pmax, Ppyörä, Phäviö,
   Mmax, k, …).
3. **Importe para a sua biblioteca.** **Importar novos** adiciona os ensaios ainda não
   guardados; **Importar todos…** reimporta tudo e substitui as cópias existentes (com um
   aviso). Os ensaios importados são desduplicados pelo conteúdo, por isso reimportar uma
   disquete apenas adiciona o que é novo.
4. **Encontre ensaios mais tarde.** O separador **Biblioteca** lista os ensaios guardados
   por data. Adicione uma **descrição** (cliente, veículo, notas) e pesquise-a.
5. **Imprima ou exporte PDF.** Clique em **Imprimir / PDF…** e escolha a impressora, ou
   selecione "Guardar como PDF" / "Imprimir para ficheiro" na caixa de diálogo de impressão.
6. **Reponha uma disquete.** **Repor disquete…** elimina apenas os ensaios dinamométricos
   da imagem, mantendo as definições e a calibração da máquina (`FLA.CFG`), os idiomas e os
   tipos de letra. É guardada primeiro uma cópia de segurança da imagem com data e hora.
7. **Definições (⚙).** Escolha o **idioma** e o **sistema de unidades** (métrico kW/Nm/°C
   ou imperial bhp/lb·ft/°F). A sua escolha é memorizada.

## Onde os seus dados são guardados

Os ensaios importados e as definições ficam numa pasta `fla-dynoview`:

- **Windows:** `%APPDATA%\fla-dynoview\`
- **macOS:** `~/Library/Application Support/fla-dynoview/`
- **Linux:** `~/.local/share/fla-dynoview/`

Contém o `settings.json`, a biblioteca de ensaios `db/` (JSON legível, organizado por
data, com o `.ERG` original junto de cada ensaio) e as cópias de segurança das disquetes
em `db/backups/`.

> O idioma da própria máquina é uma definição de tempo de execução que não é guardada na
> disquete, por isso a aplicação não o consegue detetar automaticamente — escolha o seu
> idioma em Definições.
