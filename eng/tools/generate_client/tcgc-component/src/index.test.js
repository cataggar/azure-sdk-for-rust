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
  assert.equal(model.clients[0].authentication, null);
  assert.equal(model.clients[0].api_version, null);
  assert.deepEqual(model.clients[0].operations[0].parameters, []);
  assert.equal(model.clients[0].operations[0].kind, "basic");
  assert.equal(model.clients[0].operations[0].paging, null);
  assert.equal(model.clients[0].operations[0].body, null);
  assert.deepEqual(model.clients[0].operations[0].status_codes, [200]);
  assert.deepEqual(model.clients[0].operations[0].exceptions, []);
  assert.deepEqual(model.clients[0].operations[0].response_type, { kind: "model", name: "Widget" });
  assert.deepEqual(model.models[0].fields[0], {
    name: "displayName", wire_name: "display_name", type: { kind: "string" },
    optional: true, read_only: true, doc: null,
  });
});

test("preserves only verified JSON base64url encoding on direct bytes fields", () => {
  const field = { name: "secretBackup", serializedName: "value", optional: true,
    type: { kind: "bytes", encode: "base64url" },
    serializationOptions: { json: { name: "value" } } };
  const run = (candidate) => adaptPackage({
    clients: [], models: [{ name: "RestoreSecretParameters", properties: [candidate] }],
  }).models[0].fields[0];
  assert.deepEqual(run(field), {
    name: "secretBackup", wire_name: "value",
    type: { kind: "bytes", encoding: "base64url" },
    optional: true, read_only: false, doc: null,
  });
  assert.deepEqual(run({ ...field, optional: false }).type,
    { kind: "bytes", encoding: "base64url" });
  for (const candidate of [
    { ...field, type: { kind: "bytes" } },
    { ...field, type: { kind: "bytes", encode: "base64" } },
    { ...field, encode: "base64url" },
    { ...field, serializationOptions: undefined },
    { ...field, serializationOptions: { json: { name: "other" } } },
    { ...field, serializationOptions: { json: { name: "value", format: "raw" } } },
    { ...field, clientDefaultValue: "" },
    { ...field, isMultipartFileInput: true },
  ]) {
    assert.throws(() => run(candidate), /unsupported bytes field encoding/);
  }
  assert.throws(() => adaptType({ kind: "bytes", encode: "base64url" }, "body"),
    /bytes require a verified model field encoding/);
  assert.throws(() => adaptType({ kind: "array", valueType: {
    kind: "bytes", encode: "base64url",
  } }, "nested"), /bytes require a verified model field encoding/);
});

test("preserves only explicit int32 Unix timestamps on direct time fields", () => {
  const field = { name: "deletedDate", serializedName: "deletedDate", optional: true,
    type: { kind: "utcDateTime", encode: "unixTimestamp",
      wireType: { kind: "int32", name: "int32" } },
    serializationOptions: { json: { name: "deletedDate" } } };
  const run = (candidate) => adaptPackage({
    clients: [], models: [{ name: "DeletedSecret", properties: [candidate] }],
  }).models[0].fields[0];
  assert.deepEqual(run(field).type,
    { kind: "utcDateTime", format: "unixTime", wire_type: "int32" });
  for (const candidate of [
    { ...field, type: { kind: "utcDateTime" } },
    { ...field, type: { ...field.type, encode: "rfc3339" } },
    { ...field, type: { ...field.type, wireType: undefined } },
    { ...field, type: { ...field.type, wireType: { kind: "int64", name: "int64" } } },
    { ...field, encode: "unixTimestamp" },
    { ...field, serializationOptions: undefined },
    { ...field, serializationOptions: { json: { name: "other" } } },
    { ...field, serializationOptions: { json: { name: "deletedDate", format: "string" } } },
    { ...field, clientDefaultValue: 0 },
  ]) {
    assert.throws(() => run(candidate), /unsupported utcDateTime field encoding/);
  }
  assert.throws(() => adaptType(field.type, "response"),
    /utcDateTime requires a verified model field encoding/);
  assert.throws(() => adaptType({ kind: "array", valueType: field.type }, "nested"),
    /utcDateTime requires a verified model field encoding/);
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
      type: { kind: "string" }, optional: false, constant: null, client_owned: false },
    { name: "searchTerm", wire_name: "search_term", location: "query",
      type: { kind: "string" }, optional: true, constant: null, client_owned: false },
    { name: "requestId", wire_name: "x-request-id", location: "header",
      type: { kind: "string" }, optional: false, constant: null, client_owned: false },
    { name: "clientVersion", wire_name: "x-client-version", location: "header",
      type: { kind: "string" }, optional: false, constant: "1.0", client_owned: false },
  ]);
  assert.deepEqual(op.status_codes, [200, 204]);
  assert.equal(op.response_type, null);
  assert.equal(op.path, "/widgets/{widget_id}");
  assert.equal(Object.hasOwn(op, "headers"), false);
});

