const ONBOARD_KEY = "grok-desktop.onboarding-done";
const CWD_KEY = "grok-desktop.last-cwd";
const SAFETY_KEY = "grok-desktop.safety-ack";
const EXPLAIN_KEY = "grok-desktop.explain-mode";

export function isOnboardingDone(): boolean {
  return localStorage.getItem(ONBOARD_KEY) === "1";
}

export function setOnboardingDone() {
  localStorage.setItem(ONBOARD_KEY, "1");
}

export function isSafetyAcked(): boolean {
  return localStorage.getItem(SAFETY_KEY) === "1";
}

export function setSafetyAcked() {
  localStorage.setItem(SAFETY_KEY, "1");
}

export function getExplainMode(): boolean {
  return localStorage.getItem(EXPLAIN_KEY) === "1";
}

export function setExplainMode(on: boolean) {
  localStorage.setItem(EXPLAIN_KEY, on ? "1" : "0");
}

export function getLastCwd(fallback = ""): string {
  const stored = localStorage.getItem(CWD_KEY);
  if (stored && stored.trim()) return stored;
  return fallback;
}

export function setLastCwd(cwd: string) {
  localStorage.setItem(CWD_KEY, cwd);
}
