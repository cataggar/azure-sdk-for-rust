// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { test } from "node:test";
import assert from "node:assert/strict";
import { validateFiles, tcgc } from "./wasi-host.js";

test("accepts isolated spec and package virtual paths", () => {
  const files = validateFiles({
    "/spec/main.tsp": "namespace Example;",
    "/node_modules/@typespec/compiler/package.json": "{}",
  });
  assert.equal(files.get("/spec/main.tsp"), "namespace Example;");
});

test("rejects malformed virtual files and unknown options", async () => {
  for (const path of ["/spec/../private.tsp", "/spec//main.tsp", "C:\\spec\\main.tsp", "/etc/passwd"]) {
    assert.throws(() => validateFiles({ [path]: "x" }), /invalid virtual file/);
  }
  assert.throws(() => validateFiles({ "/spec/main.tsp": 42 }), /invalid virtual file/);
  await assert.rejects(tcgc.compile("/spec", JSON.stringify({ __spec_files: {}, unsupported: true })),
    (error) => /only __spec_files/.test(error.payload));
  await assert.rejects(tcgc.compile("/spec", JSON.stringify({ __spec_files: {} })),
    (error) => /missing client.tsp or main.tsp/.test(error.payload));
});