test("preserves a whole-segment hyphenated path name without changing wire encoding", () => {
  const source = { kind: "method", name: "secretName", type: { kind: "string" }, optional: false };
  const wire = binding("path", "secretName", "secret-name", source);
  const run = (route) => operationModel({ path: route, methods: [source], bindings: [wire] });
  const op = run("/secrets/{secret-name}");
  assert.equal(op.path, "/secrets/{secret-name}");
  assert.deepEqual(op.parameters[0], {
    name: "secretName", wire_name: "secret-name", location: "path",
    type: { kind: "string" }, optional: false, constant: null, client_owned: false,
  });
  for (const route of [
    "/secrets/prefix-{secret-name}", "/secrets/{secret-name}.json",
    "/secrets/{secret-name}/{secret-name}", "/secrets/{secret-name}/..",
    "/secrets//{secret-name}", "/secrets/{secret-name}?query=1",
  ]) {
    assert.throws(() => run(route), /unsupported path template|must have exactly one matching binding/);
  }
});

test("retains a terminal optional path segment and rejects ambiguous omission", () => {
  const name = { kind: "method", name: "secretName", type: { kind: "string" }, optional: false };
  const version = { kind: "method", name: "secretVersion", type: { kind: "string" }, optional: true };
  const nameBinding = binding("path", "secretName", "secret-name", name);
  const versionBinding = binding("path", "secretVersion", "secret-version", version);
  const run = (path, overrides = {}) => operationModel({
    path, methods: [name, version],
    bindings: [nameBinding, { ...versionBinding, ...overrides }],
  });
  const op = run("/secrets/{secret-name}/{secret-version}");
  assert.equal(op.path, "/secrets/{secret-name}/{secret-version}");
  assert.deepEqual(op.parameters[1], {
    name: "secretVersion", wire_name: "secret-version", location: "path",
    type: { kind: "string" }, optional: true, constant: null, client_owned: false,
  });
  assert.throws(() => run("/secrets/{secret-version}/{secret-name}"),
    /optional path binding must be the final segment/);
  assert.throws(() => run("/secrets/{secret-name}/{secret-version}/items"),
    /optional path binding must be the final segment/);
  assert.throws(() => run("/secrets/{secret-name}/{secret-version}",
    { clientDefaultValue: "" }), /unsupported optional path binding/);
  assert.throws(() => run("/secrets/{secret-name}/{secret-version}",
    { defaultValue: "latest" }), /unsupported optional path binding/);
  const numericVersion = { ...version, type: { kind: "int32" } };
  assert.throws(() => operationModel({
    path: "/secrets/{secret-version}", methods: [numericVersion],
    bindings: [binding("path", "secretVersion", "secret-version", numericVersion)],
  }), /unsupported optional path binding/);
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
    contentTypes: ["application/octet-stream"] }]), /bytes require a verified model field encoding/);
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
  }), /client Client: unsupported credential scheme/);
  assert.throws(() => adaptPackage({
    clients: [{ name: "Client", clientInitialization: { parameters: [{ kind: "endpoint", name: "endpoint" }] },
      methods: [{ name: "list", kind: "paging" }] }], models: [],
  }), /client Client\.list: unsupported or missing HTTP verb/);
  assert.throws(() => adaptPackage({
    clients: [{ name: "Client", clientInitialization: { parameters: [{ kind: "endpoint", name: "endpoint" }] },
      methods: [{ name: "create", kind: "basic", operation: { verb: "post", path: "/widgets",
        bodyParam: { kind: "body" }, responses: [{ statusCodes: 201 }] } }] }], models: [],
  }), /unsupported request body binding or JSON encoding/);
});

