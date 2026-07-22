import type { ChannelsDto, CurveVisibility } from "../types";
import type { Series } from "./chart-types";
import { label } from "../i18n";
import { power as toPower, torque as toTorque, speed as toSpeed } from "../units";

// Curve colours.
export const POWER_GREEN = "#35d43a"; // engine power
export const WHEEL_BLUE = "#4db8ff"; // wheel power
export const LOSS_ORANGE = "#ff9d3a"; // wheel power-loss
export const TORQUE_RED = "#ff4d4d"; // torque

const ALL_CURVES: CurveVisibility = { engine: true, wheel: false, loss: false, torque: true };

const TORQUE_CONST = 9549.296; // 60000 / (2π)

/** Road speed (km/h) per sample = sampleIndex × this. The FLA samples the sweep
 *  at a fixed 0.5 km/h step (confirmed against the machine's on-screen axis). */
export const KMH_PER_SAMPLE = 0.5;

export interface Physical {
  /** True when the run carries an engine-rpm channel (rpm pickup was connected).
   *  When false the run is power-vs-road-speed only; torque is unavailable. */
  hasRpm: boolean;
  /** Engine rpm at each kept sample (0 when `!hasRpm`). */
  rpm: number[];
  /** Road speed (km/h) at each kept sample = original sample index × 0.5. */
  speedKmh: number[];
  wheelKw: number[];
  lossKw: number[];
  engineKw: number[];
  /** Torque (Nm); 0 when `!hasRpm` (torque needs rpm). */
  torqueNm: number[];
}

/// Derive the physically-calibrated curves (kW / Nm) from the raw channels and
/// DIN factor. Mirrors `core/src/erg.rs::compute_curves`, with two robustness
/// rules the raw math lacks:
///  - power/loss/engine do NOT require the rpm channel (many runs have no rpm),
///  - the wheel-power channel is sampled sparsely — gap samples are stored as 0,
///    interleaved with real readings. Plotting those 0s draws the curve as dense
///    vertical "noise", so we skip them (the native FLA software does the same).
export function physical(ch: ChannelsDto, kDin: number | null): Physical {
  const k = kDin ?? 1;
  const hasRpm = ch.ch3.length > 0;
  // Clamp to every channel we actually read. For rpm runs that includes ch3
  // (matching the Rust reference `compute_curves`), so an rpm channel trimmed
  // shorter than ch0/ch1 never emits rpm=0 tail samples that would snap the
  // rpm-axis curve back to x=0. For no-rpm runs ch3 is empty and deliberately
  // ignored — power/loss/engine don't need it.
  const m = hasRpm
    ? Math.min(ch.ch0.length, ch.ch1.length, ch.ch3.length)
    : Math.min(ch.ch0.length, ch.ch1.length);
  const rpm: number[] = [];
  const speedKmh: number[] = [];
  const wheelKw: number[] = [];
  const lossKw: number[] = [];
  const engineKw: number[] = [];
  const torqueNm: number[] = [];
  for (let i = 0; i < m; i++) {
    if (ch.ch0[i] === 0) continue; // power-gap sample → skip (not a real 0 kW point)
    const r = hasRpm ? (ch.ch3[i] ?? 0) : 0; // ?? 0 is belt-and-suspenders; m already clamps to ch3 when hasRpm

    const e = ((ch.ch0[i] - ch.ch1[i]) / 10) * k;
    rpm.push(r);
    speedKmh.push(i * KMH_PER_SAMPLE);
    wheelKw.push(ch.ch0[i] / 10);
    lossKw.push(ch.ch1[i] / 10);
    engineKw.push(e);
    torqueNm.push(hasRpm && r > 0 ? (e * TORQUE_CONST) / r : 0);
  }
  return { hasRpm, rpm, speedKmh, wheelKw, lossKw, engineKw, torqueNm };
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
  /** Road speed (km/h) at Pmax — shown instead of rpm for runs with no rpm. */
  vAtPmax: number | null;
  ppyoraKw: number | null;
  phavioKw: number | null;
  mmaxNm: number | null;
  rpmAtMmax: number | null;
}

export interface Views {
  /** Power curves (left axis) + torque (right axis), one combined list. */
  series: Series[];
  /** X-axis values per sample (engine rpm, or road speed when the run has no
   *  rpm), converted to the active unit system; same length as each series. */
  x: number[];
  /** True → X is engine rpm and torque is drawn; false → X is road speed. */
  hasRpm: boolean;
  scalars: CropScalars;
}

/// Detect the "pull": the final monotonic rpm climb up to peak rpm. Returns the
/// crop that isolates it, so the chart opens on the clean acceleration sweep
/// (the FLA's launch/idle region has non-monotonic rpm that plots messily).
/// Returns the full range if the run is already clean or too short.
export function autoCrop(ch: ChannelsDto, kDin: number | null): CropRange {
  const p = physical(ch, kDin);
  // Crop against whichever axis this run uses (rpm, or road speed when no rpm).
  const axis = p.hasRpm ? p.rpm : p.speedKmh;
  const n = axis.length;
  if (n < 6) return { start: 0, end: 1 };
  let peak = 0;
  for (let i = 1; i < n; i++) if (axis[i] > axis[peak]) peak = i;
  let start = peak;
  while (start > 0 && axis[start - 1] <= axis[start]) start--;
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
export function views(
  ch: ChannelsDto,
  kDin: number | null,
  crop?: CropRange,
  vis: CurveVisibility = ALL_CURVES,
): Views {
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
  const spd = p.speedKmh.slice(a, b);
  const tq = p.torqueNm.slice(a, b);
  // X axis: engine rpm when present, else road speed (converted to the unit system).
  const x = p.hasRpm ? rpm : spd.map(toSpeed);

  // Power curves on the left axis, torque on the right. Which curves are drawn is
  // controlled by `vis`; all channel data is computed regardless. Loss is plotted
  // as its magnitude (drag power), matching the positive Ploss scalar. Torque is
  // only meaningful — and only drawn — when the run carries rpm.
  const all: Series[] = [];
  if (vis.engine)
    all.push({ values: eng.map(toPower), color: POWER_GREEN, label: label("term.engine"), axis: "left" });
  if (vis.wheel)
    all.push({ values: whl.map(toPower), color: WHEEL_BLUE, label: label("term.wheel"), axis: "left" });
  if (vis.loss)
    all.push({ values: loss.map((l) => toPower(-l)), color: LOSS_ORANGE, label: label("term.loss"), axis: "left" });
  if (p.hasRpm && vis.torque)
    all.push({ values: tq.map(toTorque), color: TORQUE_RED, label: label("term.torque"), axis: "right" });
  const series = all.filter((s) => s.values.length);

  let scalars: CropScalars = {
    pmaxKw: null,
    rpmAtPmax: null,
    vAtPmax: null,
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
      rpmAtPmax: p.hasRpm ? rpm[pi] : null,
      vAtPmax: spd[pi] ?? null,
      ppyoraKw: Math.max(...whl),
      phavioKw: -loss[pi],
      mmaxNm: p.hasRpm ? tq[mi] : null,
      rpmAtMmax: p.hasRpm ? rpm[mi] : null,
    };
  }
  return { series, x, hasRpm: p.hasRpm, scalars };
}
