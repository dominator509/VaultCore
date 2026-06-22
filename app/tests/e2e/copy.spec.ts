import { expect, test } from "@playwright/test";

test("copy starts an auto-clear countdown", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("button", { name: "Unlock with passphrase" }).click();
  await page.getByRole("button", { name: "Copy" }).first().click();
  await expect(page.getByText("Copy: 20 seconds remaining")).toBeVisible();
});
