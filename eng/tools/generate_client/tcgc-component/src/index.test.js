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
          responses: [{ statusCodes: 200, type: { kind: "model", name: "Widget" },
            contentTypes: ["application/json"] }],
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
  assert.equal(model.schema_version, 2);
  assert.deepEqual(model.clients[0].operations[0].parameters, []);
  assert.deepEqual(model.clients[0].operations[0].status_codes, [200]);
  assert.deepEqual(model.clients[0].operations[0].response_type, { kind: "model", name: "Widget" });
  assert.deepEqual(model.models[0].fields[0], {
    name: "displayName", wire_name: "display_name", type: { kind: "string" },
    optional: true, read_only: true, doc: null,
  });
});

function operationModel({ path = "/widgets/{widget_id}", bindings = [], methods = [],
  responses = [{ statusCodes: 200 }, { statusCodes: 204 }], response } = {}) {
  return adaptPackage({
    clients: [{
      name: "Client",
      clientInitialization: { parameters: [{ kind: "endpoint", name: "endpoint" }] },
      methods: [{
        name: "getWidget", kind: "basic", parameters: methods, response,
        operation: { verb: "get", path, parameters: bindings, responses },
      }],
    }],
    models: [],
  }).clients[0].operations[0];
}

function binding(kind, name, serializedName, source, overrides = {}) {
  return {
    kind, name, serializedName, type: source.type, optional: source.optional,
    methodParameterSegments: [[source]], correspondingMethodParams: [source],
    ...(kind === "path" ? { style: "simple", allowReserved: false, explode: false } : {}),
    ...(kind === "query" ? { explode: false } : {}),
    ...overrides,
  };
}

test("adapts direct method bindings with distinct wire names and numeric successes", () => {
  const id = { kind: "method", name: "widgetId", type: { kind: "string" }, optional: false };
  const filter = { kind: "method", name: "searchTerm", type: { kind: "string" }, optional: true };
  const requestId = { kind: "method", name: "requestId", type: { kind: "string" }, optional: false };
  const op = operationModel({
    methods: [id, filter, requestId],
    bindings: [
      binding("path", "widgetId", "widget_id", id),
      binding("query", "searchTerm", "search_term", filter),
      binding("header", "requestId", "x-request-id", requestId),
      { kind: "header", name: "clientVersion", serializedName: "x-client-version",
        type: { kind: "constant", value: "1.0" } },
    ],
  });
  assert.deepEqual(op.parameters, [
    { name: "widgetId", wire_name: "widget_id", location: "path",
      type: { kind: "string" }, optional: false, constant: null },
    { name: "searchTerm", wire_name: "search_term", location: "query",
      type: { kind: "string" }, optional: true, constant: null },
    { name: "requestId", wire_name: "x-request-id", location: "header",
      type: { kind: "string" }, optional: false, constant: null },
    { name: "clientVersion", wire_name: "x-client-version", location: "header",
      type: { kind: "string" }, optional: false, constant: "1.0" },
  ]);
  assert.deepEqual(op.status_codes, [200, 204]);
  assert.equal(op.response_type, null);
  assert.equal(op.path, "/widgets/{widget_id}");
  assert.equal(Object.hasOwn(op, "headers"), false);
});

test("rejects missing, nested, ambiguous, and inconsistent method links", () => {
  const source = { kind: "method", name: "id", type: { kind: "string" }, optional: false };
  const original = binding("path", "id", "widget_id", source);
  const run = (overrides, methods = [source]) => operationModel({
    methods, bindings: [{ ...original, ...overrides }],
  });
  assert.throws(() => run({ methodParameterSegments: [] }), /one direct method parameter segment/);
  assert.throws(() => run({ methodParameterSegments: [[source, { kind: "property" }]] }),
    /one direct method parameter segment/);
  assert.throws(() => run({ methodParameterSegments: [[source], [source]] }),
    /one direct method parameter segment/);
  assert.throws(() => run({ correspondingMethodParams: [source, source] }),
    /ambiguous correspondingMethodParams/);
  assert.throws(() => run({ correspondingMethodParams: [{ ...source }] }),
    /ambiguous correspondingMethodParams/);
  assert.throws(() => run({}, []), /not in method.parameters/);
  assert.throws(() => run({ optional: true }), /unsupported method parameter/);
  assert.throws(() => run({ serializedName: "different" }), /path placeholder widget_id/);
  assert.throws(() => run({ style: "matrix" }), /unsupported wire encoding/);
  assert.throws(() => run({ allowReserved: true }), /unsupported wire encoding/);
  assert.throws(() => operationModel({ methods: [source], bindings: [] }),
    /path placeholder widget_id/);
});

