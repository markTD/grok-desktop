import { invoke } from "@tauri-apps/api/core";

export type ModelInfo = {
  id: string;
  name: string;
  description: string | null;
  contextWindow: number | null;
  supportsReasoningEffort: boolean;
};

export type UpdateCheck = {
  currentVersion: string | null;
  latestVersion: string | null;
  updateAvailable: boolean;
  channel: string | null;
  autoUpdate: boolean | null;
  error: string | null;
};

export type BuildMonitor = {
  binaryPath: string | null;
  version: string | null;
  stableVersion: string | null;
  checkedAt: string | null;
  ready: boolean;
  authenticated: boolean;
  message: string;
  update: UpdateCheck;
  models: ModelInfo[];
  changelogPreview: string[];
  grokHome: string | null;
};

export type ProjectInfo = {
  cwd: string;
  isGit: boolean;
  gitRoot: string | null;
  branch: string | null;
  sessionsGroup: string | null;
  sessionsGroupPath: string | null;
  sessionCount: number;
};

export type SessionPaths = {
  sessionId: string;
  sessionDir: string | null;
  planMd: string | null;
  summaryJson: string | null;
  updatesJsonl: string | null;
};

export async function fetchBuildMonitor(): Promise<BuildMonitor> {
  return invoke<BuildMonitor>("build_monitor_report");
}

export async function runGrokUpdate(): Promise<string> {
  return invoke<string>("grok_run_update");
}

export async function fetchProjectInfo(cwd: string): Promise<ProjectInfo> {
  return invoke<ProjectInfo>("project_info", { cwd });
}

export async function fetchSessionPaths(
  cwd: string,
  sessionId: string,
): Promise<SessionPaths> {
  return invoke<SessionPaths>("session_paths", { cwd, sessionId });
}
