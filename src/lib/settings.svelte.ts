// Reactive app settings (language, unit system, field-label overrides, curve
// visibility, print logo/header), backed by the persisted settings.json via the
// Tauri commands. This is a `.svelte.ts` module so it can hold `$state` shared
// across components.
import { getSettings, setSettings } from "./api";
import type { Locale } from "./i18n";
import type { UnitSystem } from "./units";
import type { CurveVisibility } from "./types";

let language = $state<Locale>("en");
let unitSystem = $state<UnitSystem>("metric");
let labels = $state<Record<string, string>>({});
let curves = $state<CurveVisibility>({ engine: true, wheel: false, loss: false, torque: true });
let logo = $state<string | null>(null);
let headerText = $state<string | null>(null);

export function lang(): Locale {
  return language;
}
export function unitSys(): UnitSystem {
  return unitSystem;
}
export function labelOverrides(): Record<string, string> {
  return labels;
}
export function curveVisibility(): CurveVisibility {
  return curves;
}
export function logoDataUri(): string | null {
  return logo;
}
export function printHeaderText(): string | null {
  return headerText;
}

/** Load persisted settings on startup (falls back to defaults). */
export async function initSettings(): Promise<void> {
  try {
    const s = await getSettings();
    if (s.language) language = s.language as Locale;
    if (s.unitSystem === "imperial" || s.unitSystem === "metric") unitSystem = s.unitSystem;
    if (s.labelOverrides) labels = s.labelOverrides;
    if (s.curveVisibility) curves = s.curveVisibility;
    logo = s.logoDataUri ?? null;
    headerText = s.printHeaderText ?? null;
  } catch {
    // keep defaults
  }
}

async function persist(): Promise<void> {
  try {
    await setSettings({
      language,
      unitSystem,
      labelOverrides: labels,
      curveVisibility: curves,
      logoDataUri: logo,
      printHeaderText: headerText,
    });
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

/** Set (or clear, when `value` is empty) the override for one label key. */
export async function setLabelOverride(key: string, value: string): Promise<void> {
  const next = { ...labels };
  if (value.trim()) next[key] = value;
  else delete next[key];
  labels = next;
  await persist();
}
export async function resetLabelOverrides(): Promise<void> {
  labels = {};
  await persist();
}

export async function setCurveVisibility(v: CurveVisibility): Promise<void> {
  curves = v;
  await persist();
}

export async function setLogo(dataUri: string | null): Promise<void> {
  logo = dataUri;
  await persist();
}
export async function setPrintHeaderText(text: string | null): Promise<void> {
  headerText = text && text.trim() ? text : null;
  await persist();
}
