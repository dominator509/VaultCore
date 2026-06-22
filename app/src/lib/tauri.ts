import { invoke } from "@tauri-apps/api/core";

export type Role = "Owner" | "Admin" | "Editor" | "Viewer" | "Auditor";
export type SecretType =
  | "API_KEY"
  | "LOGIN"
  | "OAUTH_APP"
  | "SSH_KEY"
  | "WALLET_KEY"
  | "CERT"
  | "NOTE"
  | "BLOB";
export type LifecycleState =
  | "draft"
  | "active"
  | "rotating"
  | "archived"
  | "soft_deleted"
  | "purged";

export type SessionToken = { session_id: string };
export type Ack = { ok: boolean };
export type SecretSummary = {
  id: string;
  secret_type: SecretType;
  name: string;
  state: LifecycleState;
};
export type SecretListFilter = { role: Role; query?: string | null };
export type SecretInput = {
  role: Role;
  secret_type: SecretType;
  name: string;
  payload_handle: string;
};
export type SecretPatch = { role: Role; name?: string | null };
export type RevealResponse = { ttl_ms: number; payload_handle: string };
export type AuditFilter = { role: Role };
export type AuditViewEntry = {
  op: string;
  target_id?: string | null;
  result: string;
};
export type ChainStatus = { valid: boolean };

const seededSecrets: SecretSummary[] = [
  {
    id: "local-1",
    secret_type: "LOGIN",
    name: "Payroll admin",
    state: "active",
  },
  {
    id: "local-2",
    secret_type: "API_KEY",
    name: "Deploy token",
    state: "active",
  },
  {
    id: "local-3",
    secret_type: "NOTE",
    name: "Breakglass note",
    state: "archived",
  },
];

let mockSecrets = [...seededSecrets];
let nextId = 4;

const hasTauriRuntime = (): boolean => "__TAURI_INTERNALS__" in window;

async function call<T>(
  command: string,
  args: Record<string, unknown>,
): Promise<T> {
  if (hasTauriRuntime()) {
    return invoke<T>(command, args);
  }
  return mockInvoke<T>(command, args);
}

export const vaultApi = {
  unlock(method: string, proof: string): Promise<SessionToken> {
    return call("unlock", { method, proof });
  },
  list(filter: SecretListFilter): Promise<SecretSummary[]> {
    return call("list", { filter });
  },
  reveal(
    secretId: string,
    reason: string,
    role: Role,
  ): Promise<RevealResponse> {
    return call("reveal", { secretId, reason, role });
  },
  copy(secretId: string, ttlMs: number, role: Role): Promise<Ack> {
    return call("copy", { secretId, ttlMs, role });
  },
  create(secretInput: SecretInput): Promise<SecretSummary> {
    return call("create", { secretInput });
  },
  update(secretId: string, patch: SecretPatch): Promise<SecretSummary> {
    return call("update", { secretId, patch });
  },
  rotate(
    secretId: string,
    newPayloadHandle: string,
    role: Role,
  ): Promise<SecretSummary> {
    return call("rotate", { secretId, newPayloadHandle, role });
  },
  softDelete(secretId: string, role: Role): Promise<Ack> {
    return call("soft_delete", { secretId, role });
  },
  purge(secretId: string, confirmationToken: string, role: Role): Promise<Ack> {
    return call("purge", { secretId, confirmationToken, role });
  },
  auditView(filter: AuditFilter): Promise<AuditViewEntry[]> {
    return call("audit_view", { filter });
  },
  verifyAuditChain(): Promise<ChainStatus> {
    return call("verify_audit_chain", {});
  },
  lock(): Promise<Ack> {
    return call("lock", {});
  },
};

async function mockInvoke<T>(
  command: string,
  args: Record<string, unknown>,
): Promise<T> {
  await new Promise((resolve) => window.setTimeout(resolve, 8));
  switch (command) {
    case "unlock": {
      const proof = String(args.proof ?? "");
      if (proof.length === 0) {
        throw vaultError(
          "VC-AUTH-001",
          "Auth",
          true,
          "proof",
          "Unlock proof is required",
        );
      }
      return { session_id: "local-session" } as T;
    }
    case "list": {
      const filter = args.filter as SecretListFilter;
      const query = filter.query?.toLowerCase() ?? "";
      return mockSecrets.filter((secret) =>
        secret.name.toLowerCase().includes(query),
      ) as T;
    }
    case "reveal": {
      if (String(args.reason ?? "").trim().length === 0) {
        throw vaultError(
          "VC-VAL-001",
          "Validation",
          true,
          "reason",
          "Reason is required",
        );
      }
      return {
        ttl_ms: 30000,
        payload_handle: `payload://${String(args.secretId)}`,
      } as T;
    }
    case "copy":
    case "soft_delete":
    case "purge":
    case "lock":
      return { ok: true } as T;
    case "create": {
      const input = args.secretInput as SecretInput;
      const created: SecretSummary = {
        id: `local-${nextId}`,
        secret_type: input.secret_type,
        name: input.name,
        state: "draft",
      };
      nextId += 1;
      mockSecrets = [created, ...mockSecrets];
      return created as T;
    }
    case "update": {
      const patch = args.patch as SecretPatch;
      const id = String(args.secretId);
      mockSecrets = mockSecrets.map((secret) =>
        secret.id === id
          ? { ...secret, name: patch.name ?? secret.name }
          : secret,
      );
      return mockSecrets.find((secret) => secret.id === id) as T;
    }
    case "rotate": {
      const id = String(args.secretId);
      mockSecrets = mockSecrets.map((secret) =>
        secret.id === id ? { ...secret, state: "active" } : secret,
      );
      return mockSecrets.find((secret) => secret.id === id) as T;
    }
    case "audit_view":
      return [
        { op: "unlock", target_id: null, result: "allowed" },
        { op: "reveal", target_id: "local-1", result: "allowed" },
      ] as T;
    case "verify_audit_chain":
      return { valid: true } as T;
    default:
      throw vaultError("VC-IPC-001", "Ipc", true, null, "Unknown command");
  }
}

function vaultError(
  code: string,
  category: string,
  recoverable: boolean,
  field: string | null,
  message: string,
) {
  return { code, category, recoverable, field, message };
}
