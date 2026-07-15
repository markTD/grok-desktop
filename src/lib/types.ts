/** Status returned by the Rust `grok_status` command. */
export type GrokStatus = {
  binaryPath: string | null;
  version: string | null;
  authenticated: boolean;
  message: string;
  ready: boolean;
};

export type ConnectResult = {
  sessionId: string;
  cwd: string;
  modelId: string | null;
  binaryPath: string;
};

export type PromptResult = {
  stopReason: string | null;
  meta: Record<string, unknown> | null;
};

export type ConnectionInfo = {
  connected: boolean;
  sessionId: string | null;
  cwd: string | null;
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

/** One row in the chat transcript. */
export type ChatItem =
  | { id: string; role: "user"; text: string }
  | { id: string; role: "assistant"; text: string }
  | { id: string; role: "thought"; text: string }
  | { id: string; role: "tool"; title: string; status: string; detail?: string }
  | { id: string; role: "system"; text: string };
