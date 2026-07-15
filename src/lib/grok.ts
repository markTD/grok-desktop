import { invoke } from "@tauri-apps/api/core";
import type { GrokStatus } from "./types";

export async function fetchGrokStatus(): Promise<GrokStatus> {
  return invoke<GrokStatus>("grok_status");
}
