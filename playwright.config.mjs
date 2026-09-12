import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./e2e/tests",
  testMatch: "**/*.spec.mjs",
  fullyParallel: true,
  workers: 2,
  retries: 0,
  reporter: "list",
  use: {
    baseURL: "http://127.0.0.1:3107",
    viewport: { width: 1440, height: 1000 },
    trace: "retain-on-failure",
  },
  webServer: {
    command: "cargo run --locked --offline",
    url: "http://127.0.0.1:3107/healthz",
    env: { PORT: "3107", SITE_URL: "http://127.0.0.1:3107", RUST_LOG: "warn" },
    reuseExistingServer: false,
    timeout: 120000,
  },
});
