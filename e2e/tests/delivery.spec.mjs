import { test, expect } from "@playwright/test";
import { readFile, mkdir } from "node:fs/promises";

const mutations = {
  selection: ["delivery.js", "panel.hidden = panel.dataset.scope !== select.value", "panel.hidden = panel.dataset.scope !== 'product'"],
  extras: ["delivery.js", "chip.hidden = !extras[i].checked", "chip.hidden = true"],
  brief: ["delivery.js", 'nextBrief + (current ? "\\n\\n" + current : "")', "nextBrief"],
  motion: ["hero.js", "!paused && !motion.matches", "!motion.matches"],
  reduced: ["hero.js", "if (motion.matches)", "if (false)"],
};

test.beforeEach(async ({ page }) => {
  await page.route("https://fonts.googleapis.com/**", (route) => route.abort());
  await page.route("https://fonts.gstatic.com/**", (route) => route.abort());
  await page.route("https://cdn.jsdelivr.net/**", (route) => route.abort());
  const mutation = mutations[process.env.DELIVERY_MUTATION];
  if (mutation) {
    const [file, before, after] = mutation;
    const source = await readFile(new URL(`../../assets/${file}`, import.meta.url), "utf8");
    expect(source).toContain(before);
    await page.route(`**/assets/${file}`, (route) => route.fulfill({ contentType: "text/javascript", body: source.replace(before, after) }));
  }
});

const locales = [
  ["/", "sk", "Váš nápad. Náš kód. Nový začiatok.", "Firemný systém", "Roly a oprávnenia"],
  ["/en", "en", "Your idea. Our code. A new dawn.", "Business system", "Roles and permissions"],
  ["/cz", "cs", "Váš nápad. Náš kód. Nový začátek.", "Firemní systém", "Role a oprávnění"],
  ["/de", "de", "Ihre Idee. Unser Code. Ein neuer Anfang.", "Unternehmenssystem", "Rollen und Rechte"],
];

for (const [path, lang, headline, label, module] of locales) {
  test(`complete delivery positioning and localized selection ${lang}`, async ({ page }) => {
    await page.goto(path);
    await expect(page.locator("html")).toHaveAttribute("lang", lang);
    await expect(page.locator("h1")).toHaveText(headline);
    await expect(page.locator(".hero-actions a").first()).toHaveAttribute("href", "#navrh");
    await expect(page.locator(".delivery-strip li")).toHaveCount(4);
    await page.getByTestId("delivery-kind").selectOption("business");
    await expect(page.locator('[data-scope="business"] h3')).toHaveText(label);
    await expect(page.locator('[data-scope="business"]')).toBeVisible();
    await expect(page.locator('[data-scope="business"]')).toContainText(module);
    await expect(page.locator('[data-scope="product"]')).toBeHidden();
    await page.getByTestId("delivery-brief").click();
    await expect(page.locator('textarea[name="message"]')).toHaveValue(new RegExp(label));
    await expect(page.locator('textarea[name="message"]')).toHaveValue(new RegExp(module));
  });
}

test("selection switches all three delivery surfaces and returns to the original", async ({ page }) => {
  await page.goto("/");
  for (const scope of ["automation", "business", "product"]) {
    await page.getByTestId("delivery-kind").selectOption(scope);
    await expect(page.locator(`[data-scope="${scope}"]`)).toBeVisible();
    await expect(page.locator("[data-scope]:visible")).toHaveCount(1);
    await page.getByTestId("delivery-brief").click();
    const heading = await page.locator(`[data-scope="${scope}"] h3`).textContent();
    await expect(page.locator('textarea[name="message"]')).toHaveValue(new RegExp(`^${heading}`));
  }
});

for (const [id, label] of [["delivery-integrations", "Prepojenie na existujúce systémy"], ["delivery-ai", "AI funkcie"]]) {
  test(`optional scope is included and removed: ${id}`, async ({ page }) => {
    await page.goto("/");
    const chip = page.locator(".delivery-chip").filter({ hasText: label });
    await expect(chip).toBeHidden();
    await page.getByTestId(id).check();
    await expect(chip).toBeVisible();
    await page.getByTestId("delivery-brief").click();
    await expect(page.locator('textarea[name="message"]')).toHaveValue(new RegExp(label));
    await page.getByTestId(id).uncheck();
    await expect(chip).toBeHidden();
    await page.getByTestId("delivery-brief").click();
    await expect(page.locator('textarea[name="message"]')).not.toHaveValue(new RegExp(label));
  });
}

test("brief preserves visitor text and updates without duplicate insertion or submission", async ({ page }) => {
  const contactRequests = [];
  page.on("request", (request) => { if (request.url().endsWith("/contact")) contactRequests.push(request); });
  await page.goto("/");
  const message = page.locator('textarea[name="message"]');
  await message.fill("Potrebujeme portál pre našich obchodných partnerov.");
  await page.getByTestId("delivery-brief").click();
  await expect(message).toHaveValue(/Potrebujeme portál pre našich obchodných partnerov\.$/);
  await expect(message).toBeFocused();
  const first = await message.inputValue();
  await page.getByTestId("delivery-brief").click();
  await expect(message).toHaveValue(first);
  await page.getByTestId("delivery-kind").selectOption("business");
  await page.getByTestId("delivery-brief").click();
  await expect(message).toHaveValue(/^Firemný systém/);
  await expect(message).not.toHaveValue(/Webový produkt/);
  await expect(message).toHaveValue(/Potrebujeme portál pre našich obchodných partnerov\.$/);
  expect(contactRequests).toHaveLength(0);
});

