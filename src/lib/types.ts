/** Status returned by the Rust `grok_status` command. */
export type GrokStatus = {
  binaryPath: string | null;
  version: string | null;
  authenticated: boolean;
  message: string;
  ready: boolean;
};
