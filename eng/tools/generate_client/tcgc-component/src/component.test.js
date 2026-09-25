// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { test } from "node:test";
import assert from "node:assert/strict";
import * as fs from "node:fs";
import * as path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { execFileSync } from "node:child_process";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

test("Wasm export compiles a virtual project and returns diagnostics", async () => {
  const jco = path.join(root, "node_modules", "@bytecodealliance", "jco", "src", "jco.js");
  execFileSync(process.execPath, [jco, "transpile", path.join(root, "dist", "tcgc.wasm"),
    "-o", path.join(root, "dist", "transpiled"), "--no-typescript", "--quiet"]);
  const { tcgc } = await import(pathToFileURL(path.join(root, "dist", "transpiled", "tcgc.js")).href);
  const manifest = JSON.parse(fs.readFileSync(path.join(root, "dist", "resources.json"), "utf8"));
  const resources = Object.fromEntries(manifest.files.map((name) => [
    name, fs.readFileSync(path.join(root, "dist", "resources", name.slice(1)), "utf8"),
  ]));
  const options = (source) => JSON.stringify({ __spec_files: {
    ...resources, "/spec/main.tsp": source,
  } });
  const result = JSON.parse(tcgc.compile("/spec", options(`
    import "@typespec/http";
    using TypeSpec.Http;
    @service namespace Example;
    model PingResult { message: string; }
    @route("/ping") op ping(): PingResult;
  `)));
  assert.equal(result.schema_version, 2);
  assert.ok(result.clients.some((client) => client.operations.some((op) => op.name === "ping")));
  const withPath = JSON.parse(tcgc.compile("/spec", options(`
    import "@typespec/http";
    using TypeSpec.Http;
    @service namespace Example;
    model Widget { name: string; }
    @route("/widgets/{widget_id}")
    op getWidget(@path widget_id: string): Widget;
  `)));
  assert.equal(withPath.schema_version, 2);
  const getWidget = withPath.clients.flatMap((client) => client.operations)
    .find((op) => op.name === "getWidget");
  assert.ok(getWidget);
  assert.equal(getWidget.path, "/widgets/{widget_id}");
  assert.deepEqual(getWidget.parameters.find((param) => param.location === "path"), {
    name: "widget_id", wire_name: "widget_id", location: "path",
    type: { kind: "string" }, optional: false, constant: null, client_owned: false,
  });
  assert.throws(() => tcgc.compile("/spec", options("model Broken { invalid: ; }")),
    /TypeSpec compilation failed/);
});
