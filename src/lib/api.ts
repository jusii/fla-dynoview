// Thin, typed wrappers around the Tauri command layer.
import { invoke } from "@tauri-apps/api/core";
import type { DecodedRun, ImageSummary, Paths, ShopInfo } from "./types";

export const openImage = (path: string) =>
  invoke<ImageSummary>("open_image", { path });

export const readErgFromImage = (path: string, entryPath: string) =>
  invoke<DecodedRun>("read_erg_from_image", { path, entryPath });

export const openErgFile = (path: string) =>
  invoke<DecodedRun>("open_erg_file", { path });

export const getShopHeader = (path: string) =>
  invoke<ShopInfo>("get_shop_header", { path });

export const appPaths = () => invoke<Paths>("app_paths");

export const initialPath = () => invoke<string | null>("initial_path");
