import { expect, test } from "@playwright/test";

test("vault health shows local health and alert states", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("button", { name: "Unlock with passphrase" }).click();
  await page.getByRole("button", { name: "Vault Health" }).click();

  await expect(
    page.getByRole("heading", { name: "Vault Health" }),
  ).toBeVisible();
  await expect(page.getByText("SpecAnchor verified")).toBeVisible();
  await expect(page.getByText("Audit head")).toBeVisible();
  await expect(page.getByText("genesis")).toBeVisible();
  await expect(page.getByText("Active session")).toBeVisible();
  await expect(page.getByText("local-session")).toBeVisible();
  await expect(page.getByText("IPC signature failures clear")).toBeVisible();
  await expect(page.getByText("Authorization denials clear")).toBeVisible();
});
