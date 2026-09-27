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
  assert.ok(op.parameters.every((param) => param.client_owned === false));
});

test("compiles a hyphenated Key Vault path placeholder as a whole URL segment", async () => {
  const options = JSON.stringify({ __spec_files: {
    ...resources,
    "/spec/main.tsp": `
      import "@typespec/http";
      using TypeSpec.Http;
      @service namespace Example;
      model Secret { name: string; }
      @route("/secrets/{secret-name}")
      op getSecret(@path("secret-name") secretName: string): Secret;
    `,
  } });
  const result = JSON.parse(await tcgc.compile("/spec", options));
  const op = result.clients.flatMap((client) => client.operations)
    .find((operation) => operation.name === "getSecret");
  assert.ok(op);
  assert.equal(op.path, "/secrets/{secret-name}");
  assert.deepEqual(op.parameters.find((param) => param.location === "path"), {
    name: "secretName", wire_name: "secret-name", location: "path",
    type: { kind: "string" }, optional: false, constant: null, client_owned: false,
  });
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

test("compiles a recursive nullable model union without dropping its optional fields", async () => {
  const options = JSON.stringify({ __spec_files: {
    ...resources,
    "/spec/main.tsp": `
      import "@typespec/http";
      using TypeSpec.Http;
      @service namespace Example;
      union Error { null, { code?: string; message?: string; innerError?: Error; } }
      model Wrapper { error?: Error; }
      @route("/ping") op ping(): Wrapper;
    `,
  } });
  const result = JSON.parse(await tcgc.compile("/spec", options));
  const error = result.models.find((model) => model.fields.some((field) => field.name === "innerError"));
  const wrapper = result.models.find((model) => model.name === "Wrapper");
  assert.ok(error);
  assert.ok(wrapper);
  assert.deepEqual(error.fields.find((field) => field.name === "innerError").type,
    { kind: "model", name: error.name });
  assert.deepEqual(wrapper.fields.find((field) => field.name === "error").type,
    { kind: "model", name: error.name });
  assert.equal(result.enums.length, 0);
});

test("compiles a single implicit OAuth2 scope with its flow and authorization URL", async () => {
  const options = JSON.stringify({ __spec_files: {
    ...resources,
    "/spec/main.tsp": `
      import "@typespec/http";
      using TypeSpec.Http;
      @useAuth(OAuth2Auth<[{
        type: OAuth2FlowType.implicit,
        authorizationUrl: "https://login.example.com/authorize",
        scopes: ["https://vault.example.com/.default"],
      }]>)
      @service namespace Example;
      model PingResult { message: string; }
      @route("/ping") op ping(): PingResult;
    `,
  } });
  const result = JSON.parse(await tcgc.compile("/spec", options));
  assert.deepEqual(result.clients[0].authentication, {
    kind: "oauth2", flow: "implicit", authorization_url: "https://login.example.com/authorize",
    scopes: ["https://vault.example.com/.default"],
  });
  assert.equal(result.clients[0].api_version, null);
});

test("compiles a direct model body with explicit JSON content type", async () => {
  const options = JSON.stringify({ __spec_files: {
    ...resources,
    "/spec/main.tsp": `
      import "@typespec/http";
      using TypeSpec.Http;
      @service namespace Example;
      model Widget { name: string; }
      @route("/widgets") @post op create(@body parameters: Widget): Widget;
    `,
  } });
  const result = JSON.parse(await tcgc.compile("/spec", options));
  const op = result.clients.flatMap((client) => client.operations)
    .find((operation) => operation.name === "create");
  assert.ok(op);
  assert.deepEqual(op.body, {
    name: "parameters", type: { kind: "model", name: "Widget" }, optional: false,
  });
  assert.equal(op.parameters.find((param) => param.wire_name === "Content-Type").constant,
    "application/json");
});

test("rejects a dynamic Content-Type header on a JSON model body", async () => {
  const options = JSON.stringify({ __spec_files: {
    ...resources,
    "/spec/main.tsp": `
      import "@typespec/http";
      using TypeSpec.Http;
      @service namespace Example;
      model Widget { name: string; }
      @route("/widgets") @post op create(
        @header("Content-Type") contentType: string,
        @body parameters: Widget
      ): Widget;
    `,
  } });
  await assert.rejects(tcgc.compile("/spec", options),
    (error) => /unsupported request body binding or JSON encoding/.test(error.payload));
});

test("compiles an explicitly base64url-encoded JSON bytes field", async () => {
  const options = JSON.stringify({ __spec_files: {
    ...resources,
    "/spec/main.tsp": `
      import "@typespec/http";
      using TypeSpec.Http;
      @service namespace Example;
      model Backup { @encode("base64url") value?: bytes; }
      @route("/backup") op getBackup(): Backup;
    `,
  } });
  const result = JSON.parse(await tcgc.compile("/spec", options));
  assert.deepEqual(result.models.find((model) => model.name === "Backup").fields[0].type,
    { kind: "bytes", encoding: "base64url" });
});

test("compiles an explicit integer Unix timestamp model field", async () => {
  const options = JSON.stringify({ __spec_files: {
    ...resources,
    "/spec/main.tsp": `
      import "@typespec/http";
      using TypeSpec.Http;
      @service namespace Example;
      model DeletedSecret { @encode("unixTimestamp", int32) deletedDate?: utcDateTime; }
      @route("/deleted") op getDeleted(): DeletedSecret;
    `,
  } });
  const result = JSON.parse(await tcgc.compile("/spec", options));
  assert.deepEqual(result.models.find((model) => model.name === "DeletedSecret").fields[0].type,
    { kind: "utcDateTime", format: "unixTime", wire_type: "int32" });
});

const pinnedProject = path.join(process.env.LOCALAPPDATA ?? root, "azure-sdk-for-rust",
  "typespec", "8d521358db4a46b60acaa6aa4bfc26769e2ff6cc", "specification",
  "keyvault", "data-plane", "Secrets");

test("pinned Key Vault compiles paging metadata and all operations", {
  skip: !fs.existsSync(path.join(pinnedProject, "client.tsp")),
}, async () => {
  const files = { ...resources };
  const collect = (directory, virtual) => {
    for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
      const file = path.join(directory, entry.name);
      const target = `${virtual}/${entry.name}`;
      if (entry.isDirectory()) collect(file, target);
      else if (entry.name.endsWith(".tsp")) files[target] = fs.readFileSync(file, "utf8");
    }
  };
  collect(pinnedProject, "/spec");
  const result = JSON.parse(await tcgc.compile("/spec", JSON.stringify({ __spec_files: files })));
  const client = result.clients.find((value) => value.name === "Secret");
  assert.ok(client);
  assert.equal(client.operations.length, 12);
  assert.deepEqual(client.operations.filter((op) => op.kind === "paging").map((op) => op.name), [
    "ListSecretProperties", "ListSecretPropertiesVersions", "ListDeletedSecretProperties",
  ]);
  const pager = client.operations.find((op) => op.name === "ListSecretProperties");
  assert.deepEqual(pager.response_type, { kind: "model", name: "ListSecretPropertiesResult" });
  assert.deepEqual(pager.paging, {
    next_link: { name: "nextLink", wire_name: "nextLink", optional: true },
    items: { name: "value", wire_name: "value", optional: true,
      type: { kind: "array", value_type: { kind: "model", name: "SecretProperties" } } },
    next_link_verb: "GET", reinject_api_version: true,
  });
  assert.deepEqual(pager.parameters.map((param) =>
    [param.wire_name, param.location, param.client_owned]), [
    ["api-version", "query", true], ["maxresults", "query", false],
    ["Accept", "header", false],
  ]);
  assert.deepEqual(pager.status_codes, [200]);
  assert.deepEqual(pager.exceptions, [{
    status_codes: "*", content_type: "application/json",
    type: { kind: "model", name: "KeyVaultError" },
  }]);
  const backup = result.models.find((model) => model.name === "BackupSecretResult");
  assert.deepEqual(backup.fields.find((field) => field.name === "value").type,
    { kind: "bytes", encoding: "base64url" });
  const restore = result.models.find((model) => model.name === "RestoreSecretParameters");
  assert.deepEqual(restore.fields.find((field) => field.name === "SecretBackup"), {
    name: "SecretBackup", wire_name: "value",
    type: { kind: "bytes", encoding: "base64url" },
    optional: false, read_only: false,
    doc: "The backup blob associated with a secret bundle.",
  });
  const times = result.models.flatMap((model) =>
    model.fields.filter((field) => field.type.kind === "utcDateTime")
      .map((field) => [model.name, field.name, field.wire_name, field.optional, field.type]));
  assert.equal(times.length, 8);
  for (const [, , , optional, type] of times) {
    assert.equal(optional, true);
    assert.deepEqual(type, { kind: "utcDateTime", format: "unixTime", wire_type: "int32" });
  }
  assert.deepEqual(times.filter(([model]) => model === "SecretAttributes")
    .map(([, name, wireName]) => [name, wireName]), [
    ["NotBefore", "nbf"], ["Expires", "exp"], ["created", "created"], ["updated", "updated"],
  ]);
});
