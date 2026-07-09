import type { ChannelsDto } from "../types";
import type { Series } from "./chart-types";

// Bosch-screen-inspired curve colours.
export const MAGENTA = "#d94fd9";
export const GREEN = "#35d43a";
export const ORANGE = "#ff7a4a";
export const CYAN = "#49c7ff";

export function powerSeries(ch: ChannelsDto): Series[] {
  return [
    { values: ch.ch0, color: MAGENTA, label: "Engine power (ch0)" },
    { values: ch.ch1, color: GREEN, label: "Wheel-loss (ch1)" },
  ].filter((s) => s.values.length);
}

export function torqueSeries(ch: ChannelsDto): Series[] {
  return [
    { values: ch.ch2, color: ORANGE, label: "Torque (ch2)" },
    { values: ch.ch3, color: CYAN, label: "RPM (ch3)" },
  ].filter((s) => s.values.length);
}
