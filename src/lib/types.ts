/** Status returned by the Rust `grok_status` command. */
export type GrokStatus = {
  binaryPath: string | null;
  version: string | null;
  authenticated: boolean;
  message: string;
  ready: boolean;
};

export type ConnectOptions = {
  cwd: string;
  alwaysApprove?: boolean;
  resumeSessionId?: string | null;
  rules?: string | null;
  model?: string | null;
  effort?: string | null;
};

export type ConnectResult = {
  sessionId: string;
  cwd: string;
  modelId: string | null;
  binaryPath: string;
  resumed: boolean;
  alwaysApprove: boolean;
};

export type PromptResult = {
  stopReason: string | null;
  meta: Record<string, unknown> | null;
};

export type ConnectionInfo = {
  connected: boolean;
  sessionId: string | null;
  cwd: string | null;
  alwaysApprove: boolean;
};

export type AcpUpdateEvent = {
  kind: string;
  update: Record<string, unknown>;
  sessionId: string;
};

export type AcpStatusEvent = {
  connected: boolean;
  sessionId: string | null;
  cwd: string | null;
  message: string;
};

export type PermissionOption = {
  optionId?: string;
  id?: string;
  name?: string;
  label?: string;
  kind?: string;
  [key: string]: unknown;
};

export type PermissionEvent = {
  requestId: number;
  toolCall: Record<string, unknown> | null;
  options: PermissionOption[];
  raw: Record<string, unknown>;
};

export type PermissionReply = {
  requestId: number;
  outcome: "selected" | "cancelled";
  optionId?: string | null;
};

/** One row in the chat transcript. */
export type ChatItem =
  | { id: string; role: "user"; text: string }
  | { id: string; role: "assistant"; text: string }
  | { id: string; role: "thought"; text: string }
  | {
      id: string;
      role: "tool";
      title: string;
      status: string;
      detail?: string;
      toolCallId?: string;
    }
  | { id: string; role: "system"; text: string }
  | { id: string; role: "loop"; text: string };

export type RecentSession = {
  sessionId: string;
  cwd: string;
  modelId?: string | null;
  label?: string;
  updatedAt: number;
};

export type KickoffAnswers = {
  goal: string;
  projectKind: "existing" | "greenfield" | "learn" | "fix";
  risk: "careful" | "balanced" | "fast";
  planFirst: boolean;
  extra: string;
};

export type KickoffPlan = {
  starterPrompt: string;
  rules: string;
  alwaysApprove: boolean;
  summary: string;
};
