import { test, expect } from "@playwright/test";
import { mkdir, readFile } from "node:fs/promises";

const projects = [
  ["jarvis", "Jarvis"], ["cadence", "Cadence"], ["tally", "Tally"],
  ["forge", "Forge"], ["mr-reviewer", "MR Reviewer"], ["archgen", "ArchGen"],
  ["repre", "Mapovanie dát"], ["parley", "Parley / vox"], ["own-ide", "Own IDE"],
];

const locales = [
  {
    "path": "/",
    "lang": "sk",
    "nav": "Riešenia",
    "heading": "Nástroje pre každodennú prácu",
    "interest": "Zaujíma nás",
    "contact": "Mám záujem o riešenie",
    "workflow": "Príklady využitia",
    "mappingName": "Mapovanie dát",
    "mappingCategory": "Dodávateľské dáta pre e-shop",
    "mappingDescription": "Prepojí dodávateľské feedy s katalógom vášho e-shopu. Priradí zdrojové polia k produktovým údajom, zjednotí formáty a kategórie a pripraví dáta na import. Uložené mapovania a kontrola zmien zjednodušujú pravidelné aktualizácie.",
    "promises": [
      "Pomoc s poštou, úlohami a poradami.",
      "Plánovanie vydaní podľa kapacít tímu.",
      "Mesačné výkazy v zákazníkovej šablóne.",
      "Overenie API aj nadväzujúcich systémov.",
      "Posúdenie zmien pred ich zaradením.",
      "Diagramy a dokumentácia k návrhu.",
      "Dáta od dodávateľov priamo do e-shopu.",
      "Prepis a preklad počas porady.",
      "Práca s kódom a AI asistentom."
    ]
  },
  {
    "path": "/en",
    "lang": "en",
    "nav": "Solutions",
    "heading": "Tools for everyday work",
    "interest": "We are interested in",
    "contact": "Discuss this solution",
    "workflow": "Examples of use",
    "mappingName": "Data mapping",
    "mappingCategory": "Supplier data for your online store",
    "mappingDescription": "Connect supplier feeds to your online store's catalogue. Map source fields to product data, normalize formats and categories, and prepare data for import. Saved mappings and change checks simplify regular updates.",
    "promises": [
      "Help with mail, tasks and meetings.",
      "Release planning based on team capacity.",
      "Monthly timesheets in your customer's template.",
      "Check APIs and connected systems.",
      "Assess changes before they are merged.",
      "Diagrams and documentation for your design.",
      "Supplier data directly into your online store.",
      "Transcription and translation during meetings.",
      "Work with code and an AI assistant."
    ]
  },
  {
    "path": "/cz",
    "lang": "cs",
    "nav": "Řešení",
    "heading": "Nástroje pro každodenní práci",
    "interest": "Zajímá nás",
    "contact": "Mám zájem o řešení",
    "workflow": "Příklady využití",
    "mappingName": "Mapování dat",
    "mappingCategory": "Dodavatelská data pro e-shop",
    "mappingDescription": "Propojí dodavatelské feedy s katalogem vašeho e-shopu. Přiřadí zdrojová pole k produktovým údajům, sjednotí formáty a kategorie a připraví data k importu. Uložená mapování a kontrola změn zjednodušují pravidelné aktualizace.",
    "promises": [
      "Pomoc s poštou, úkoly a poradami.",
      "Plánování vydání podle kapacit týmu.",
      "Měsíční výkazy v zákazníkově šabloně.",
      "Ověření API i navazujících systémů.",
      "Posouzení změn před jejich zařazením.",
      "Diagramy a dokumentace k návrhu.",
      "Data od dodavatelů přímo do e-shopu.",
      "Přepis a překlad během porady.",
      "Práce s kódem a AI asistentem."
    ]
  },
  {
    "path": "/de",
    "lang": "de",
    "nav": "Lösungen",
    "heading": "Werkzeuge für die tägliche Arbeit",
    "interest": "Wir interessieren uns für",
    "contact": "Über diese Lösung sprechen",
    "workflow": "Anwendungsbeispiele",
    "mappingName": "Datenmapping",
    "mappingCategory": "Lieferantendaten für Ihren Online-Shop",
    "mappingDescription": "Verbindet Lieferantenfeeds mit dem Katalog Ihres Online-Shops. Ordnet Quellfelder den Produktdaten zu, vereinheitlicht Formate und Kategorien und bereitet Daten für den Import vor. Gespeicherte Zuordnungen und Änderungsprüfungen vereinfachen regelmäßige Aktualisierungen.",
    "promises": [
      "Hilfe bei Post, Aufgaben und Meetings.",
      "Release-Planung nach Teamkapazität.",
      "Monatliche Nachweise in der Kundenvorlage.",
      "APIs und verbundene Systeme prüfen.",
      "Änderungen vor der Übernahme bewerten.",
      "Diagramme und Dokumentation zum Entwurf.",
      "Lieferantendaten direkt in Ihren Online-Shop.",
      "Transkription und Übersetzung im Meeting.",
      "Mit Code und einem KI-Assistenten arbeiten."
    ]
  }
];

