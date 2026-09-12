import { execFileSync, spawnSync } from "node:child_process";
import assert from "node:assert/strict";
import { setTimeout } from "node:timers/promises";

const name = `smart-dawn-preview-${process.pid}`;
const docker = (...args) => execFileSync("docker", args, { encoding: "utf8", windowsHide: true }).trim();
const vercel = process.argv.includes("--vercel");
const portOverride = process.argv.includes("--port-override");
const image = vercel ? "smart-dawn-web:vercel-preview" : "smart-dawn-web:preview";
const containerPort = vercel && !portOverride ? 80 : 8080;
const portEnv = vercel && !portOverride ? [] : ["--env", "PORT=8080"];
docker("run", "--detach", "--rm", "--name", name, "--publish", `127.0.0.1::${containerPort}`, ...portEnv, "--env", "SITE_URL=https://smartdawn.test", image);

try {
  const port = JSON.parse(docker("inspect", name))[0].NetworkSettings.Ports[`${containerPort}/tcp`][0].HostPort;
  const base = `http://127.0.0.1:${port}`;
  let ready = false;
  for (let i = 0; i < 50; i++) {
    try {
      const response = await fetch(`${base}/healthz`);
      ready = response.status === 200 && await response.text() === "ok";
      if (ready) break;
    } catch {}
    await setTimeout(100);
  }
  assert.ok(ready, "container must listen on the configured PORT");
  assert.equal(docker("exec", name, "id", "-u"), "10001");
  for (const [path, lang] of [["/", "sk"], ["/en", "en"], ["/cz", "cs"], ["/de", "de"]]) {
    const response = await fetch(base + path);
    assert.equal(response.status, 200);
    const html = await response.text();
    assert.ok(html.includes(`<html lang="${lang}">`));
    assert.ok(html.includes('data-testid="delivery-preview"'));
    assert.ok(html.includes(`href="https://smartdawn.test${path}"`));
  }
  const response = await fetch(`${base}/assets/hero.wasm`);
  assert.equal(response.headers.get("content-type"), "application/wasm");
  const wasm = (await WebAssembly.instantiate(await response.arrayBuffer())).instance.exports;
  wasm.init(2, 10, 10);
  assert.equal(wasm.build_links(100), 1);
  for (const path of ["/assets/delivery.js", "/assets/hero.js", "/assets/style.css"]) {
    assert.equal((await fetch(base + path)).status, 200);
  }
  assert.equal((await fetch(`${base}/missing`)).status, 404);
  console.log("Container checks: 11 passed");
  if (vercel && !portOverride) {
    const result = spawnSync(process.execPath, ["node_modules/@playwright/test/cli.js", "test"], {
      env: { ...process.env, PLAYWRIGHT_BASE_URL: base },
      stdio: "inherit",
      timeout: 180000,
      windowsHide: true,
    });
    assert.equal(result.status, 0, "the deployed container must pass the browser suite");
  }
} finally {
  docker("stop", name);
}