test("lowers only a direct JSON model body linked to its method source", () => {
  const model = { kind: "model", name: "Widget", properties: [] };
  const source = { kind: "method", name: "parameters", type: model, optional: false,
    onClient: false, flatten: true };
  const body = { kind: "body", name: "parameters", serializedName: "parameters", type: model,
    optional: false, onClient: false, flatten: true,
    contentTypes: ["application/json"], defaultContentType: "application/json",
    serializationOptions: { json: { name: "parameters" } },
    methodParameterSegments: [[source]], correspondingMethodParams: [source] };
  const header = { kind: "header", name: "contentType", serializedName: "Content-Type",
    type: { kind: "constant", value: "application/json" }, optional: false };
  const client = { name: "Client",
    clientInitialization: { parameters: [{ kind: "endpoint", name: "endpoint" }] },
    methods: [{ kind: "basic", name: "create", parameters: [source], operation: {
      verb: "post", path: "/widgets", parameters: [header], bodyParam: body,
      responses: [{ statusCodes: 201 }],
    } }],
  };
  const run = (candidate) => adaptPackage({ clients: [candidate], models: [model] });
  const result = run(client).clients[0].operations[0];
  assert.deepEqual(result.body, {
    name: "parameters", type: { kind: "model", name: "Widget" }, optional: false,
  });
  assert.deepEqual(result.parameters, [{
    name: "contentType", wire_name: "Content-Type", location: "header",
    type: { kind: "string" }, optional: false, constant: "application/json", client_owned: false,
  }]);
  const withoutHeader = run({ ...client, methods: [{
    ...client.methods[0], operation: { ...client.methods[0].operation, parameters: [] },
  }] }).clients[0].operations[0];
  assert.deepEqual(withoutHeader.body, result.body);
  assert.deepEqual(withoutHeader.parameters, []);
  const invalid = (overrides, params = [header]) => run({ ...client, methods: [{
    ...client.methods[0], operation: { ...client.methods[0].operation,
      parameters: params, bodyParam: { ...body, ...overrides } },
  }] });
  assert.throws(() => invalid({ contentTypes: ["text/plain"] }), /unsupported request body/);
  assert.throws(() => invalid({ defaultContentType: "application/xml" }), /unsupported request body/);
  assert.throws(() => invalid({ serializationOptions: { json: { name: "other" } } }),
    /unsupported request body/);
  assert.throws(() => invalid({ encode: "base64" }), /unsupported request body/);
  assert.throws(() => invalid({ clientDefaultValue: model }), /unsupported request body/);
  assert.throws(() => invalid({ type: { ...model } }), /unsupported request body/);
  assert.throws(() => invalid({ methodParameterSegments: [[{ ...source }]] }),
    /unsupported request body/);
  assert.throws(() => invalid({ optional: true }), /unsupported request body/);
  assert.throws(() => invalid({ contentTypes: [] }, []), /unsupported request body/);
  assert.throws(() => invalid({ defaultContentType: undefined }, []), /unsupported request body/);
  assert.throws(() => invalid({ serializationOptions: {} }, []), /unsupported request body/);
  assert.throws(() => invalid({}, [{ ...header,
    type: { kind: "constant", value: "text/plain" } }]), /unsupported request body/);
  assert.throws(() => invalid({}, [{ ...header,
    type: { kind: "string" } }]), /unsupported request body/);
  assert.throws(() => invalid({}, [{ ...header,
    optional: true }]), /unsupported request body/);
  assert.throws(() => invalid({}, [header, { ...header, name: "duplicate" }]),
    /unsupported request body/);
});

test("lowers a registered nullable model union and its recursive optional fields", () => {
  const errorModel = { kind: "model", name: "KeyVaultErrorError", properties: [] };
  const error = { kind: "nullable", name: "Error", isGeneratedName: false, type: errorModel };
  errorModel.properties = [
    { name: "code", type: { kind: "string" }, optional: true },
    { name: "innerError", type: error, optional: true },
  ];
  const packageModel = adaptPackage({
    clients: [],
    models: [errorModel, { name: "KeyVaultError", properties: [
      { name: "error", type: error, optional: true },
      { name: "requiredError", type: error, optional: false },
    ] }],
    unions: [error],
  });
  assert.equal(packageModel.schema_version, 2);
  assert.deepEqual(packageModel.models[0].fields[1].type,
    { kind: "model", name: "KeyVaultErrorError" });
  assert.deepEqual(packageModel.models[1].fields[0].type,
    { kind: "model", name: "KeyVaultErrorError" });
  assert.deepEqual(packageModel.models[1].fields[1].type,
    { kind: "nullable", value_type: { kind: "model", name: "KeyVaultErrorError" } });
  assert.deepEqual(packageModel.models.map((model) => model.name),
    ["KeyVaultErrorError", "KeyVaultError"]);
});

