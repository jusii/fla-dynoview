import type { Messages } from "../index";

// Nederlands — getypt tegen de canonieke catalogus (Messages), dus ontbrekende
// sleutels zijn een compilatiefout. Dynotermen/-afkortingen gebruiken Bosch' eigen
// bewoording (uit de \TXT\ENGL_KW-tabel); eenheden komen uit units.ts.
const nl: Messages = {
  // werkbalk / kader
  "app.openImg": ".img openen…",
  "app.openErg": ".ERG openen…",
  "app.tabDisk": "Diskette",
  "app.tabLibrary": "Bibliotheek",
  "app.printPdf": "Afdrukken / PDF…",
  "app.settings": "Instellingen",
  "app.shopFallback": "(geen werkplaatsnaam)",
  // zijbalk-acties
  "app.importNew": "Nieuwe importeren ({count})",
  "app.importAll": "Alles importeren…",
  "app.resetDisk": "Diskette resetten…",
  "app.runsHeader": "{count} metingen",
  "app.inLibrary": "In bibliotheek",
  "app.deleted": "verwijderd",
  "app.browseEmpty":
    "Open een Bosch FLA 203 diskette-image (.img) om de vermogensmetingen te bekijken, of open een los .ERG-bestand.",
  // bibliotheek
  "app.searchPlaceholder": "Zoek op omschrijving / datum…",
  "app.libraryHeader": "{count} in bibliotheek",
  "app.noDescription": "(geen omschrijving)",
  "app.libraryEmpty": "Nog geen metingen geïmporteerd. Open een diskette en klik op “Nieuwe importeren”.",
  // detail / editor
  "app.description": "Omschrijving",
  "app.descPlaceholder": "Klant, voertuig, notities…",
  "app.save": "Opslaan",
  "app.deleteFromLibrary": "Uit bibliotheek verwijderen",
  "app.loading": "Laden…",
  "app.selectRun": "Selecteer een meting om de krommen te bekijken.",
  // meldingen (toasts)
  "toast.nothingNew": "Niets nieuws om te importeren — alle metingen staan al in de bibliotheek.",
  "toast.imported": "{added} nieuwe meting(en) geïmporteerd; {skipped} overgeslagen.",
  "toast.importedOver": "{added} geïmporteerd, {overwritten} overschreven.",
  "toast.wiped": "{deleted} meting(en) gewist. Back-up opgeslagen in {path}",
  "toast.descSaved": "Omschrijving opgeslagen.",
  // dialoogvensters
  "dialog.importAllMsg":
    "ALLE {count} metingen opnieuw importeren en bestaande kopieën in de bibliotheek OVERSCHRIJVEN?",
  "dialog.importAllTitle": "Alles importeren / overschrijven",
  "dialog.resetMsg":
    "ALLE vermogensmetingen van deze diskette-image verwijderen?\n\nInstellingen & kalibratie (FLA.CFG), taaltabellen en lettertypen blijven behouden. Er wordt eerst een back-up van de image met tijdstempel opgeslagen.\n\n{path}",
  "dialog.resetTitle": "Diskette resetten",
  "dialog.deleteMsg": "Deze meting uit je bibliotheek verwijderen? (De brondiskette blijft ongewijzigd.)",
  "dialog.deleteTitle": "Meting verwijderen",
  // instellingenpaneel
  "settings.title": "Instellingen",
  "settings.language": "Taal",
  "settings.units": "Eenheden",
  "settings.unitMetric": "Metrisch (kW, Nm, °C)",
  "settings.unitImperial": "Imperiaal (bhp, lb·ft, °F)",
  "settings.revealFolder": "Datamap openen",
  "settings.close": "Sluiten",
  // volledige dynotermen (Bosch)
  "term.engine": "motorvermogen",
  "term.torque": "koppel",
  "term.wheel": "wielvermogen",
  "term.loss": "vermogenverlies wiel",
  // dyno-afkortingen (Bosch)
  "abbr.pmax": "Pmax",
  "abbr.pnim": "Popg.",
  "abbr.ppyora": "Pwiel",
  "abbr.phavio": "Pverl",
  "abbr.mmax": "Mmax",
  "abbr.paine": "Druk",
  "abbr.lamp": "Temp.",
  "abbr.k": "k",
  // detail / grafiek
  "detail.atN": "@ n",
  "detail.noDate": "geen datum",
  "detail.noPower": "Geen vermogenskromme in deze meting.",
  "detail.noTorque": "Geen koppelkromme in deze meting.",
  "chart.sweep": "Verloop →",
  "chart.versus": "over",
  // afdrukken
  "print.reportTitle": "Vermogensrapport",
  "print.runDate": "Meetdatum",
  "print.generatedBy": "Gegenereerd door FLA Dynoview",
};

export default nl;
