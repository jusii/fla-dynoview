# FLA Dynoview — インストールと使い方（日本語）

Bosch FLA 203 の測定データを最新の PC で読み込んで保存し、印刷または PDF で書き出します。

## インストール

### 方法 A — ビルド済みバイナリをダウンロードする
お使いのシステムに合ったファイルを [Releases ページ](https://github.com/jusii/fla-dynoview/releases) から入手してください。

- **Windows** — `FLA.Dynoview_x64-setup.exe`（インストーラー）
- **macOS** — `FLA.Dynoview_aarch64.dmg`（Apple Silicon）または `FLA.Dynoview_x64.dmg`（Intel）
- **Linux** — `FLA.Dynoview_amd64.AppImage`（ポータブル）または `..._amd64.deb`

これらのビルドは**署名されていない**ため、初回起動時に OS が警告を表示します。

- **macOS:** アプリを右クリック → **開く**（初回のみ）、または `xattr -cr "/Applications/FLA Dynoview.app"` を実行します。
- **Windows:** SmartScreen → **詳細情報 → 実行**。
- **Linux（AppImage）:** `chmod +x FLA.Dynoview_*.AppImage` を実行してから起動します。

### 方法 B — ソースからビルドする
Rust、Node.js 20 以降、および Tauri の前提条件が必要です。Debian/Ubuntu の場合:

```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev \
  libjavascriptcoregtk-4.1-dev librsvg2-dev patchelf build-essential
npm install
npm run tauri dev      # run it
npm run tauri build    # build installers
```

## 使い方

1. **ディスクまたは測定を開く。** **.img を開く…** をクリックして Bosch のフロッピーイメージを開くか、
   **.ERG を開く…** で単一の測定ファイルを開きます。ファイルパスを指定してコマンドラインから
   起動することもできます。
2. **測定を閲覧する。** **ディスク** タブにはイメージ上のすべての測定が一覧表示され、削除された測定は
   取り消し線付きで表示されます。測定をクリックすると、**出力** と **トルク** のチャート
   （kW / Nm）とその数値（Pmax、Ppyörä、Phäviö、Mmax、k、…）が表示されます。
3. **ライブラリにインポートする。** **新規インポート** はまだ保存されていない測定を追加します。
   **すべてインポート…** はすべてを再インポートし、既存のコピーを（警告付きで）上書きします。
   インポートされた測定は内容によって重複が除外されるため、同じディスクを再インポートしても
   新しいものだけが追加されます。
4. **後で測定を探す。** **ライブラリ** タブには保存した測定が日付順に一覧表示されます。
   **説明**（顧客、車両、メモ）を追加して検索できます。
5. **印刷または PDF で書き出す。** **印刷 / PDF…** をクリックしてプリンターを選択するか、
   印刷ダイアログで「PDF として保存」/「ファイルへ出力」を選びます。
6. **ディスクをリセットする。** **ディスクをリセット…** はイメージから測定データのみを削除し、
   機器の設定と校正データ（`FLA.CFG`）、言語、フォントは保持します。先にタイムスタンプ付きの
   イメージのバックアップが保存されます。
7. **設定（⚙）。** **言語** と **単位系**（メートル法 kW/Nm/°C またはヤード・ポンド法
   bhp/lb·ft/°F）を選択します。選択した内容は保存されます。

## データの保存場所

インポートした測定と設定は `fla-dynoview` フォルダに保存されます。

- **Windows:** `%APPDATA%\fla-dynoview\`
- **macOS:** `~/Library/Application Support/fla-dynoview/`
- **Linux:** `~/.local/share/fla-dynoview/`

このフォルダには `settings.json`、`db/` 測定ライブラリ（人間が読める JSON 形式で、日付ごとに
整理され、各測定には元の `.ERG` が併置されています）、および `db/backups/` のディスクバックアップが
含まれます。

> 機器自体の言語はディスクに保存されないランタイム設定のため、アプリが自動的に検出することは
> できません。設定で言語を選択してください。
