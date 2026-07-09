import type { Messages } from "../index";

// Español — traducido del catálogo canónico en inglés (en.ts).
// Los términos y abreviaturas del banco de potencia usan la terminología
// propia de Bosch; las unidades provienen de units.ts, no de aquí.
const es: Messages = {
  // barra de herramientas / interfaz
  "app.openImg": "Abrir .img…",
  "app.openErg": "Abrir .ERG…",
  "app.tabDisk": "Disco",
  "app.tabLibrary": "Biblioteca",
  "app.printPdf": "Imprimir / PDF…",
  "app.settings": "Ajustes",
  "app.shopFallback": "(sin nombre de taller)",
  // acciones de la barra lateral
  "app.importNew": "Importar nuevas ({count})",
  "app.importAll": "Importar todas…",
  "app.resetDisk": "Restablecer disco…",
  "app.runsHeader": "{count} mediciones",
  "app.inLibrary": "En la biblioteca",
  "app.deleted": "eliminada",
  "app.browseEmpty":
    "Abra una imagen de disquete Bosch FLA 203 (.img) para explorar sus mediciones de banco, o abra un único archivo .ERG.",
  // biblioteca
  "app.searchPlaceholder": "Buscar descripción / fecha…",
  "app.libraryHeader": "{count} en la biblioteca",
  "app.noDescription": "(sin descripción)",
  "app.libraryEmpty": "Aún no se ha importado ninguna medición. Abra un disco e «Importar nuevas».",
  // detalle / editor
  "app.description": "Descripción",
  "app.descPlaceholder": "Cliente, vehículo, notas…",
  "app.save": "Guardar",
  "app.deleteFromLibrary": "Eliminar de la biblioteca",
  "app.loading": "Cargando…",
  "app.selectRun": "Seleccione una medición para ver sus curvas.",
  // avisos
  "toast.nothingNew": "No hay nada nuevo que importar: todas las mediciones ya están en la biblioteca.",
  "toast.imported": "Se importaron {added} medición(es) nueva(s); se omitieron {skipped}.",
  "toast.importedOver": "Se importaron {added}, se sobrescribieron {overwritten}.",
  "toast.wiped": "Se borraron {deleted} medición(es). Copia de seguridad guardada en {path}",
  "toast.descSaved": "Descripción guardada.",
  // diálogos
  "dialog.importAllMsg":
    "¿Reimportar TODAS las {count} mediciones y SOBRESCRIBIR las copias que ya estén en la biblioteca?",
  "dialog.importAllTitle": "Importar todas / sobrescribir",
  "dialog.resetMsg":
    "¿Eliminar TODAS las mediciones de banco de esta imagen de disco?\n\nSe conservan los ajustes y la calibración (FLA.CFG), las tablas de idiomas y las fuentes. Primero se guarda una copia de seguridad de la imagen con marca de tiempo.\n\n{path}",
  "dialog.resetTitle": "Restablecer disco",
  "dialog.deleteMsg": "¿Quitar esta medición de su biblioteca? (No se modifica el disco de origen.)",
  "dialog.deleteTitle": "Eliminar medición",
  // panel de ajustes
  "settings.title": "Ajustes",
  "settings.language": "Idioma",
  "settings.units": "Unidades",
  "settings.unitMetric": "Métrico (kW, Nm, °C)",
  "settings.unitImperial": "Imperial (bhp, lb·ft, °F)",
  "settings.revealFolder": "Abrir carpeta de datos",
  "settings.close": "Cerrar",
  // términos completos del banco (Bosch)
  "term.engine": "Potencia motor",
  "term.torque": "Momento de par",
  "term.wheel": "Potencia rueda",
  "term.loss": "Pot. perdia rueda",
  // abreviaturas del banco (Bosch)
  "abbr.pmax": "Pmáx",
  "abbr.pnim": "Pnom",
  "abbr.ppyora": "Prued",
  "abbr.phavio": "Pperd",
  "abbr.mmax": "Mmáx",
  "abbr.paine": "Pres.",
  "abbr.lamp": "Temp.",
  "abbr.k": "k",
  // detalle / gráfico
  "detail.atN": "@ n",
  "detail.noDate": "sin fecha",
  "detail.noPower": "No hay curva de potencia en esta medición.",
  "detail.noTorque": "No hay curva de par en esta medición.",
  "chart.sweep": "barrido →",
  "chart.versus": "frente a",
  // impresión
  "print.reportTitle": "Informe de banco de potencia",
  "print.runDate": "Fecha de medición",
  "print.generatedBy": "Generado por FLA Dynoview",
  "crop.trim": "Recortar",
  "crop.reset": "Restablecer",
  "app.runDate": "Fecha",
  "app.dateNow": "Ahora",
};

export default es;
