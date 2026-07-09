# FLA Dynoview — Installation et utilisation (Français)

Lisez et archivez les passages au banc Bosch FLA 203 sur un PC moderne, puis imprimez-les ou exportez-les en PDF.

## Installation

### Option A — télécharger une version prête à l'emploi
Récupérez le fichier correspondant à votre système sur la [page des versions](https://github.com/jusii/fla-dynoview/releases) :

- **Windows** — `FLA.Dynoview_x64-setup.exe` (programme d'installation)
- **macOS** — `FLA.Dynoview_aarch64.dmg` (Apple Silicon) ou `FLA.Dynoview_x64.dmg` (Intel)
- **Linux** — `FLA.Dynoview_amd64.AppImage` (portable) ou `..._amd64.deb`

Les versions ne sont **pas signées** ; le système d'exploitation affiche donc un avertissement au premier lancement :

- **macOS :** faites un clic droit sur l'application → **Ouvrir** (une seule fois), ou exécutez `xattr -cr "/Applications/FLA Dynoview.app"`.
- **Windows :** SmartScreen → **Informations complémentaires → Exécuter quand même**.
- **Linux (AppImage) :** `chmod +x FLA.Dynoview_*.AppImage` puis lancez-le.

### Option B — compiler depuis les sources
Nécessite Rust, Node.js 20+ et les prérequis Tauri. Sous Debian/Ubuntu :

```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev \
  libjavascriptcoregtk-4.1-dev librsvg2-dev patchelf build-essential
npm install
npm run tauri dev      # run it
npm run tauri build    # build installers
```

## Utilisation

1. **Ouvrir une disquette ou un passage.** Cliquez sur **Ouvrir .img…** pour ouvrir une
   image de disquette Bosch, ou sur **Ouvrir .ERG…** pour un fichier de passage isolé.
   Vous pouvez aussi lancer l'application depuis la ligne de commande en indiquant un
   chemin de fichier.
2. **Parcourir les passages.** L'onglet **Disquette** répertorie chaque passage de
   l'image ; les passages supprimés sont affichés barrés. Cliquez sur un passage pour
   afficher ses graphiques de **puissance** et de **couple** (kW / Nm) ainsi que ses
   valeurs (Pmax, Ppyörä, Phäviö, Mmax, k, …).
3. **Importer dans votre bibliothèque.** **Importer les nouveaux** ajoute les passages
   pas encore enregistrés ; **Tout importer…** réimporte l'ensemble et écrase les copies
   existantes (avec un avertissement). Les passages importés sont dédoublonnés d'après
   leur contenu : réimporter une disquette n'ajoute donc que les nouveautés.
4. **Retrouver des passages plus tard.** L'onglet **Bibliothèque** répertorie vos
   passages enregistrés par date. Ajoutez une **description** (client, véhicule, notes)
   et effectuez une recherche dedans.
5. **Imprimer ou exporter en PDF.** Cliquez sur **Imprimer / PDF…** et choisissez votre
   imprimante, ou sélectionnez « Enregistrer au format PDF » / « Imprimer dans un
   fichier » dans la boîte de dialogue d'impression.
6. **Réinitialiser une disquette.** **Réinitialiser la disquette…** supprime uniquement
   les passages au banc de l'image, tout en conservant les réglages et l'étalonnage de
   l'appareil (`FLA.CFG`), les langues et les polices. Une sauvegarde horodatée de
   l'image est enregistrée au préalable.
7. **Paramètres (⚙).** Choisissez la **langue** et le **système d'unités** (métrique
   kW/Nm/°C ou impérial bhp/lb·ft/°F). Votre choix est mémorisé.

## Où sont stockées vos données

Les passages importés et les réglages se trouvent dans un dossier `fla-dynoview` :

- **Windows :** `%APPDATA%\fla-dynoview\`
- **macOS :** `~/Library/Application Support/fla-dynoview/`
- **Linux :** `~/.local/share/fla-dynoview/`

Il contient `settings.json`, la bibliothèque de passages `db/` (JSON lisible par un
humain, organisé par date, avec le fichier `.ERG` d'origine à côté de chaque passage) et
les sauvegardes de disquette `db/backups/`.

> La langue de l'appareil lui-même est un réglage d'exécution qui n'est pas enregistré
> sur la disquette ; l'application ne peut donc pas la détecter automatiquement —
> choisissez votre langue dans les Paramètres.
