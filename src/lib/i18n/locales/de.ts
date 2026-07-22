import type { Messages } from "../index";

// Deutsch — getypt gegen den kanonischen Katalog (Messages), daher sind fehlende
// Schlüssel ein Kompilierfehler. Prüfstandsbegriffe/-abkürzungen verwenden Boschs
// eigene Formulierung (aus der \TXT\ENGL_KW-Tabelle); Einheiten kommen aus units.ts.
const de: Messages = {
  // Symbolleiste / Rahmen
  "app.openImg": ".img öffnen…",
  "app.openErg": ".ERG öffnen…",
  "app.tabDisk": "Diskette",
  "app.tabLibrary": "Bibliothek",
  "app.printPdf": "Drucken / PDF…",
  "app.settings": "Einstellungen",
  "app.shopFallback": "(kein Werkstattname)",
  // Seitenleisten-Aktionen
  "app.importNew": "Neue importieren ({count})",
  "app.importAll": "Alle importieren…",
  "app.resetDisk": "Diskette zurücksetzen…",
  "app.runsHeader": "{count} Messläufe",
  "app.inLibrary": "In Bibliothek",
  "app.deleted": "gelöscht",
  "app.browseEmpty":
    "Öffnen Sie ein Bosch FLA 203 Diskettenabbild (.img), um dessen Messläufe zu durchsuchen, oder öffnen Sie eine einzelne .ERG-Datei.",
  // Bibliothek
  "app.searchPlaceholder": "Beschreibung / Datum suchen…",
  "app.libraryHeader": "{count} in Bibliothek",
  "app.noDescription": "(keine Beschreibung)",
  "app.libraryEmpty": "Noch keine Messläufe importiert. Öffnen Sie eine Diskette und „Neue importieren“.",
  // Detail / Editor
  "app.description": "Beschreibung",
  "app.descPlaceholder": "Kunde, Fahrzeug, Notizen…",
  "app.save": "Speichern",
  "app.deleteFromLibrary": "Aus Bibliothek löschen",
  "app.loading": "Wird geladen…",
  "app.selectRun": "Wählen Sie einen Messlauf, um seine Kurven anzuzeigen.",
  // Meldungen (Toasts)
  "toast.nothingNew": "Nichts Neues zu importieren – alle Messläufe sind bereits in der Bibliothek.",
  "toast.imported": "{added} neue Messläufe importiert; {skipped} übersprungen.",
  "toast.importedOver": "{added} importiert, {overwritten} überschrieben.",
  "toast.wiped": "{deleted} Messläufe gelöscht. Sicherung gespeichert unter {path}",
  "toast.descSaved": "Beschreibung gespeichert.",
  // Dialoge
  "dialog.importAllMsg":
    "ALLE {count} Messläufe erneut importieren und vorhandene Kopien in der Bibliothek ÜBERSCHREIBEN?",
  "dialog.importAllTitle": "Alle importieren / überschreiben",
  "dialog.resetMsg":
    "ALLE Messläufe von diesem Diskettenabbild löschen?\n\nEinstellungen & Kalibrierung (FLA.CFG), Sprachtabellen und Schriftarten bleiben erhalten. Zuvor wird eine mit Zeitstempel versehene Sicherung des Abbilds gespeichert.\n\n{path}",
  "dialog.resetTitle": "Diskette zurücksetzen",
  "dialog.deleteMsg": "Diesen Messlauf aus Ihrer Bibliothek entfernen? (Die Quelldiskette wird nicht verändert.)",
  "dialog.deleteTitle": "Messlauf löschen",
  // Einstellungsbereich
  "settings.title": "Einstellungen",
  "settings.language": "Sprache",
  "settings.units": "Einheiten",
  "settings.unitMetric": "Metrisch (kW, Nm, °C)",
  "settings.unitImperial": "Imperial (bhp, lb·ft, °F)",
  "settings.revealFolder": "Datenordner öffnen",
  "settings.close": "Schließen",
  // Prüfstand-Vollbegriffe (Bosch)
  "term.engine": "Motorleistung",
  "term.torque": "Drehmoment",
  "term.wheel": "Radleistung",
  "term.loss": "Rad- Verlustleistung",
  // Prüfstand-Abkürzungen (Bosch)
  "abbr.pmax": "Pmax",
  "abbr.pnim": "Pnenn",
  "abbr.ppyora": "Prad",
  "abbr.phavio": "Pverl",
  "abbr.mmax": "Mmax",
  "abbr.paine": "Druck",
  "abbr.lamp": "Temp.",
  "abbr.k": "k",
  // Detail / Diagramm
  "detail.atN": "@ n",
  "detail.atV": "@ v",
  "detail.noDate": "kein Datum",
  "detail.noPower": "Keine Leistungskurve in diesem Messlauf.",
  "detail.noTorque": "Keine Drehmomentkurve in diesem Messlauf.",
  "chart.sweep": "Verlauf →",
  "chart.versus": "über",
  // Druck
  "print.reportTitle": "Prüfstandsbericht",
  "print.runDate": "Messdatum",
  "print.generatedBy": "Erstellt mit FLA Dynoview",
  "crop.trim": "Zuschneiden",
  "crop.reset": "Zurücksetzen",
  "app.runDate": "Datum",
  "app.dateNow": "Jetzt",
  "app.saveReadings": "Messwerte speichern",
  "toast.readingsSaved": "Messwerte gespeichert.",
  "app.compare": "Vergleichen ({count})",
  "app.comparison": "Vergleich",
  "compare.selectHint": "2–4 Messläufe zum Überlagern auswählen.",
  "compare.checkTitle": "Zum Vergleich hinzufügen",
  "compare.close": "Vergleich schließen",
  "compare.lineNote": "Durchgezogene Linie = Leistung · gestrichelte Linie = Drehmoment.",
  "settings.fieldLabels": "Feldbeschriftungen",
  "settings.fieldLabelsHint": "Benennen Sie die auf dem Bildschirm und in Ausdrucken angezeigten Beschriftungen um. Für die Standardvorgabe leer lassen.",
  "settings.resetLabels": "Beschriftungen zurücksetzen",
  "settings.logo": "Logo drucken",
  "settings.chooseLogo": "Bild auswählen…",
  "settings.removeLogo": "Logo entfernen",
  "settings.headerText": "Kopfzeilentext drucken",
  "settings.headerHint": "Leer lassen, um den Werkstattnamen zu verwenden.",
};

export default de;
