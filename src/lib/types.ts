// Frontend mirror of the Rust DTOs in src-tauri/src/model.rs.
// (These are hand-written for now; a later pass generates them with ts-rs.)

export interface RunEntry {
  name: string;
  path: string;
  size: number;
  deleted: boolean;
  sha256: string;
  date: string | null;
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
  rpmRaw: number | null;
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
  dbDir: string;
  backupsDir: string;
}
