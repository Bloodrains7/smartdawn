import { spawnSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";

const checks = [
  ["selection", "selection switches all three"],
  ["extras", "optional scope is included and removed"],
  ["brief", "brief preserves visitor text"],
  ["motion", "WASM draws a real graph"],
  ["reduced", "reduced motion skips"],
  ["portfolio-priority", "Jarvis leads the portfolio"],
  ["portfolio-catalogue", "portfolio catalogue and inquiry"],
  ["portfolio-inquiry", "solution inquiry preserves visitor text"],
  ["portfolio-layout", "portfolio layout keeps every solution readable at 360px"],
  ["copy-punctuation", "page copy uses plain sentences and concrete benefits in sk"],
  ["copy-industries", "industry list excludes casinos and retains the other sectors in sk"],
  ["copy-benefits", "page copy uses plain sentences and concrete benefits in sk"],
];

mkdirSync("artifacts/mutations", { recursive: true });
let killed = 0;
for (const [mutation, title] of checks) {
  const result = spawnSync(process.execPath, ["node_modules/@playwright/test/cli.js", "test", "--grep", title, "--workers=1"], {
    env: { ...process.env, DELIVERY_MUTATION: mutation },
    encoding: "utf8",
    timeout: 180000,
    windowsHide: true,
  });
  const output = result.stdout + result.stderr;
  writeFileSync(`artifacts/mutations/${mutation}.txt`, output);
  const rejected = result.status === 1 && /[1-9]\d* failed/.test(output) && /expect\(/.test(output)
    && !output.includes("expect(source).toContain(before)") && !output.includes("Error: mutation target");
  if (rejected) killed++;
  console.log(`${mutation}: ${rejected ? "killed" : "not proven"}`);
}
console.log(`Mutation checks: ${killed}/${checks.length} killed`);
if (killed !== checks.length) process.exitCode = 1;
