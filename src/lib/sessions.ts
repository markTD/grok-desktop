import type { RecentSession } from "./types";

const KEY = "grok-desktop.recent-sessions";
const MAX = 12;

export function loadRecentSessions(): RecentSession[] {
  try {
    const raw = localStorage.getItem(KEY);
    if (!raw) return [];
    const parsed = JSON.parse(raw) as RecentSession[];
    return Array.isArray(parsed) ? parsed : [];
  } catch {
    return [];
  }
}

export function saveRecentSession(entry: RecentSession) {
  const list = loadRecentSessions().filter((s) => s.sessionId !== entry.sessionId);
  list.unshift(entry);
  localStorage.setItem(KEY, JSON.stringify(list.slice(0, MAX)));
}

export function clearRecentSessions() {
  localStorage.removeItem(KEY);
}
