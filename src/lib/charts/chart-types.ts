export interface Series {
  values: number[];
  color: string;
  label: string;
  /** Which Y axis this series is scaled against. Defaults to "left". */
  axis?: "left" | "right";
}
