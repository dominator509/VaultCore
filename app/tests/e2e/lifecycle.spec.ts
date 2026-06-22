import { expect, test } from "@playwright/test";

test("create edit rotate soft-delete and purge lifecycle", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("button", { name: "Unlock with passphrase" }).click();
  await page.getByRole("button", { name: "Create" }).click();
  await page.getByLabel("Name").fill("New test note");
  await page.getByLabel("Payload handle").fill("payload://test-note");
  await page.locator("form").getByRole("button", { name: "Create" }).click();
  await page.getByRole("button", { name: "List/Search" }).click();
  await page.getByLabel("Search metadata").fill("Payroll");
  await page.getByLabel("Name").first().fill("Payroll admin edited");
  await page.getByRole("button", { name: "Update" }).first().click();
  await expect(
    page.getByRole("article", { name: "Payroll admin edited" }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Rotate" }).first().click();
  await page.getByRole("button", { name: "Soft delete" }).first().click();
  await page.getByLabel("Confirmation token").first().fill("PURGE");
  await page.getByRole("button", { name: "Purge" }).first().click();
  await expect(
    page.getByRole("article", { name: "Payroll admin edited" }),
  ).toHaveCount(0);
});
