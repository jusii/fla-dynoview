# FLA Dynoview — Asennus & käyttö (Suomi)

Lue ja arkistoi Bosch FLA 203 -dynoajot nykyaikaisella tietokoneella, ja tulosta tai vie PDF.

## Asennus

### Vaihtoehto A — lataa valmis versio
Hae järjestelmääsi sopiva tiedosto [Releases-sivulta](https://github.com/jusii/fla-dynoview/releases):

- **Windows** — `FLA.Dynoview_x64-setup.exe` (asennusohjelma)
- **macOS** — `FLA.Dynoview_aarch64.dmg` (Apple Silicon) tai `FLA.Dynoview_x64.dmg` (Intel)
- **Linux** — `FLA.Dynoview_amd64.AppImage` (kannettava) tai `..._amd64.deb`

Versiot ovat **allekirjoittamattomia**, joten käyttöjärjestelmä varoittaa ensimmäisellä
käynnistyksellä:

- **macOS:** napsauta sovellusta hiiren oikealla → **Avaa** (kerran), tai suorita
  `xattr -cr "/Applications/FLA Dynoview.app"`.
- **Windows:** SmartScreen → **Lisätietoja → Suorita silti**.
- **Linux (AppImage):** `chmod +x FLA.Dynoview_*.AppImage` ja käynnistä se.

### Vaihtoehto B — käännä lähdekoodista
Vaatii Rustin, Node.js 20+ ja Taurin esivaatimukset. Debian/Ubuntu:

```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev \
  libjavascriptcoregtk-4.1-dev librsvg2-dev patchelf build-essential
npm install
npm run tauri dev      # suorita
npm run tauri build    # rakenna asennuspaketit
```

## Käyttö

1. **Avaa levy tai ajo.** Napsauta **Avaa .img…** avataksesi Bosch-levykkeen kuvan, tai
   **Avaa .ERG…** yksittäistä ajotiedostoa varten. Voit myös käynnistää sovelluksen
   komentoriviltä tiedostopolulla.
2. **Selaa ajoja.** **Levy**-välilehti listaa levyn kaikki ajot; poistetut ajot näkyvät
   yliviivattuina. Napsauta ajoa nähdäksesi sen **teho-** ja **momenttikäyrät** (kW / Nm)
   ja arvot (Pmax, Ppyörä, Phäviö, Mmax, k, …).
3. **Tuo kirjastoon.** **Tuo uudet** lisää vielä tallentamattomat ajot; **Tuo kaikki…**
   tuo kaiken uudelleen ja korvaa aiemmat (varoituksen kera). Tuodut ajot tunnistetaan
   sisällön perusteella, joten levyn uudelleentuonti lisää vain uudet ajot.
4. **Löydä ajot myöhemmin.** **Kirjasto**-välilehti listaa tallennetut ajot päivämäärän
   mukaan. Lisää **kuvaus** (asiakas, ajoneuvo, muistiinpanot) ja hae sitä.
5. **Tulosta tai vie PDF.** Napsauta **Tulosta / PDF…** ja valitse tulostin, tai valitse
   tulostusikkunassa ”Tallenna PDF-muodossa” / ”Tulosta tiedostoon”.
6. **Tyhjennä levy.** **Tyhjennä levy…** poistaa levykuvasta vain dynoajot ja säilyttää
   koneen asetukset ja kalibroinnin (`FLA.CFG`), kielet ja fontit. Levystä tallennetaan
   ensin aikaleimattu varmuuskopio.
7. **Asetukset (⚙).** Valitse **kieli** ja **yksikköjärjestelmä** (metrinen kW/Nm/°C tai
   brittiläinen bhp/lb·ft/°F). Valintasi muistetaan.

## Missä tiedot säilytetään

Tuodut ajot ja asetukset ovat `fla-dynoview`-kansiossa:

- **Windows:** `%APPDATA%\fla-dynoview\`
- **macOS:** `~/Library/Application Support/fla-dynoview/`
- **Linux:** `~/.local/share/fla-dynoview/`

Se sisältää `settings.json`-tiedoston, `db/`-ajokirjaston (ihmisluettavaa JSON:ia,
järjestettynä päivämäärän mukaan, alkuperäinen `.ERG` kunkin ajon vieressä) ja
`db/backups/`-levyvarmuuskopiot.

> Koneen oma kieli on ajonaikainen asetus, jota ei tallenneta levylle, joten sovellus ei
> voi tunnistaa sitä automaattisesti — valitse kieli Asetuksista.
