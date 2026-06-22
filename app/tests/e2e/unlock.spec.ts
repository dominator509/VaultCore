import { expect, test } from "@playwright/test";

test("unlock flow reaches the workspace and settings stub", async ({
  page,
}) => {
  await page.goto("/");
  await expect(page.getByRole("heading", { name: "Locked" })).toBeVisible();
  await page.getByLabel("Passphrase").fill("local-proof");
  await page.getByRole("button", { name: "Unlock with passphrase" }).click();
  await expect(
    page.getByRole("heading", { name: "Session active" }),
  ).toBeVisible();
  await page.getByRole("button", { name: "Settings" }).click();
  await expect(page.getByRole("heading", { name: "Settings" })).toBeVisible();
});
