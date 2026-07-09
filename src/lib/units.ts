// Unit-system conversion + labels. Metric is the stored/native form (the .ERG
// values are kW / Nm / °C / hPa); imperial converts on display.
import { unitSys } from "./settings.svelte";

export type UnitSystem = "metric" | "imperial";

const KW_TO_BHP = 1.34102;
const NM_TO_LBFT = 0.737562;

/** kW → bhp when imperial. */
export function power(kw: number): number {
  return unitSys() === "imperial" ? kw * KW_TO_BHP : kw;
}
/** Nm → lb·ft when imperial. */
export function torque(nm: number): number {
  return unitSys() === "imperial" ? nm * NM_TO_LBFT : nm;
}
/** °C → °F when imperial. */
export function temp(c: number): number {
  return unitSys() === "imperial" ? (c * 9) / 5 + 32 : c;
}
/** hPa and mbar are numerically identical; only the label changes. */
export function pressure(hpa: number): number {
  return hpa;
}

export const unitPower = () => (unitSys() === "imperial" ? "bhp" : "kW");
export const unitTorque = () => (unitSys() === "imperial" ? "lb·ft" : "Nm");
export const unitTemp = () => (unitSys() === "imperial" ? "°F" : "°C");
export const unitPressure = () => (unitSys() === "imperial" ? "mbar" : "hPa");
export const unitRpm = () => "rpm";
