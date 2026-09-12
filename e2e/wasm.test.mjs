import { readFile } from "node:fs/promises";
import assert from "node:assert/strict";
import { test } from "node:test";

async function engine() {
  const bytes = await readFile(new URL("../assets/hero.wasm", import.meta.url));
  return (await WebAssembly.instantiate(bytes)).instance.exports;
}

test("proximity graph contains exactly the nearby pairs and their opacity", async () => {
  const wasm = await engine();
  wasm.init(4, 600, 400);
  const positions = new Float32Array(wasm.memory.buffer, wasm.positions_ptr(), 8);
  positions.set([0, 0, 30, 40, 100, 0, 400, 300]);
  const count = wasm.build_links(100);
  assert.equal(count, 2);
  const edges = new Float32Array(wasm.memory.buffer, wasm.links_ptr(), count * 5);
  assert.deepEqual([...edges.slice(0, 4)], [0, 0, 30, 40]);
  assert.ok(Math.abs(edges[4] - 0.315) < 0.00001);
  assert.deepEqual([...edges.slice(5, 9)], [30, 40, 100, 0]);
  assert.ok(Math.abs(edges[9] - 0.147) < 0.00001);
});

test("invalid radius, empty fields and isolated nodes produce no connections", async () => {
  const wasm = await engine();
  wasm.init(2, 1000, 1000);
  new Float32Array(wasm.memory.buffer, wasm.positions_ptr(), 4).set([0, 0, 900, 900]);
  for (const radius of [0, -1, NaN, Infinity, 1]) assert.equal(wasm.build_links(radius), 0);
  wasm.init(0, 100, 100);
  assert.equal(wasm.build_links(100), 0);
  wasm.init(1, 100, 100);
  assert.equal(wasm.build_links(100), 0);
});

test("graph supports the full particle capacity without duplicate or stale edges", async () => {
  const wasm = await engine();
  wasm.init(999, 10, 10);
  assert.equal(wasm.count(), 320);
  assert.equal(wasm.build_links(100), 51040);
  const edges = new Float32Array(wasm.memory.buffer, wasm.links_ptr(), 51040 * 5);
  assert.ok([...edges].every(Number.isFinite));
  wasm.init(2, 10, 10);
  assert.equal(wasm.build_links(100), 1);
});

test("graph uses positions after simulation and resize", async () => {
  const wasm = await engine();
  wasm.seed(42);
  wasm.init(2, 100, 100);
  const view = new Float32Array(wasm.memory.buffer, wasm.positions_ptr(), 4);
  const before = [...view];
  wasm.step(0.016);
  assert.notDeepEqual([...view], before);
  const moved = [...view];
  wasm.resize(200, 300);
  assert.ok(Math.abs(view[0] - moved[0] * 2) < 0.001);
  assert.ok(Math.abs(view[1] - moved[1] * 3) < 0.001);
  assert.equal(wasm.build_links(1000), 1);
  const edge = new Float32Array(wasm.memory.buffer, wasm.links_ptr(), 5);
  assert.deepEqual([...edge.slice(0, 4)], [...view]);
});