test("rejects unions without a unique registered nullable model target", () => {
  const model = { kind: "model", name: "ErrorBody", properties: [] };
  const union = { kind: "nullable", name: "Error", isGeneratedName: false, type: model };
  const run = (candidate, models = [model], enums = []) =>
    adaptPackage({ clients: [], models, enums, unions: [candidate] });
  assert.throws(() => run({ ...union, kind: "union" }), /union Error: unsupported/);
  assert.throws(() => run({ ...union, type: { kind: "string" } }), /union Error: unsupported/);
  assert.throws(() => run({ ...union, type: { ...model } }), /union Error: unsupported/);
  assert.throws(() => run({ ...union, name: undefined }), /union <anonymous>: unsupported/);
  assert.throws(() => run({ ...union, isGeneratedName: true }), /union Error: unsupported/);
  assert.throws(() => run(union, [model, { name: "Error" }]), /union Error: unsupported/);
  assert.throws(() => run(union, [model], [{ name: "Error" }]), /union Error: unsupported/);
  assert.throws(() => adaptPackage({ clients: [], models: [model], unions: [union, union] }),
    /union Error: unsupported/);
  assert.throws(() => adaptPackage({ clients: [], models: [
    model, { name: "Wrapper", properties: [{ name: "error", type: { ...union } }] },
  ], unions: [union] }), /unsupported named nullable union Error/);
});

test("does not erase explicit exceptions when lowering a nullable error union", () => {
  const model = { kind: "model", name: "ErrorBody", properties: [] };
  const union = { kind: "nullable", name: "Error", isGeneratedName: false, type: model };
  assert.throws(() => adaptPackage({
    models: [model], unions: [union],
    clients: [{ name: "Client",
      clientInitialization: { parameters: [{ kind: "endpoint", name: "endpoint" }] },
      methods: [{ name: "get", kind: "basic", operation: {
        verb: "get", path: "/resource", responses: [{ statusCodes: 200 }],
        exceptions: [{ statusCodes: 400, type: union }],
      } }],
    }],
  }), /unsupported exception response or JSON metadata/);
});

test("retains only a verified catch-all JSON model exception", () => {
  const model = { kind: "model", name: "KeyVaultError", properties: [] };
  const exception = { kind: "http", statusCodes: "*", type: model, headers: [],
    contentTypes: ["application/json"], defaultContentType: "application/json",
    serializationOptions: { json: { name: "" } } };
  const operation = { verb: "get", path: "/resource", responses: [{ statusCodes: 200 }],
    exceptions: [exception] };
  const run = (exceptions) => adaptPackage({
    models: [model], clients: [{ name: "Client",
      clientInitialization: { parameters: [{ kind: "endpoint", name: "endpoint" }] },
      methods: [{ name: "get", kind: "basic", operation: { ...operation, exceptions } }],
    }],
  }).clients[0].operations[0];
  assert.deepEqual(run([exception]).exceptions, [{
    status_codes: "*", content_type: "application/json",
    type: { kind: "model", name: "KeyVaultError" },
  }]);
  for (const candidate of [
    { ...exception, statusCodes: 400 },
    { ...exception, type: { ...model } },
    { ...exception, headers: [{ name: "x-error" }] },
    { ...exception, contentTypes: ["application/xml"] },
    { ...exception, defaultContentType: "application/xml" },
    { ...exception, serializationOptions: { json: { name: "error" } } },
    { ...exception, serializationOptions: { json: { name: "" }, xml: {} } },
    { ...exception, streamMetadata: {} },
    { ...exception, unexpectedEncoding: "xml" },
  ]) {
    assert.throws(() => run([candidate]), /unsupported exception response or JSON metadata/);
  }
  assert.throws(() => run([exception, exception]), /unsupported exception responses/);
  assert.throws(() => run([null]), /unsupported exception response or JSON metadata/);
});

