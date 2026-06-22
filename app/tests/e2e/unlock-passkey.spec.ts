import { expect, test } from "@playwright/test";

test("passkey unlock reaches the active session", async ({ page }) => {
  await page.goto("/");

  await page.getByRole("button", { name: "Use passkey" }).click();

  await expect(
    page.getByRole("heading", { name: "Session active" }),
  ).toBeVisible();
});