test("WASM draws a real graph and animation can be paused and resumed", async ({ page }) => {
  await page.goto("/");
  const hero = page.locator(".hero");
  const toggle = page.getByTestId("motion-toggle");
  await expect(hero).toHaveAttribute("data-engine", "wasm");
  await expect(hero).toHaveAttribute("data-motion", "running");
  expect(await page.locator("#neural").evaluate((canvas) => canvas.getContext("2d").getImageData(0, 0, canvas.width, canvas.height).data.some((value) => value !== 0))).toBe(true);
  await toggle.click();
  await expect(toggle).toHaveAttribute("aria-pressed", "true");
  await expect(hero).toHaveAttribute("data-motion", "paused");
  const frame = await page.locator("#neural").evaluate((canvas) => canvas.toDataURL());
  await page.waitForTimeout(150);
  expect(await page.locator("#neural").evaluate((canvas) => canvas.toDataURL())).toBe(frame);
  await toggle.click();
  await expect(hero).toHaveAttribute("data-motion", "running");
  await expect(toggle).toHaveAttribute("aria-pressed", "false");
  await page.locator("#kontakt").scrollIntoViewIfNeeded();
  await expect(hero).toHaveAttribute("data-motion", "paused");
  await page.locator("h1").scrollIntoViewIfNeeded();
  await expect(hero).toHaveAttribute("data-motion", "running");
});

test("reduced motion skips the WASM download while the project selector works", async ({ page }) => {
  await page.emulateMedia({ reducedMotion: "reduce" });
  const wasmRequests = [];
  page.on("request", (request) => { if (request.url().endsWith(".wasm")) wasmRequests.push(request); });
  await page.goto("/");
  await expect(page.locator(".hero")).toHaveAttribute("data-engine", "static");
  await expect(page.getByTestId("motion-toggle")).toBeHidden();
  await page.getByTestId("delivery-kind").selectOption("automation");
  await expect(page.locator('[data-scope="automation"]')).toBeVisible();
  expect(wasmRequests).toHaveLength(0);
});

test("changing motion preference pauses an already running engine", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator(".hero")).toHaveAttribute("data-motion", "running");
  await page.emulateMedia({ reducedMotion: "reduce" });
  await expect(page.locator(".hero")).toHaveAttribute("data-motion", "paused");
  await expect(page.getByTestId("motion-toggle")).toBeHidden();
  await page.emulateMedia({ reducedMotion: "no-preference" });
  await expect(page.locator(".hero")).toHaveAttribute("data-motion", "running");
});

test("WASM failure leaves the delivery selector and contact links usable", async ({ page }) => {
  await page.route("**/assets/hero.wasm", (route) => route.abort());
  await page.goto("/");
  await expect(page.locator(".hero")).toHaveAttribute("data-engine", "static");
  await expect(page.locator("h1")).toBeVisible();
  await page.getByTestId("delivery-kind").selectOption("business");
  await page.getByTestId("delivery-brief").click();
  await expect(page.locator('textarea[name="message"]')).toHaveValue(/^Firemný systém/);
  await expect(page.locator('.contact-form a[href="mailto:hello@smartdawn.eu"]')).toBeVisible();
});

test("server-rendered offer remains usable without JavaScript", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto("http://127.0.0.1:3107/");
  await expect(page.locator("h1")).toHaveText(locales[0][2]);
  await expect(page.locator('[data-scope="product"]')).toBeVisible();
  await expect(page.getByTestId("delivery-kind")).toBeHidden();
  await expect(page.locator(".delivery-strip li")).toHaveCount(4);
  await page.locator('.hero-actions a[href="#kontakt"]').click();
  await expect(page.locator(".contact-form")).toBeInViewport();
  await context.close();
});

for (const width of [360, 768, 1440]) {
  test(`layout stays inside the viewport at ${width}px`, async ({ page }) => {
    await page.setViewportSize({ width, height: 1000 });
    await page.goto("/");
    await page.getByTestId("delivery-kind").selectOption("automation");
    await expect(page.locator('[data-scope="automation"]')).toBeVisible();
    const bounds = await page.getByTestId("delivery-preview").boundingBox();
    expect(bounds.x).toBeGreaterThanOrEqual(0);
    expect(bounds.x + bounds.width).toBeLessThanOrEqual(width);
    expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(width);
    if (width < 980) {
      await page.locator(".hamburger").click();
      await expect(page.locator('.mobile-menu a[href="#kontakt"]').first()).toBeVisible();
      await page.locator(".hamburger").click();
      await expect(page.locator('.mobile-menu a[href="#kontakt"]').first()).toBeHidden();
    }
    await mkdir("artifacts", { recursive: true });
    await page.screenshot({ path: `artifacts/smart-dawn-${width}.png`, fullPage: false });
  });
}

test("production routes serve WASM with its MIME type and a healthy endpoint", async ({ request }) => {
  const health = await request.get("/healthz");
  expect(health.status()).toBe(200);
  expect(await health.text()).toBe("ok");
  const wasm = await request.get("/assets/hero.wasm");
  expect(wasm.headers()["content-type"]).toBe("application/wasm");
  expect(WebAssembly.validate(await wasm.body())).toBe(true);
});