function credentialClient({ scheme = { type: "oauth2", flows: [{
  type: "implicit", authorizationUrl: "https://login.example.com/authorize",
  scopes: [{ value: "https://vault.example.com/.default" }],
}] }, version = true, binding = true } = {}) {
  const apiVersion = { name: "apiVersion", kind: "method", type: { kind: "string" },
    optional: true, onClient: true, isApiVersionParam: true, clientDefaultValue: "2026-05-01" };
  const wireVersion = { name: "apiVersion", kind: "query", serializedName: "api-version",
    type: { kind: "string" }, optional: false, onClient: true, isApiVersionParam: true,
    clientDefaultValue: apiVersion.clientDefaultValue, explode: false,
    methodParameterSegments: [[apiVersion]], correspondingMethodParams: [apiVersion] };
  const client = { name: "Client", clientInitialization: { parameters: [
    { kind: "endpoint", name: "endpoint" },
    { kind: "credential", name: "credential", onClient: true, optional: false,
      type: { kind: "credential", scheme } },
    ...(version ? [apiVersion] : []),
  ] }, methods: [{ name: "get", kind: "basic", parameters: [],
    operation: { verb: "get", path: "/resource",
      parameters: binding ? [wireVersion] : [], responses: [{ statusCodes: 200 }] } }] };
  return { client, apiVersion, wireVersion };
}

test("preserves a single implicit OAuth2 flow and verified client api-version binding", () => {
  const { client } = credentialClient();
  const result = adaptPackage({ clients: [client], models: [] }).clients[0];
  assert.deepEqual(result.authentication, {
    kind: "oauth2", flow: "implicit", authorization_url: "https://login.example.com/authorize",
    scopes: ["https://vault.example.com/.default"],
  });
  assert.deepEqual(result.api_version, { name: "apiVersion", default: "2026-05-01" });
  assert.deepEqual(result.operations[0].parameters, [{
    name: "apiVersion", wire_name: "api-version", location: "query",
    type: { kind: "string" }, optional: false, constant: null, client_owned: true,
  }]);
});

test("same-name method parameter stays method-owned alongside client api-version", () => {
  const { client, apiVersion, wireVersion } = credentialClient({ binding: false });
  const source = { name: "apiVersion", kind: "method", type: { kind: "string" }, optional: false };
  const methodBinding = {
    ...wireVersion, serializedName: "method-version", onClient: false, isApiVersionParam: false,
    clientDefaultValue: undefined, methodParameterSegments: [[source]],
    correspondingMethodParams: [source],
  };
  const method = client.methods[0];
  const methodOwned = {
    ...client, methods: [{ ...method, parameters: [source],
      operation: { ...method.operation, parameters: [methodBinding] } }],
  };
  const operation = adaptPackage({ clients: [methodOwned], models: [] }).clients[0].operations[0];
  assert.deepEqual(operation.parameters, [{
    name: "apiVersion", wire_name: "method-version", location: "query",
    type: { kind: "string" }, optional: false, constant: null, client_owned: false,
  }]);
  assert.equal(methodOwned.clientInitialization.parameters[2], apiVersion);
  const ambiguous = { ...methodOwned, methods: [{ ...methodOwned.methods[0],
    operation: { ...method.operation, parameters: [methodBinding, wireVersion] } }] };
  assert.throws(() => adaptPackage({ clients: [ambiguous], models: [] }),
    /ambiguous client api-version bindings/);
});

test("rejects unsupported credential schemes and api-version binding shortcuts", () => {
  const run = (client) => adaptPackage({ clients: [client], models: [] });
  const { client, apiVersion, wireVersion } = credentialClient();
  const credential = client.clientInitialization.parameters[1];
  for (const scheme of [
    { type: "bearer" }, { type: "apiKey" }, { type: "union" },
    { type: "oauth2", flows: [] },
    { type: "oauth2", flows: [{ type: "authorizationCode", scopes: [{ value: "scope" }] }] },
    { type: "oauth2", flows: [{ type: "implicit", authorizationUrl: "http://example.com",
      scopes: [{ value: "scope" }] }] },
    { type: "oauth2", flows: [{ type: "implicit", authorizationUrl: "https://example.com",
      scopes: [{ value: "one" }, { value: "two" }] }] },
  ]) {
    assert.throws(() => run({ ...client, clientInitialization: { parameters: [
      client.clientInitialization.parameters[0], { ...credential, type: { kind: "credential", scheme } },
      apiVersion,
    ] } }), /unsupported credential scheme/);
  }
  assert.throws(() => run({ ...client, clientInitialization: { parameters: [
    ...client.clientInitialization.parameters, credential,
  ] } }), /unsupported credential scheme/);
  assert.throws(() => run({ ...client, clientInitialization: { parameters: [
    ...client.clientInitialization.parameters, { kind: "custom", name: "extra" },
  ] } }), /unsupported initialization parameter extra/);
  assert.throws(() => run({ ...client, clientInitialization: { parameters: [
    ...client.clientInitialization.parameters, apiVersion,
  ] } }), /unsupported api-version initialization/);
  assert.throws(() => run(credentialClient({ version: false }).client),
    /unsupported client api-version binding/);
  const binding = (overrides) => run({
    ...client, methods: [{ ...client.methods[0], operation: {
      ...client.methods[0].operation,
      parameters: [{ ...wireVersion, ...overrides }],
    } }],
  });
  assert.throws(() => binding({ clientDefaultValue: "2025-01-01" }), /unsupported client api-version binding/);
  assert.throws(() => binding({ methodParameterSegments: [[{ ...apiVersion }]] }),
    /unsupported client api-version binding/);
  assert.throws(() => binding({ correspondingMethodParams: [] }),
    /unsupported client api-version binding/);
  assert.throws(() => binding({ serializedName: "version" }),
    /unsupported client api-version binding/);
  assert.throws(() => binding({ onClient: false }), /unsupported client api-version binding/);
  assert.throws(() => run({ ...client, clientInitialization: { parameters: [
    client.clientInitialization.parameters[0], credential, { ...apiVersion, clientDefaultValue: undefined },
  ] } }), /unsupported api-version initialization/);
});

