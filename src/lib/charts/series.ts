import type { ChannelsDto } from "../types";
import type { Series } from "./chart-types";
import { t } from "../i18n";
import { power as toPower, torque as toTorque } from "../units";

// Bosch-screen-inspired curve colours.
export const MAGENTA = "#d94fd9";
export const GREEN = "#35d43a";
export const ORANGE = "#ff7a4a";
export const CYAN = "#49c7ff";

const TORQUE_CONST = 9549.296; // 60000 / (2π)

export interface Physical {
  rpm: number[];
  wheelKw: number[];
  lossKw: number[];
  engineKw: number[];
  torqueNm: number[];
}

/// Derive the physically-calibrated curves (kW / Nm) from the raw channels and
/// DIN factor. Mirrors `core/src/erg.rs::compute_curves`.
export function physical(ch: ChannelsDto, kDin: number | null): Physical {
  const k = kDin ?? 1;
  const n = Math.min(ch.ch0.length, ch.ch1.length, ch.ch3.length);
  const rpm: number[] = [];
  const wheelKw: number[] = [];
  const lossKw: number[] = [];
  const engineKw: number[] = [];
  const torqueNm: number[] = [];
  for (let i = 0; i < n; i++) {
    const w = ch.ch0[i] / 10;
    const l = ch.ch1[i] / 10;
    const r = ch.ch3[i];
    const e = ((ch.ch0[i] - ch.ch1[i]) / 10) * k;
    rpm.push(r);
    wheelKw.push(w);
    lossKw.push(l);
    engineKw.push(e);
    torqueNm.push(r > 0 ? (e * TORQUE_CONST) / r : 0);
  }
  return { rpm, wheelKw, lossKw, engineKw, torqueNm };
}

/// A trim window over the sweep, as fractions 0..1.
export interface CropRange {
  start: number;
  end: number;
}

/// Peak scalars (kW / Nm), computed over the visible (cropped) window.
export interface CropScalars {
  pmaxKw: number | null;
  rpmAtPmax: number | null;
  ppyoraKw: number | null;
  phavioKw: number | null;
  mmaxNm: number | null;
  rpmAtMmax: number | null;
}

export interface Views {
  /** Power curves (left axis) + torque (right axis), one combined list. */
  series: Series[];
  /** Engine rpm per sample, for the X-axis labels (same length as the series). */
  rpm: number[];
  scalars: CropScalars;
}

/// Detect the "pull": the final monotonic rpm climb up to peak rpm. Returns the
/// crop that isolates it, so the chart opens on the clean acceleration sweep
/// (the FLA's launch/idle region has non-monotonic rpm that plots messily).
/// Returns the full range if the run is already clean or too short.
export function autoCrop(ch: ChannelsDto, kDin: number | null): CropRange {
  const { rpm } = physical(ch, kDin);
  const n = rpm.length;
  if (n < 6) return { start: 0, end: 1 };
  let peak = 0;
  for (let i = 1; i < n; i++) if (rpm[i] > rpm[peak]) peak = i;
  let start = peak;
  while (start > 0 && rpm[start - 1] <= rpm[start]) start--;
  const end = Math.min(n, peak + 1);
  if (end - start < 3) return { start: 0, end: 1 };
  return { start: start / n, end: end / n };
}

function argmax(a: number[]): number {
  let bi = 0;
  let bv = -Infinity;
  for (let i = 0; i < a.length; i++) {
    if (a[i] > bv) {
      bv = a[i];
      bi = i;
    }
  }
  return bi;
}

/// Build the combined display series (power on the left axis, torque on the
/// right), the rpm axis, and recomputed peak scalars for the (optionally
/// cropped) run. Values are converted to the active unit system.
export function views(ch: ChannelsDto, kDin: number | null, crop?: CropRange): Views {
  const p = physical(ch, kDin);
  const n = p.engineKw.length;
  let a = 0;
  let b = n;
  if (crop) {
    a = Math.max(0, Math.floor(crop.start * n));
    b = Math.min(n, Math.ceil(crop.end * n));
    if (a >= b) {
      a = 0;
      b = n;
    }
  }
  const eng = p.engineKw.slice(a, b);
  const whl = p.wheelKw.slice(a, b);
  const loss = p.lossKw.slice(a, b);
  const rpm = p.rpm.slice(a, b);
  const tq = p.torqueNm.slice(a, b);

  // The classic dyno chart: two curves only — power (left) and torque (right).
  // Wheel power / loss are still surfaced as scalars in the info box.
  const all: Series[] = [
    { values: eng.map(toPower), color: MAGENTA, label: t("term.engine"), axis: "left" },
    { values: tq.map(toTorque), color: ORANGE, label: t("term.torque"), axis: "right" },
  ];
  const series = all.filter((s) => s.values.length);

  let scalars: CropScalars = {
    pmaxKw: null,
    rpmAtPmax: null,
    ppyoraKw: null,
    phavioKw: null,
    mmaxNm: null,
    rpmAtMmax: null,
  };
  if (eng.length) {
    const pi = argmax(eng);
    const mi = argmax(tq);
    scalars = {
      pmaxKw: eng[pi],
      rpmAtPmax: rpm[pi],
      ppyoraKw: Math.max(...whl),
      phavioKw: -loss[pi],
      mmaxNm: tq[mi],
      rpmAtMmax: rpm[mi],
    };
  }
  return { series, rpm, scalars };
}
