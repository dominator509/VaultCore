import { expect, test } from "@playwright/test";

test("audit view and vault health surface chain status", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("button", { name: "Unlock with passphrase" }).click();
  await page.getByRole("button", { name: "Audit View" }).click();
  await expect(page.getByRole("heading", { name: "Audit View" })).toBeVisible();
  await page.getByRole("button", { name: "Verify chain" }).click();
  await expect(page.getByText("Audit head: valid")).toBeVisible();
  await page.getByRole("button", { name: "Vault Health" }).click();
  await expect(
    page.getByRole("heading", { name: "Vault Health" }),
  ).toBeVisible();
  await expect(page.getByText("SpecAnchor")).toBeVisible();
});
