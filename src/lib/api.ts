// Thin, typed wrappers around the Tauri command layer.
import { invoke } from "@tauri-apps/api/core";
import type {
  DecodedRun,
  ImageSummary,
  ImportReport,
  Paths,
  ResetReport,
  RunIndexEntry,
  RunRecord,
  Settings,
  ShopInfo,
} from "./types";

// --- reading images / runs ---
export const openImage = (path: string) =>
  invoke<ImageSummary>("open_image", { path });

export const readErgFromImage = (path: string, entryPath: string) =>
  invoke<DecodedRun>("read_erg_from_image", { path, entryPath });

export const openErgFile = (path: string) =>
  invoke<DecodedRun>("open_erg_file", { path });

export const getShopHeader = (path: string) =>
  invoke<ShopInfo>("get_shop_header", { path });

// --- library / import ---
export const importRuns = (
  imagePath: string,
  entryPaths: string[],
  overwrite: boolean,
) => invoke<ImportReport>("import_runs", { imagePath, entryPaths, overwrite });

export const importAll = (
  imagePath: string,
  overwrite: boolean,
  includeDeleted: boolean,
) => invoke<ImportReport>("import_all", { imagePath, overwrite, includeDeleted });

export const listDbRuns = (query?: string) =>
  invoke<RunIndexEntry[]>("list_db_runs", { query: query ?? null });

export const getDbRun = (id: string) => invoke<RunRecord>("get_db_run", { id });

export const updateRunDescription = (id: string, description: string) =>
  invoke<void>("update_run_description", { id, description });

export const deleteDbRun = (id: string) =>
  invoke<void>("delete_db_run", { id });

// --- reset ---
export const resetImage = (imagePath: string, confirm: boolean) =>
  invoke<ResetReport>("reset_image", { imagePath, confirm });

// --- misc ---
export const appPaths = () => invoke<Paths>("app_paths");

export const openDataFolder = () => invoke<void>("open_data_folder");

export const initialPath = () => invoke<string | null>("initial_path");

// --- settings ---
export const getSettings = () => invoke<Settings>("get_settings");

export const setSettings = (settings: Settings) =>
  invoke<void>("set_settings", { settings });
