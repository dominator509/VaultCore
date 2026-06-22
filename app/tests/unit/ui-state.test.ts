import { describe, expect, it } from "vitest";
import { errorText } from "../../src/lib/errors";
import { nextCountdown, ttlToSeconds } from "../../src/lib/timers";
import { visibleSecrets } from "../../src/state/appState";

describe("UI state helpers", () => {
  it("filters metadata-only secret summaries", () => {
    expect(
      visibleSecrets(
        [
          {
            id: "1",
            name: "Payroll admin",
            secret_type: "LOGIN",
            state: "active",
          },
          {
            id: "2",
            name: "Deploy token",
            secret_type: "API_KEY",
            state: "active",
          },
        ],
        "deploy",
      ),
    ).toHaveLength(1);
  });

  it("counts down without going negative", () => {
    expect(nextCountdown(2)).toBe(1);
    expect(nextCountdown(0)).toBe(0);
  });

  it("maps TTL milliseconds to visible seconds", () => {
    expect(ttlToSeconds(30000)).toBe(30);
    expect(ttlToSeconds(1)).toBe(1);
  });

  it("renders typed VaultError messages with stable codes", () => {
    expect(
      errorText({
        code: "VC-AUTHZ-001",
        category: "Authorization",
        recoverable: false,
        field: "role",
        message: "role is not authorized",
      }),
    ).toContain("VC-AUTHZ-001");
  });
});
