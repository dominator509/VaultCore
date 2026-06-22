import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";

test("primary flows have no serious or critical axe violations", async ({
  page,
}) => {
  await page.goto("/");
  await page.getByRole("button", { name: "Unlock with passphrase" }).click();
  await page.getByRole("button", { name: "Audit View" }).click();
  await page.getByRole("button", { name: "Vault Health" }).click();

  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa"])
    .analyze();
  const serious = results.violations.filter((violation) =>
    ["serious", "critical"].includes(violation.impact ?? ""),
  );

  expect(serious).toEqual([]);
});
