// Frontend mirror of the Rust DTOs in src-tauri/src/model.rs.
// (These are hand-written for now; a later pass generates them with ts-rs.)

export interface RunEntry {
  name: string;
  path: string;
  size: number;
  deleted: boolean;
  sha256: string;
  date: string | null;
  inLibrary: boolean;
}

export interface ImageSummary {
  imagePath: string;
  shopName: string;
  runs: RunEntry[];
}

export interface ShopInfo {
  name: string;
}

export interface ChannelsDto {
  ch0: number[];
  ch1: number[];
  ch2: number[];
  ch3: number[];
}

export interface ResultsDto {
  pnimKw: number | null;
  pressureHpa: number | null;
  tempC: number | null;
  kDin: number | null;
  pmaxKw: number | null;
  rpmAtPmax: number | null;
  ppyoraKw: number | null;
  phavioKw: number | null;
  mmaxNm: number | null;
  rpmAtMmax: number | null;
  trailerScan: [number, number][];
}

export interface DecodedRun {
  size: number;
  numChannels: number;
  date: string | null;
  sha256: string;
  channels: ChannelsDto;
  results: ResultsDto;
}

export interface Paths {
  root: string;
  dbDir: string;
  backupsDir: string;
}

export interface CurveVisibility {
  engine: boolean;
  wheel: boolean;
  loss: boolean;
  torque: boolean;
}

export interface Settings {
  language: string;
  unitSystem: "metric" | "imperial";
  labelOverrides: Record<string, string>;
  curveVisibility: CurveVisibility;
  logoDataUri: string | null;
  printHeaderText: string | null;
}

/// Per-run display overrides for editable readings (display-only).
export interface ValueOverrides {
  tempC: number | null;
  pressureHpa: number | null;
}

export interface RunRecord {
  schemaVersion: number;
  id: string;
  sha256: string;
  sourceImage: string | null;
  sourceEntry: string | null;
  wasDeletedEntry: boolean;
  shopName: string | null;
  runDate: string | null;
  importedAt: string;
  description: string;
  valueOverrides: ValueOverrides;
  results: ResultsDto;
  channels: ChannelsDto;
}

export interface RunIndexEntry {
  id: string;
  sha256: string;
  runDate: string | null;
  sourceImage: string | null;
  pnimKw: number | null;
  description: string;
  importedAt: string;
  path: string;
}

export interface ImportReport {
  added: string[];
  skipped: string[];
  overwritten: string[];
  failed: string[];
}

export interface ResetReport {
  backupPath: string;
  deleted: string[];
}

/// One run loaded into the comparison overlay.
export interface CompareRun {
  id: string;
  title: string;
  date: string | null;
  channels: ChannelsDto;
  kDin: number | null;
}

/// A run currently being viewed, from either an image or the library.
export interface CurrentRun {
  title: string;
  date: string | null;
  description: string;
  results: ResultsDto;
  channels: ChannelsDto;
  /// Shop/owner name (from the source disk), for the toolbar + printout.
  shopName: string | null;
  /// Per-run display overrides for editable readings (display-only).
  overrides: ValueOverrides;
  /// Library record id when this run is saved in the library, else null.
  libId: string | null;
}