test("rejects unsupported wire formats and query optionality", () => {
  const source = { kind: "method", name: "filter", type: { kind: "string" }, optional: true };
  const run = (overrides) => operationModel({ path: "/widgets", methods: [source],
    bindings: [binding("query", "filter", "filter_value", source, overrides)] });
  assert.throws(() => run({ explode: true }), /unsupported wire encoding/);
  assert.throws(() => run({ explode: undefined }), /unsupported wire encoding/);
  assert.throws(() => run({ collectionFormat: "csv" }), /unsupported wire encoding/);
  assert.throws(() => run({ encode: "commaDelimited" }), /unsupported wire encoding/);
  assert.throws(() => run({ optional: false }), /unsupported method parameter/);
  assert.throws(() => run({ serializedName: undefined }), /missing name/);
  assert.throws(() => run({ type: { kind: "array", valueType: source.type } }),
    /unsupported method parameter/);
  assert.throws(() => run({ type: { kind: "enum", name: "Other" } }),
    /unsupported method parameter/);
  assert.throws(() => operationModel({ path: "/widgets", bindings: [{
    kind: "header", name: "bad", serializedName: "x-bad", type: { kind: "constant", value: 42 },
  }] }), /unsupported constant request binding/);
  assert.throws(() => operationModel({ path: "/widgets", bindings: [{
    kind: "header", name: "bad", serializedName: "x-bad",
    type: { kind: "constant", value: "fixed" },
    correspondingMethodParams: [source, source],
  }] }), /ambiguous constant header/);
});

test("rejects unsupported response shapes rather than inferring a default", () => {
  const run = (responses, response) => operationModel({
    path: "/widgets", responses, response,
  });
  assert.throws(() => run([{ statusCodes: { start: 200, end: 299 } }]),
    /unsupported success status codes/);
  assert.throws(() => run([{ statusCodes: "*" }]), /unsupported success status codes/);
  assert.throws(() => run([{ statusCodes: 400 }]), /unsupported success status codes/);
  assert.throws(() => run([{ statusCodes: 200 }, { statusCodes: 200 }]),
    /unsupported success status codes/);
  assert.throws(() => run([{ statusCodes: 200, type: { kind: "string" },
    contentTypes: ["application/json"] },
    { statusCodes: 204 }]), /same type/);
  assert.throws(() => run([{ statusCodes: 200, type: { kind: "bytes" },
    contentTypes: ["application/octet-stream"] }]), /unsupported response content type/);
  assert.throws(() => run([{ statusCodes: 200, type: { kind: "model", name: "Widget" } }]),
    /typed response requires an explicit JSON content type/);
  assert.throws(() => run([{ statusCodes: 200, type: { kind: "string" },
    contentTypes: [] }]), /typed response requires an explicit JSON content type/);
  assert.throws(() => run([{ statusCodes: 200, type: { kind: "string" },
    contentTypes: ["text/plain"] }]), /unsupported response content type/);
  assert.throws(() => run([{ statusCodes: 200, type: { kind: "string" },
    contentTypes: ["application/json", "text/plain"] }]), /unsupported response content type/);
  assert.throws(() => run([{ statusCodes: 200, type: { kind: "string" },
    defaultContentType: "application/xml" }]), /unsupported response content type/);
  assert.throws(() => run([{ statusCodes: 200, type: { kind: "string" },
    contentTypes: ["application/json"] }], { optional: true }), /optional response bodies/);
  assert.deepEqual(run([{ statusCodes: 200, type: { kind: "string" },
    contentTypes: ["application/json"] }]).response_type, { kind: "string" });
  assert.deepEqual(run([{ statusCodes: 200, type: { kind: "model", name: "Widget" },
    defaultContentType: "application/json" }]).response_type, { kind: "model", name: "Widget" });
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
  assert.throws(() => adaptPackage({
    clients: [{ name: "Client", clientInitialization: { parameters: [{ kind: "endpoint", name: "endpoint" }] },
      methods: [{ name: "create", kind: "basic", operation: { verb: "post", path: "/widgets",
        bodyParam: { kind: "body" }, responses: [{ statusCodes: 201 }] } }] }], models: [],
  }), /request bodies are not yet supported/);
});
