export interface Series {
  values: number[];
  color: string;
  label: string;
  /** Which Y axis this series is scaled against. Defaults to "left". */
  axis?: "left" | "right";
  /** Per-series rpm (X) values. When absent, the chart's shared `rpm` is used.
   *  Used by the comparison overlay where each run has its own rpm sweep. */
  rpm?: number[];
  /** Draw dashed (e.g. torque curves in the comparison overlay). */
  dash?: boolean;
}
