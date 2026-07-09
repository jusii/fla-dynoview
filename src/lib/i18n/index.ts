// Tiny reactive i18n: `t(key, params?)` resolves against the current locale
// (from the settings store) and falls back to English. Because `t()` reads the
// reactive `lang()`, any component that calls it re-renders when the language
// changes.
import { lang } from "../settings.svelte";
import en from "./locales/en";
import fi from "./locales/fi";
import de from "./locales/de";
import fr from "./locales/fr";
import it from "./locales/it";
import es from "./locales/es";
import pt from "./locales/pt";
import nl from "./locales/nl";
import da from "./locales/da";
import ja from "./locales/ja";

/** The message shape — every locale must provide exactly these keys (any string value). */
export type Messages = Record<keyof typeof en, string>;

export type Locale = "en" | "fi" | "de" | "fr" | "it" | "es" | "pt" | "nl" | "da" | "ja";

const catalogs: Record<Locale, Messages> = { en, fi, de, fr, it, es, pt, nl, da, ja };

/** Languages offered in the settings selector (native names). */
export const LOCALES: { code: Locale; name: string }[] = [
  { code: "de", name: "Deutsch" },
  { code: "en", name: "English" },
  { code: "es", name: "Español" },
  { code: "fr", name: "Français" },
  { code: "it", name: "Italiano" },
  { code: "nl", name: "Nederlands" },
  { code: "pt", name: "Português" },
  { code: "fi", name: "Suomi" },
  { code: "da", name: "Dansk" },
  { code: "ja", name: "日本語" },
];

export function t(key: keyof Messages, params?: Record<string, string | number>): string {
  const cat = catalogs[lang()] ?? en;
  let s: string = cat[key] ?? en[key] ?? (key as string);
  if (params) {
    for (const [k, v] of Object.entries(params)) {
      s = s.split(`{${k}}`).join(String(v));
    }
  }
  return s;
}
