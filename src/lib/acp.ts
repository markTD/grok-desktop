import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  AcpStatusEvent,
  AcpUpdateEvent,
  ConnectOptions,
  ConnectResult,
  ConnectionInfo,
  PermissionEvent,
  PermissionReply,
  PromptResult,
} from "./types";

export async function acpConnect(options: ConnectOptions): Promise<ConnectResult> {
  return invoke<ConnectResult>("acp_connect", { options });
}

export async function acpPrompt(text: string): Promise<PromptResult> {
  return invoke<PromptResult>("acp_prompt", { text });
}

export async function acpRespondPermission(reply: PermissionReply): Promise<void> {
  return invoke("acp_respond_permission", { reply });
}

export async function acpDisconnect(): Promise<void> {
  return invoke("acp_disconnect");
}

export async function acpConnection(): Promise<ConnectionInfo> {
  return invoke<ConnectionInfo>("acp_connection");
}

export function onAcpUpdate(
  handler: (event: AcpUpdateEvent) => void,
): Promise<UnlistenFn> {
  return listen<AcpUpdateEvent>("acp://update", (e) => handler(e.payload));
}

export function onAcpStatus(
  handler: (event: AcpStatusEvent) => void,
): Promise<UnlistenFn> {
  return listen<AcpStatusEvent>("acp://status", (e) => handler(e.payload));
}

export function onAcpError(handler: (message: string) => void): Promise<UnlistenFn> {
  return listen<string>("acp://error", (e) => handler(e.payload));
}

export function onAcpLog(handler: (message: string) => void): Promise<UnlistenFn> {
  return listen<string>("acp://log", (e) => handler(e.payload));
}

export function onAcpPermission(
  handler: (event: PermissionEvent) => void,
): Promise<UnlistenFn> {
  return listen<PermissionEvent>("acp://permission", (e) => handler(e.payload));
}

/** Pull text out of an ACP content block if present. */
export function contentText(update: Record<string, unknown>): string {
  const content = update.content;
  if (content && typeof content === "object" && content !== null) {
    const t = (content as { text?: unknown }).text;
    if (typeof t === "string") return t;
  }
  if (typeof update.text === "string") return update.text;
  return "";
}