test("retains paging wrapper, item identity, next link and continuation requirements", () => {
  const item = { kind: "model", name: "SecretProperties", properties: [] };
  const itemsType = { kind: "array", valueType: item };
  const link = { kind: "property", name: "nextLink", serializedName: "nextLink",
    type: { kind: "string" }, optional: true,
    serializationOptions: { json: { name: "nextLink" } } };
  const value = { kind: "property", name: "value", serializedName: "value",
    type: itemsType, optional: true, serializationOptions: { json: { name: "value" } } };
  const wrapper = { kind: "model", name: "ListSecretPropertiesResult",
    properties: [link, value] };
  const pagingMetadata = {
    nextLinkSegments: [link], nextLinkVerb: "GET",
    pageItemsSegments: [value], pageSizeParameterSegments: [],
    nextLinkReInjectedParametersSegments: [],
  };
  const { client } = credentialClient();
  const method = client.methods[0];
  const operation = { ...method.operation, parameters: [...method.operation.parameters,
    { kind: "header", name: "accept", serializedName: "Accept",
      type: { kind: "constant", value: "application/json" }, optional: false }],
  responses: [{ statusCodes: 200, type: wrapper, contentTypes: ["application/json"] }] };
  const paging = { ...method, kind: "paging", pagingMetadata,
    response: { kind: "method", optional: false, type: itemsType }, operation };
  const run = (overrides = {}) => adaptPackage({
    clients: [{ ...client, methods: [{ ...paging, ...overrides }] }],
    models: [item, wrapper],
  }).clients[0].operations[0];
  const result = run();
  assert.equal(result.kind, "paging");
  assert.deepEqual(result.response_type, { kind: "model", name: "ListSecretPropertiesResult" });
  assert.deepEqual(result.paging, {
    next_link: { name: "nextLink", wire_name: "nextLink", optional: true },
    items: { name: "value", wire_name: "value", optional: true,
      type: { kind: "array", value_type: { kind: "model", name: "SecretProperties" } } },
    next_link_verb: "GET", reinject_api_version: true,
  });
  assert.equal(result.parameters[0].client_owned, true);
  assert.equal(result.parameters[1].constant, "application/json");
  for (const invalid of [
    { nextLinkOperation: { name: "getNext" } },
    { nextLinkVerb: "POST" },
    { continuationTokenParameterSegments: [] },
    { continuationTokenResponseSegments: [] },
    { nextLinkReInjectedParametersSegments: [[link]] },
    { pageSizeParameterSegments: [value] },
    { nextLinkSegments: [{ ...link }] },
    { pageItemsSegments: [{ ...value }] },
    { unexpectedMode: true },
  ]) {
    assert.throws(() => run({ pagingMetadata: { ...pagingMetadata, ...invalid } }),
      /unsupported paging metadata or continuation binding/);
  }
  assert.throws(() => run({ response: { ...paging.response, type: { ...itemsType } } }),
    /unsupported paging metadata or continuation binding/);
  assert.throws(() => run({ kind: "basic" }), /unexpected paging metadata on basic operation/);
  assert.throws(() => run({ operation: { ...operation, responses: [
    { ...operation.responses[0], statusCodes: 201 },
  ] } }), /unsupported paging metadata or continuation binding/);
});
