import type { ChannelsDto } from "../types";
import type { Series } from "./chart-types";

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

/// Derive the physically-calibrated curves from the raw channels and DIN factor.
/// Mirrors `core/src/erg.rs::compute_curves`.
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

export function powerSeries(ch: ChannelsDto, kDin: number | null): Series[] {
  const p = physical(ch, kDin);
  return [
    { values: p.engineKw, color: MAGENTA, label: "Moottoriteho (engine)" },
    { values: p.wheelKw, color: CYAN, label: "Pyöräteho (wheel)" },
    { values: p.lossKw, color: GREEN, label: "Häviöteho (loss)" },
  ].filter((s) => s.values.length);
}

export function torqueSeries(ch: ChannelsDto, kDin: number | null): Series[] {
  const p = physical(ch, kDin);
  return [
    { values: p.torqueNm, color: ORANGE, label: "Vääntömomentti (torque)" },
  ].filter((s) => s.values.length);
}
