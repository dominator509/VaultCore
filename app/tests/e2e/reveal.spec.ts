import { expect, test } from "@playwright/test";

test("search and reveal show a payload handle with auto-clear countdown", async ({
  page,
}) => {
  await page.goto("/");
  await page.getByRole("button", { name: "Unlock with passphrase" }).click();
  await page.getByLabel("Search metadata").fill("Payroll");
  await expect(
    page.getByRole("article", { name: "Payroll admin" }),
  ).toBeVisible();
  await page.getByLabel("Reveal reason").first().fill("Operational review");
  await page.getByRole("button", { name: "Reveal" }).first().click();
  await expect(page.getByTestId("payload-window")).toContainText(
    "payload://local-1",
  );
  await expect(page.getByTestId("payload-window")).toContainText(
    "seconds remaining",
  );
});