test.beforeEach(async ({ page }) => {
  await page.route("https://fonts.googleapis.com/**", route => route.abort());
  await page.route("https://fonts.gstatic.com/**", route => route.abort());
  await page.route("https://cdn.jsdelivr.net/**", route => route.abort());
  const mutation = process.env.DELIVERY_MUTATION;
  if (mutation === "portfolio-priority" || mutation === "portfolio-catalogue") {
    await page.route(/\/($|en$|cz$|de$)/, async route => {
      const response = await route.fetch();
      let body = await response.text();
      const jarvis = body.match(/<article\b[^>]*data-project="jarvis"[^>]*>[\s\S]*?<\/article>/)?.[0];
      const tally = body.match(/<article\b[^>]*data-project="tally"[^>]*>[\s\S]*?<\/article>/)?.[0];
      expect(jarvis, "mutation target Jarvis").toBeTruthy();
      expect(tally, "mutation target Tally").toBeTruthy();
      body = mutation === "portfolio-priority" ? body.replace(jarvis, "").replace(tally, tally + jarvis) : body.replace(tally, "");
      await route.fulfill({ response, body });
    });
  }
  const assets = {
    "portfolio-inquiry": ["delivery.js", "insertBrief(link.dataset.solutionBrief);", 'insertBrief("Jarvis");'],
    "portfolio-layout": ["style.css", ".daily-solutions, .portfolio-grid { grid-template-columns: minmax(0, 1fr); }", ".daily-solutions, .portfolio-grid { grid-template-columns: repeat(3, minmax(0, 1fr)); }"],
  };
  if (assets[mutation]) {
    const [file, before, after] = assets[mutation];
    const source = await readFile(new URL(`../../assets/${file}`, import.meta.url), "utf8");
    expect(source).toContain(before);
    await page.route(`**/assets/${file}`, route => route.fulfill({ contentType: file.endsWith("css") ? "text/css" : "text/javascript", body: source.replace(before, after) }));
  }
});

for (const locale of locales) {
  test(`portfolio catalogue and inquiry are localized in ${locale.lang}`, async ({ page }) => {
    await page.goto(locale.path);
    const portfolio = page.getByTestId("portfolio");
    await expect(portfolio.locator("[data-project]")).toHaveCount(9);
    await expect(portfolio.locator("h2")).toHaveText(locale.heading);
    await expect(page.locator('.desktop-menu a[href="#praca"]')).toHaveText(locale.nav);
    await expect(page.locator('meta[name="description"]')).toHaveAttribute("content", /Jarvis.*Cadence.*Tally/);
    await expect(page.locator(".interview-body")).toContainText("Jarvis");
    await expect(page.locator("footer")).toContainText("Jarvis");
    await expect(page.locator(".faq")).toContainText("Cadence, Tally");
    await expect(portfolio.locator(".jarvis-workflow-head")).toContainText(locale.workflow);
    await expect(portfolio).not.toContainText(/Roman Empire|Open Claw|coming soon|pripravujeme|plánované/i);
    for (const [i, [slug, name]] of projects.entries()) {
      const card = portfolio.locator(`[data-project="${slug}"]`);
      const localizedName = slug === "repre" ? locale.mappingName : name;
      await expect(card.getByRole("heading", { name: localizedName, exact: true })).toHaveCount(1);
      await expect(card.locator(".solution-promise, .jarvis-promise")).toHaveText(locale.promises[i]);
      await expect(card.getByRole("link")).toHaveAccessibleName(`${locale.contact}: ${localizedName}`);
    }
    await page.getByTestId("solution-contact-jarvis").click();
    await expect(page.locator('textarea[name="message"]')).toHaveValue(`${locale.interest} Jarvis: ${locale.promises[0]}`);
  });

  test(`supplier data mapping targets online stores without vendor branding in ${locale.lang}`, async ({ page }) => {
    const requests = [];
    page.on("request", request => { if (new URL(request.url()).pathname === "/contact") requests.push(request); });
    await page.goto(locale.path);
    const card = page.locator('[data-project="repre"]');
    await expect(card.locator("h4")).toHaveText(locale.mappingName);
    await expect(card.locator(".solution-category")).toHaveText(locale.mappingCategory);
    await expect(card.locator(".solution-promise")).toHaveText(locale.promises[6]);
    await expect(card.locator(".solution-description")).toHaveText(locale.mappingDescription);
    await expect(card.locator(".solution-tags li")).toHaveText(["CSV", "XML", "API"]);
    await expect(page.locator("body")).not.toContainText(/\bREPRE\b|\bPOHOD\w*/i);
    const link = page.getByTestId("solution-contact-repre");
    await expect(link).toHaveAccessibleName(`${locale.contact}: ${locale.mappingName}`);
    const message = page.locator('textarea[name="message"]');
    for (const note of ["", "CSV / XML → shop.example"]) {
      await message.fill(note);
      await link.click();
      await expect(message).toHaveValue(`${locale.interest} ${locale.mappingName}: ${locale.promises[6]}${note ? `\n\n${note}` : ""}`);
      await expect(message).not.toHaveValue(/REPRE|POHOD/i);
    }
    expect(requests).toHaveLength(0);
  });
}

