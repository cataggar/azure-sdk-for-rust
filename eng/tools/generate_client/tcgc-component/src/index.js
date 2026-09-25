// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { createTypeSpecLibrary } from "@typespec/compiler";
import { createSdkContext } from "@azure-tools/typespec-client-generator-core";

export const LIB_NAME = "@azure-tools/typespec-rust-model";
export const $lib = createTypeSpecLibrary({
  name: LIB_NAME,
  diagnostics: {},
  emitter: { options: { type: "object", additionalProperties: false, properties: {} } },
});

export const slot = { model: null, error: null };

export async function $onEmit(context) {
  try {
    const sdk = await createSdkContext(context, LIB_NAME, {
      disableUsageAccessPropagationToBase: true,
    });
    const errors = sdk.diagnostics.filter((d) => d.severity === "error");
    if (errors.length) {
      throw new Error(`TCGC errors:\n${errors.map((d) => `${d.code}: ${d.message}`).join("\n")}`);
    }
    slot.model = adaptPackage(sdk.sdkPackage);
  } catch (error) {
    slot.error = error;
  }
}

function required(value, path) {
  if (typeof value !== "string" || !value) throw new Error(`${path}: missing name`);
  return value;
}

export function adaptType(type, path) {
  if (!type || !type.kind) throw new Error(`${path}: missing type`);
  switch (type.kind) {
    case "string":
    case "boolean":
    case "int32":
    case "int64":
    case "float64":
    case "bytes":
    case "utcDateTime":
      return { kind: type.kind };
    case "model":
    case "enum":
      return { kind: type.kind, name: required(type.name, path) };
    case "array":
    case "dict":
      return { kind: type.kind, value_type: adaptType(type.valueType, path) };
    case "nullable":
      return { kind: "nullable", value_type: adaptType(type.type, path) };
    default:
      throw new Error(`${path}: unsupported type kind ${type.kind}`);
  }
}

function adaptModel(model) {
  const path = `model ${required(model.name, "model")}`;
  if (model.baseModel || model.additionalProperties || model.discriminatorProperty) {
    throw new Error(`${path}: inheritance, additional properties, and discriminators are not supported`);
  }
  return {
    name: model.name,
    cross_language_id: model.crossLanguageDefinitionId ?? null,
    doc: model.doc ?? null,
    fields: (model.properties ?? []).map((field) => ({
      name: required(field.name, path),
      wire_name: required(field.serializedName ?? field.name, path),
      type: adaptType(field.type, `${path}.${field.name}`),
      optional: field.optional === true,
      read_only: field.readOnly === true,
      doc: field.doc ?? null,
    })),
  };
}

function adaptEnum(value) {
  const path = `enum ${required(value.name, "enum")}`;
  if (value.valueType?.kind !== "string") throw new Error(`${path}: only string enums are supported`);
  return {
    name: value.name,
    cross_language_id: value.crossLanguageDefinitionId ?? null,
    extensible: value.isFixed === false,
    values: (value.values ?? []).map((member) => ({
      name: required(member.name, path),
      wire_value: required(member.value, `${path}.${member.name}`),
    })),
  };
}

function adaptOperation(method, clientPath) {
  const path = `${clientPath}.${required(method.name, clientPath)}`;
  if (method.kind !== "basic") throw new Error(`${path}: unsupported operation kind ${method.kind}`);
  const operation = method.operation;
  if (!operation || !["get", "put", "post", "patch", "delete", "head"].includes(operation.verb)) {
    throw new Error(`${path}: unsupported or missing HTTP verb ${operation?.verb}`);
  }
  if (operation.bodyParam) throw new Error(`${path}: request bodies are not yet supported`);
  const headers = (operation.parameters ?? []).map((param) => {
    if (param.kind !== "header" || param.type?.kind !== "constant" ||
        typeof param.type.value !== "string") {
      throw new Error(`${path}: unsupported request binding ${param.kind}:${param.name}`);
    }
    return {
      wire_name: required(param.serializedName ?? param.name, path),
      value: param.type.value,
    };
  });
  if ((operation.exceptions ?? []).length || (operation.responses ?? []).length !== 1) {
    throw new Error(`${path}: expected exactly one success response and no explicit exceptions`);
  }
  const response = operation.responses[0];
  const codes = Array.isArray(response.statusCodes) ? response.statusCodes : [response.statusCodes];
  if (codes.length !== 1 || !Number.isInteger(codes[0])) {
    throw new Error(`${path}: unsupported success status codes`);
  }
  return {
    name: method.name,
    cross_language_id: method.crossLanguageDefinitionId ?? null,
    doc: method.doc ?? null,
    http_method: operation.verb.toUpperCase(),
    path: required(operation.path, path),
    headers,
    status_code: codes[0],
    response_type: response.type ? adaptType(response.type, `${path}.response`) : null,
  };
}

function adaptClient(client, parent = null) {
  const path = `client ${required(client.name, "client")}`;
  const init = client.clientInitialization?.parameters;
  if (!Array.isArray(init) || !init.some((p) => p.kind === "endpoint")) {
    throw new Error(`${path}: endpoint initialization is required`);
  }
  const unsupported = init.filter((p) => p.kind !== "endpoint");
  if (unsupported.length) {
    throw new Error(`${path}: unsupported initialization parameter ${unsupported[0].name} (${unsupported[0].kind})`);
  }
  return {
    name: client.name,
    parent,
    cross_language_id: client.crossLanguageDefinitionId ?? null,
    doc: client.doc ?? null,
    endpoint_name: required(init.find((p) => p.kind === "endpoint").name, path),
    operations: (client.methods ?? []).map((method) => adaptOperation(method, path)),
    children: (client.children ?? []).map((child) => adaptClient(child, client.name)),
  };
}

export function adaptPackage(sdkPackage) {
  if (!sdkPackage || !Array.isArray(sdkPackage.clients) || !Array.isArray(sdkPackage.models)) {
    throw new Error("TCGC did not provide clients and models");
  }
  if ((sdkPackage.unions ?? []).length) {
    throw new Error(`union ${sdkPackage.unions[0].name ?? "<anonymous>"}: unsupported`);
  }
  return {
    schema_version: 1,
    clients: sdkPackage.clients.map((client) => adaptClient(client)),
    models: sdkPackage.models.map(adaptModel),
    enums: (sdkPackage.enums ?? []).map(adaptEnum),
  };
}
