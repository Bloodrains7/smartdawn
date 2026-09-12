import { test, expect } from "@playwright/test";

for (const width of [360, 1440]) {
  test(`deployment serves styled HTML and working assets at ${width}px`, async ({ page }) => {
    await page.setViewportSize({ width, height: 1000 });
    await page.route("https://fonts.googleapis.com/**", (route) => route.abort());
    await page.route("https://fonts.gstatic.com/**", (route) => route.abort());
    await page.route("https://cdn.jsdelivr.net/**", (route) => route.abort());
    const resources = new Map();
    page.on("response", (response) => resources.set(new URL(response.url()).pathname, response));

    await page.goto("/");
    await expect(page.locator(".hero")).toHaveAttribute("data-engine", "wasm");
    for (const [path, mime] of [
      ["/assets/style.css", "text/css"],
      ["/assets/hero.js", /(?:text|application)\/javascript/],
      ["/assets/delivery.js", /(?:text|application)\/javascript/],
      ["/assets/hero.wasm", "application/wasm"],
    ]) {
      const response = resources.get(path);
      expect(response, path).toBeDefined();
      expect(response.status(), path).toBe(200);
      expect(response.headers()["content-type"], path).toMatch(mime);
    }
    await expect(page.locator(".hero-grid")).toHaveCSS("display", "grid");
    await expect(page.locator("body")).toHaveCSS("margin-top", "0px");
    expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(width);
    await page.getByTestId("delivery-kind").selectOption("business");
    await expect(page.locator('[data-scope="business"]')).toBeVisible();
    await expect(page.locator('[data-scope="product"]')).toBeHidden();

    await page.goto("/assets/missing.css");
    await expect(page.locator("h1")).toHaveCount(0);
    const missing = resources.get("/assets/missing.css");
    expect(missing.status()).toBe(404);
    expect(missing.headers()["content-type"] || "").not.toContain("text/html");
  });
}
