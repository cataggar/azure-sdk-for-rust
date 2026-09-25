// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { test } from "node:test";
import assert from "node:assert/strict";
import { adaptPackage, adaptType } from "./index.js";

test("normalizes a minimal client and model with explicit wire metadata", () => {
  const model = adaptPackage({
    clients: [{
      name: "WidgetClient",
      clientInitialization: { parameters: [{ kind: "endpoint", name: "endpoint" }] },
      methods: [{
        name: "getWidget",
        kind: "basic",
        operation: {
          verb: "get", path: "/widgets", parameters: [],
          responses: [{ statusCodes: 200, type: { kind: "model", name: "Widget" } }],
        },
      }],
    }],
    models: [{
      name: "Widget",
      properties: [{
        name: "displayName", serializedName: "display_name",
        type: { kind: "string" }, optional: true, readOnly: true,
      }],
    }],
    enums: [],
    unions: [],
  });
  assert.equal(model.schema_version, 1);
  assert.equal(model.clients[0].operations[0].status_code, 200);
  assert.deepEqual(model.clients[0].operations[0].response_type, { kind: "model", name: "Widget" });
  assert.deepEqual(model.models[0].fields[0], {
    name: "displayName", wire_name: "display_name", type: { kind: "string" },
    optional: true, read_only: true, doc: null,
  });
});

test("rejects unmodeled constructs instead of guessing", () => {
  assert.throws(() => adaptType({ kind: "union" }, "Widget.value"), /Widget\.value: unsupported type kind union/);
  assert.throws(() => adaptPackage({
    clients: [{ name: "Client", clientInitialization: { parameters: [
      { kind: "endpoint", name: "endpoint" }, { kind: "credential", name: "credential" },
    ] } }], models: [],
  }), /client Client: unsupported initialization parameter credential/);
  assert.throws(() => adaptPackage({
    clients: [{ name: "Client", clientInitialization: { parameters: [{ kind: "endpoint", name: "endpoint" }] },
      methods: [{ name: "list", kind: "paging" }] }], models: [],
  }), /client Client\.list: unsupported operation kind paging/);
});
