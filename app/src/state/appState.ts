import type { Role, SecretSummary } from "../lib/tauri";

export type FlowState =
  | "idle"
  | "loading"
  | "empty"
  | "success"
  | "error"
  | "denied";

export type VaultSession = {
  sessionId: string;
  role: Role;
};

export type View = "list" | "create" | "audit" | "health" | "settings";

export type AppModel = {
  flow: FlowState;
  session: VaultSession | null;
  view: View;
  secrets: SecretSummary[];
};

export const initialModel: AppModel = {
  flow: "idle",
  session: null,
  view: "list",
  secrets: [],
};

export function visibleSecrets(
  secrets: SecretSummary[],
  query: string,
): SecretSummary[] {
  const normalized = query.trim().toLowerCase();
  if (normalized.length === 0) {
    return secrets;
  }
  return secrets.filter((secret) =>
    secret.name.toLowerCase().includes(normalized),
  );
}
