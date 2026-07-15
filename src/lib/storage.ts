const ONBOARD_KEY = "grok-desktop.onboarding-done";
const CWD_KEY = "grok-desktop.last-cwd";
const SAFETY_KEY = "grok-desktop.safety-ack";

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

export function getLastCwd(fallback: string): string {
  return localStorage.getItem(CWD_KEY) || fallback;
}

export function setLastCwd(cwd: string) {
  localStorage.setItem(CWD_KEY, cwd);
}
