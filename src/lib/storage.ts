const ONBOARD_KEY = "grok-desktop.onboarding-done";
const CWD_KEY = "grok-desktop.last-cwd";

export function isOnboardingDone(): boolean {
  return localStorage.getItem(ONBOARD_KEY) === "1";
}

export function setOnboardingDone() {
  localStorage.setItem(ONBOARD_KEY, "1");
}

export function getLastCwd(fallback: string): string {
  return localStorage.getItem(CWD_KEY) || fallback;
}

export function setLastCwd(cwd: string) {
  localStorage.setItem(CWD_KEY, cwd);
}