test("Jarvis leads the portfolio from the hero ahead of the daily tools", async ({ page }) => {
  await page.goto("/");
  const teaser = page.getByTestId("jarvis-teaser");
  await expect(teaser).toBeVisible();
  await expect(teaser).toContainText("Jarvis pomôže s poštou, úlohami a poradami.");
  await teaser.click();
  await expect(page).toHaveURL(/#jarvis$/);
  await expect(page.locator("#jarvis")).toBeInViewport();
  await expect(page.locator("#praca [data-project]").first()).toHaveAttribute("data-project", "jarvis");
  await expect(page.locator(".jarvis-workflow li")).toHaveCount(3);
  for (const label of ["Spracovanie pošty", "Práca s úlohami", "Zápis z porady"]) {
    await expect(page.locator(".jarvis-workflow")).toContainText(label);
  }
  await expect(page.locator(".daily-solutions [data-project]")).toHaveCount(2);
  await expect(page.locator(".daily-solutions h4")).toHaveText(["Cadence", "Tally"]);
  await expect(page.locator(".portfolio-grid [data-project]")).toHaveCount(6);
  await expect(page.locator("#jarvis .solution-tags li")).toHaveText(["E-mail", "Telegram", "Slack", "Notion", "GitLab"]);
});

for (const [i, [slug, name]] of projects.entries()) {
  test(`solution inquiry preserves visitor text without sending: ${slug}`, async ({ page }) => {
    const requests = [];
    page.on("request", request => { if (new URL(request.url()).pathname === "/contact") requests.push(request); });
    await page.goto("/");
    const message = page.locator('textarea[name="message"]');
    const note = "Používame vlastný systém a potrebujeme ho prepojiť.";
    await message.fill(note);
    const link = page.getByTestId(`solution-contact-${slug}`);
    await link.click();
    const expected = `Zaujíma nás ${name}: ${locales[0].promises[i]}\n\n${note}`;
    await expect(message).toHaveValue(expected);
    await expect(message).toBeFocused();
    await expect(page).toHaveURL(/#kontakt$/);
    await link.click();
    await expect(message).toHaveValue(expected);
    await message.fill("");
    await link.click();
    await expect(message).toHaveValue(`Zaujíma nás ${name}: ${locales[0].promises[i]}`);
    expect(requests).toHaveLength(0);
  });
}

test("switching between solutions and custom delivery replaces only the inserted brief", async ({ page }) => {
  await page.goto("/");
  const message = page.locator('textarea[name="message"]');
  const note = "Zadanie upravíme spoločne.";
  await message.fill(note);
  await page.getByTestId("solution-contact-jarvis").click();
  await page.getByTestId("solution-contact-tally").click();
  await expect(message).toHaveValue(`Zaujíma nás Tally: ${locales[0].promises[2]}\n\n${note}`);
  await page.getByTestId("delivery-kind").selectOption("business");
  await page.getByTestId("delivery-brief").click();
  await expect(message).toHaveValue(/^Firemný systém/);
  await expect(message).not.toHaveValue(/Jarvis|Tally/);
  await expect(message).toHaveValue(new RegExp(`${note}$`));
  await page.getByTestId("solution-contact-cadence").click();
  await expect(message).toHaveValue(`Zaujíma nás Cadence: ${locales[0].promises[1]}\n\n${note}`);
  const edited = "Naše upravené zadanie pre Cadence.";
  await message.fill(edited);
  await page.getByTestId("solution-contact-jarvis").click();
  await expect(message).toHaveValue(`Zaujíma nás Jarvis: ${locales[0].promises[0]}\n\n${edited}`);
});

test("Jarvis inquiry works with the keyboard", async ({ page }) => {
  await page.goto("/");
  await page.getByTestId("jarvis-teaser").focus();
  await page.keyboard.press("Enter");
  await expect(page.locator("#jarvis")).toBeInViewport();
  await page.getByTestId("solution-contact-jarvis").focus();
  await page.keyboard.press("Enter");
  await expect(page.locator('textarea[name="message"]')).toBeFocused();
  await expect(page.locator('textarea[name="message"]')).toHaveValue(`Zaujíma nás Jarvis: ${locales[0].promises[0]}`);
});

test("opening a solution inquiry in a new tab preserves the current draft", async ({ page, context }) => {
  await page.goto("/");
  const message = page.locator('textarea[name="message"]');
  await message.fill("Rozpracované zadanie.");
  const [opened] = await Promise.all([
    context.waitForEvent("page"),
    page.getByTestId("solution-contact-jarvis").click({ modifiers: ["ControlOrMeta"] }),
  ]);
  await expect(opened).toHaveURL(/#kontakt$/);
  await expect(message).toHaveValue("Rozpracované zadanie.");
  await expect(page).not.toHaveURL(/#kontakt$/);
  await opened.close();
});

test.describe("without JavaScript", () => {
  test.use({ javaScriptEnabled: false, reducedMotion: "reduce" });
  for (const locale of locales) {
    test(`portfolio and contact anchors work without JavaScript in ${locale.lang}`, async ({ page }) => {
      test.setTimeout(60_000);
      await page.goto(locale.path);
      await expect(page.getByTestId("portfolio").locator("h2")).toHaveText(locale.heading);
      await page.getByTestId("jarvis-teaser").click();
      await expect(page.locator("#jarvis")).toBeInViewport();
      for (const [slug] of projects) {
        const card = page.locator(`[data-project="${slug}"]`);
        await expect(card).toBeVisible();
        const link = card.getByRole("link");
        await link.focus();
        await expect(link).toBeFocused();
        await expect(link).toBeInViewport();
        await page.keyboard.press("Enter");
        await expect(page.locator(".contact-form")).toBeInViewport();
        await expect(page.locator('textarea[name="message"]')).toHaveValue("");
      }
    });
  }
});

for (const width of [360, 768, 1440]) {
  test(`portfolio layout keeps every solution readable at ${width}px`, async ({ page }) => {
    await page.setViewportSize({ width, height: 1000 });
    await page.emulateMedia({ reducedMotion: "reduce" });
    await page.goto("/de");
    const cards = page.locator("#praca [data-project]");
    for (const card of await cards.all()) {
      await card.scrollIntoViewIfNeeded();
      await expect(card).toBeVisible();
      const bounds = await card.boundingBox();
      expect(bounds.x).toBeGreaterThanOrEqual(0);
      expect(bounds.x + bounds.width).toBeLessThanOrEqual(width);
      expect(bounds.width).toBeGreaterThanOrEqual(280);
      for (const element of await card.locator("h3, h4, p, a").all()) {
        expect(await element.evaluate(node => node.scrollWidth <= node.clientWidth)).toBe(true);
      }
    }
    const columns = await page.locator(".portfolio-grid").evaluate(node => getComputedStyle(node).gridTemplateColumns.split(" ").length);
    expect(columns).toBe(width <= 600 ? 1 : width <= 980 ? 2 : 3);
    expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(width);
    await mkdir("artifacts", { recursive: true });
    const options = { animations: "disabled", style: ".topbar, .scroll-progress { visibility: hidden; }" };
    await page.getByTestId("portfolio").screenshot({ ...options, path: `artifacts/portfolio-${width}.png` });
    await page.locator("#jarvis").screenshot({ ...options, path: `artifacts/jarvis-${width}.png` });
  });
}
