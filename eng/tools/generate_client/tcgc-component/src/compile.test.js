// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { test } from "node:test";
import assert from "node:assert/strict";
import * as fs from "node:fs";
import * as path from "node:path";
import { fileURLToPath } from "node:url";
import { tcgc } from "./wasi-host.js";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const manifest = JSON.parse(fs.readFileSync(path.join(root, "dist", "resources.json"), "utf8"));
const resources = Object.fromEntries(manifest.files.map((name) => [
  name, fs.readFileSync(path.join(root, "dist", "resources", name.slice(1)), "utf8"),
]));

test("compiles a small TypeSpec project and surfaces invalid input", async () => {
  const options = JSON.stringify({ __spec_files: {
    ...resources,
    "/spec/main.tsp": `
      import "@typespec/http";
      using TypeSpec.Http;
      @service namespace Example;
      @route("/ping") op ping(): string;
    `,
  } });
  const result = JSON.parse(await tcgc.compile("/spec", options));
  assert.equal(result.schema_version, 1);
  assert.ok(result.clients.length > 0);
  await assert.rejects(tcgc.compile("/spec", JSON.stringify({ __spec_files: {
    ...resources, "/spec/main.tsp": "model Broken { invalid: ; }",
  } })), (error) => /TypeSpec compilation failed/.test(error.payload));
});
