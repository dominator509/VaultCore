import strings from "../i18n/en.json" with { type: "json" };

export type VaultError = {
  code: string;
  category: string;
  recoverable: boolean;
  field?: string | null;
  message: string;
};

export function isVaultError(value: unknown): value is VaultError {
  return (
    typeof value === "object" &&
    value !== null &&
    "code" in value &&
    "message" in value
  );
}

export function errorText(error: unknown): string {
  if (isVaultError(error)) {
    return `${strings.code} ${error.code}: ${error.message}`;
  }
  return `${strings.code} VC-INTERNAL-001: ${strings.error}`;
}
