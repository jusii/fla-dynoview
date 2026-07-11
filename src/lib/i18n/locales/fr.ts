import type { Messages } from "../index";

// Français — typé contre le catalogue canonique (Messages) : les clés manquantes
// provoquent donc une erreur de compilation. Les termes/abréviations du banc
// reprennent la formulation propre à Bosch (issue de la table \TXT\ENGL_KW) ;
// les unités proviennent de units.ts, pas d'ici.
const fr: Messages = {
  // barre d'outils / interface
  "app.openImg": "Ouvrir .img…",
  "app.openErg": "Ouvrir .ERG…",
  "app.tabDisk": "Disquette",
  "app.tabLibrary": "Bibliothèque",
  "app.printPdf": "Imprimer / PDF…",
  "app.settings": "Paramètres",
  "app.shopFallback": "(nom d'atelier absent)",
  // actions de la barre latérale
  "app.importNew": "Importer les nouveaux ({count})",
  "app.importAll": "Tout importer…",
  "app.resetDisk": "Réinitialiser la disquette…",
  "app.runsHeader": "{count} passages",
  "app.inLibrary": "Dans la bibliothèque",
  "app.deleted": "supprimé",
  "app.browseEmpty":
    "Ouvrez une image de disquette Bosch FLA 203 (.img) pour parcourir ses passages au banc, ou ouvrez un fichier .ERG isolé.",
  // bibliothèque
  "app.searchPlaceholder": "Rechercher description / date…",
  "app.libraryHeader": "{count} dans la bibliothèque",
  "app.noDescription": "(pas de description)",
  "app.libraryEmpty": "Aucun passage importé pour l'instant. Ouvrez une disquette et « Importer les nouveaux ».",
  // détail / éditeur
  "app.description": "Description",
  "app.descPlaceholder": "Client, véhicule, notes…",
  "app.save": "Enregistrer",
  "app.deleteFromLibrary": "Supprimer de la bibliothèque",
  "app.loading": "Chargement…",
  "app.selectRun": "Sélectionnez un passage pour afficher ses courbes.",
  // notifications
  "toast.nothingNew": "Rien de nouveau à importer — tous les passages sont déjà dans la bibliothèque.",
  "toast.imported": "{added} nouveau(x) passage(s) importé(s) ; {skipped} ignoré(s).",
  "toast.importedOver": "{added} importé(s), {overwritten} écrasé(s).",
  "toast.wiped": "{deleted} passage(s) effacé(s). Sauvegarde enregistrée dans {path}",
  "toast.descSaved": "Description enregistrée.",
  // boîtes de dialogue
  "dialog.importAllMsg":
    "Réimporter la TOTALITÉ des {count} passages et ÉCRASER les copies déjà présentes dans la bibliothèque ?",
  "dialog.importAllTitle": "Tout importer / écraser",
  "dialog.resetMsg":
    "Supprimer TOUS les passages au banc de cette image de disquette ?\n\nLes réglages et l'étalonnage (FLA.CFG), les tables de langue et les polices sont conservés. Une sauvegarde horodatée de l'image est enregistrée au préalable.\n\n{path}",
  "dialog.resetTitle": "Réinitialiser la disquette",
  "dialog.deleteMsg": "Retirer ce passage de votre bibliothèque ? (La disquette source n'est pas modifiée.)",
  "dialog.deleteTitle": "Supprimer le passage",
  // panneau des paramètres
  "settings.title": "Paramètres",
  "settings.language": "Langue",
  "settings.units": "Unités",
  "settings.unitMetric": "Métrique (kW, Nm, °C)",
  "settings.unitImperial": "Impérial (bhp, lb·ft, °F)",
  "settings.revealFolder": "Ouvrir le dossier de données",
  "settings.close": "Fermer",
  // termes complets du banc (Bosch)
  "term.engine": "puissance moteur",
  "term.torque": "couple",
  "term.wheel": "puissance roue",
  "term.loss": "puiss.absorbée roues",
  // abréviations du banc (Bosch)
  "abbr.pmax": "P.max.",
  "abbr.pnim": "P.réel",
  "abbr.ppyora": "P.roue",
  "abbr.phavio": "P.per.",
  "abbr.mmax": "Mmax",
  "abbr.paine": "press.",
  "abbr.lamp": "temp.",
  "abbr.k": "k",
  // détail / graphique
  "detail.atN": "@ n",
  "detail.noDate": "pas de date",
  "detail.noPower": "Aucune courbe de puissance dans ce passage.",
  "detail.noTorque": "Aucune courbe de couple dans ce passage.",
  "chart.sweep": "balayage →",
  "chart.versus": "en fonction de",
  // impression
  "print.reportTitle": "Rapport dynamométrique",
  "print.runDate": "Date du passage",
  "print.generatedBy": "Généré par FLA Dynoview",
  "crop.trim": "Rogner",
  "crop.reset": "Réinit.",
  "app.runDate": "Date",
  "app.dateNow": "Maint.",
  "app.saveReadings": "Enregistrer les valeurs",
  "toast.readingsSaved": "Valeurs enregistrées.",
  "app.compare": "Comparer ({count})",
  "app.comparison": "Comparaison",
  "compare.selectHint": "Sélectionnez 2 à 4 passages à superposer.",
  "compare.checkTitle": "Ajouter à la comparaison",
  "compare.close": "Fermer la comparaison",
  "compare.lineNote": "Ligne continue = puissance · ligne pointillée = couple.",
  "settings.fieldLabels": "Libellés des champs",
  "settings.fieldLabelsHint": "Renommez les légendes affichées à l'écran et sur les impressions. Laissez vide pour la valeur par défaut.",
  "settings.resetLabels": "Réinitialiser les libellés",
  "settings.logo": "Logo d'impression",
  "settings.chooseLogo": "Choisir une image…",
  "settings.removeLogo": "Supprimer le logo",
  "settings.headerText": "Texte d'en-tête d'impression",
  "settings.headerHint": "Laissez vide pour utiliser le nom de l'atelier.",
};

export default fr;
