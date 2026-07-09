// Reactive app settings (language + unit system), backed by the persisted
// settings.json via the Tauri commands. This is a `.svelte.ts` module so it can
// hold `$state` shared across components.
import { getSettings, setSettings } from "./api";
import type { Locale } from "./i18n";
import type { UnitSystem } from "./units";

let language = $state<Locale>("en");
let unitSystem = $state<UnitSystem>("metric");

export function lang(): Locale {
  return language;
}
export function unitSys(): UnitSystem {
  return unitSystem;
}

/** Load persisted settings on startup (falls back to defaults). */
export async function initSettings(): Promise<void> {
  try {
    const s = await getSettings();
    if (s.language) language = s.language as Locale;
    if (s.unitSystem === "imperial" || s.unitSystem === "metric") unitSystem = s.unitSystem;
  } catch {
    // keep defaults
  }
}

async function persist(): Promise<void> {
  try {
    await setSettings({ language, unitSystem });
  } catch {
    // non-fatal
  }
}

export async function setLanguage(l: Locale): Promise<void> {
  language = l;
  await persist();
}
export async function setUnitSystem(u: UnitSystem): Promise<void> {
  unitSystem = u;
  await persist();
}
