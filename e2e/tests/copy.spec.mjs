import { test, expect } from "@playwright/test";
import { mkdir } from "node:fs/promises";
import { locales } from "../copy-fixtures.mjs";

test.beforeEach(async ({ page }) => {
  await page.route("https://fonts.googleapis.com/**", route => route.abort());
  await page.route("https://fonts.gstatic.com/**", route => route.abort());
  await page.route("https://cdn.jsdelivr.net/**", route => route.abort());
  const replacements = {
    "copy-punctuation": ["Softvér pre ", "Softvér — pre "],
    "copy-industries": ["Bankovníctvo", "Online Casino"],
    "copy-benefits": ['class="impact-value">Čas', 'class="impact-value">0'],
  };
  const replacement = replacements[process.env.DELIVERY_MUTATION];
  if (replacement) {
    await page.route(/\/$/, async route => {
      const response = await route.fetch();
      const body = await response.text();
      expect(body, "mutation target copy").toContain(replacement[0]);
      await route.fulfill({ response, body: body.replace(replacement[0], replacement[1]) });
    });
  }
});

for (const locale of locales) {
  test(`page copy uses plain sentences and concrete benefits in ${locale.lang}`, async ({ page }) => {
    await page.goto(locale.path);
    await expect(page.locator("body")).not.toContainText(/[—–]|\s-\s/);
    await expect(page.locator("h1")).toHaveText(locale.headline);
    for (const heading of locale.headings) {
      await expect(page.getByRole("heading", { name: heading, exact: true })).toHaveCount(1);
    }
    await expect(page.locator("#delivery-title")).toHaveText(locale.question);
    await expect(page.getByTestId("delivery-brief")).toHaveAccessibleName(locale.brief);
    await page.locator(".impact").scrollIntoViewIfNeeded();
    await expect(page.locator(".impact-value")).toHaveText(locale.benefits);
    await expect(page.locator(".impact [data-count]")).toHaveCount(0);
    expect((await page.locator(".impact, .use-case-result").allTextContents()).join(" ")).not.toMatch(/\d+\s*(?:%|×|x\b|min\b|h\b)/);
  });

  test(`industry list excludes casinos and retains the other sectors in ${locale.lang}`, async ({ page }) => {
    await page.goto(locale.path);
    await expect(page.locator("body")).not.toContainText(/casino|kasino/i);
    await expect(page.locator(".marquee-item")).toHaveText([...locale.domains, ...locale.domains].map(name => `${name}◆`));
  });

  test(`contact status copy explains accepted and invalid submissions in ${locale.lang}`, async ({ page }) => {
    for (const accepted of [true, false]) {
      await page.goto(locale.path);
      await page.locator(".contact-form").evaluate((form, accepted) => {
        form.noValidate = true;
        form.elements.website.value = accepted ? "example.test" : "";
      }, accepted);
      const [response] = await Promise.all([
        page.waitForResponse(response => new URL(response.url()).pathname === "/contact"),
        page.locator('.contact-form button[type="submit"]').click(),
      ]);
      expect(response.status()).toBe(accepted ? 200 : 400);
      await expect(page.locator(".status-card > p")).toHaveText(accepted ? locale.thanksMessage : locale.errorMessage);
      await expect(page.locator(".status-card li")).toHaveCount(accepted ? 0 : 3);
      if (accepted) await expect(page.locator("h1")).toHaveText(locale.thanks);
      await expect(page.locator(".status-card a")).toHaveAttribute("href", locale.path);
    }
  });
}

test("missing page explains how to return home", async ({ page }) => {
  const response = await page.goto("/missing-page-copy-check");
  expect(response.status()).toBe(404);
  await expect(page.locator(".status-card > p")).toHaveText("Na tejto adrese sme stránku nenašli. Pokračovať môžete z hlavnej stránky.");
  await page.locator(".status-card a").click();
  await expect(page).toHaveURL(/\/$/);
});

for (const width of [360, 768, 1440]) {
  test(`headlines and benefits fit their containers at ${width}px`, async ({ page }) => {
    await page.setViewportSize({ width, height: 1000 });
    await page.emulateMedia({ reducedMotion: "reduce" });
    for (const path of ["/", "/de"]) {
      await page.goto(path);
      const header = page.locator(".topbar-inner");
      expect(await header.evaluate(node => node.scrollWidth <= node.clientWidth)).toBe(true);
      for (const selector of [".hero h1", ".impact-card"]) {
        for (const element of await page.locator(selector).all()) {
          await element.scrollIntoViewIfNeeded();
          expect(await element.evaluate(node => node.scrollWidth <= node.clientWidth), `${path} ${selector} at ${width}px`).toBe(true);
        }
      }
      expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(width);
      await mkdir("artifacts", { recursive: true });
      await page.locator(".hero").screenshot({ path: `artifacts/copy-hero-${path === "/" ? "sk" : "de"}-${width}.png`, animations: "disabled", style: ".topbar, .scroll-progress { visibility: hidden; }" });
      await page.locator(".impact").screenshot({ path: `artifacts/copy-benefits-${path === "/" ? "sk" : "de"}-${width}.png`, animations: "disabled", style: ".topbar, .scroll-progress { visibility: hidden; }" });
    }
  });
}
