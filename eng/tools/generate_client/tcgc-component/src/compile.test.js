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
      model PingResult { message: string; }
      @route("/ping") op ping(): PingResult;
    `,
  } });
  const result = JSON.parse(await tcgc.compile("/spec", options));
  assert.equal(result.schema_version, 2);
  assert.ok(result.clients.length > 0);
  await assert.rejects(tcgc.compile("/spec", JSON.stringify({ __spec_files: {
    ...resources, "/spec/main.tsp": "model Broken { invalid: ; }",
  } })), (error) => /TypeSpec compilation failed/.test(error.payload));
});

test("compiles path, query, and header method bindings from TypeSpec", async () => {
  const options = JSON.stringify({ __spec_files: {
    ...resources,
    "/spec/main.tsp": `
      import "@typespec/http";
      using TypeSpec.Http;
      @service namespace Example;
      model Widget { name: string; }
      @route("/widgets/{widget_id}")
      op getWidget(
        @path widget_id: string,
        @query search_term?: string,
        @header("x-request-id") requestId: string
      ): Widget;
    `,
  } });
  const result = JSON.parse(await tcgc.compile("/spec", options));
  const op = result.clients.flatMap((client) => client.operations)
    .find((operation) => operation.name === "getWidget");
  assert.ok(op);
  assert.equal(op.path, "/widgets/{widget_id}");
  assert.deepEqual(op.parameters.map(({ wire_name, location, optional }) =>
    ({ wire_name, location, optional })), [
    { wire_name: "widget_id", location: "path", optional: false },
    { wire_name: "search_term", location: "query", optional: true },
    { wire_name: "x-request-id", location: "header", optional: false },
    { wire_name: "Accept", location: "header", optional: false },
  ]);
  assert.equal(op.parameters.find((param) => param.wire_name === "Accept").constant, "application/json");
});

test("rejects TypeSpec text/plain responses without a response-format contract", async () => {
  const options = JSON.stringify({ __spec_files: {
    ...resources,
    "/spec/main.tsp": `
      import "@typespec/http";
      using TypeSpec.Http;
      @service namespace Example;
      @route("/ping") op ping(): string;
    `,
  } });
  await assert.rejects(tcgc.compile("/spec", options),
    (error) => /unsupported response content type text\/plain/.test(error.payload));
});
