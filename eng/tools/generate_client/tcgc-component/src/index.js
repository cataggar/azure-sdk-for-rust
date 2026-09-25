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

function adaptParameter(param, method, path) {
  const location = param.kind;
  const binding = `${path}.${param.name ?? location}`;
  if (!["path", "query", "header"].includes(location)) {
    throw new Error(`${binding}: unsupported request binding ${location}`);
  }
  const wireName = required(param.serializedName, `${binding}.serializedName`);
  if (param.encode !== undefined || param.collectionFormat !== undefined ||
      (location !== "header" && param.explode !== false) ||
      (location === "header" && param.explode !== undefined) ||
      (location === "path" && (param.style !== "simple" || param.allowReserved !== false))) {
    throw new Error(`${binding}: unsupported wire encoding, style, explode, or collection format`);
  }
  if (param.type?.kind === "constant" && location === "header" &&
      typeof param.type.value === "string") {
    if (param.optional === true || (param.correspondingMethodParams ?? []).length > 1 ||
        (param.methodParameterSegments ?? []).length > 1) {
      throw new Error(`${binding}: optional or ambiguous constant header`);
    }
    return {
      name: required(param.name, binding), wire_name: wireName, location,
      type: { kind: "string" }, optional: false, constant: param.type.value,
    };
  }
  if (param.type?.kind === "constant") {
    throw new Error(`${binding}: unsupported constant request binding`);
  }
  const segments = param.methodParameterSegments;
  if (!Array.isArray(segments) || segments.length !== 1 ||
      !Array.isArray(segments[0]) || segments[0].length !== 1 ||
      segments[0][0]?.kind !== "method") {
    throw new Error(`${binding}: expected exactly one direct method parameter segment`);
  }
  const source = segments[0][0];
  if (!method.parameters?.includes(source)) {
    throw new Error(`${binding}: method parameter segment is not in method.parameters`);
  }
  const corresponding = param.correspondingMethodParams;
  if (corresponding !== undefined &&
      (!Array.isArray(corresponding) || corresponding.length > 1 ||
       (corresponding.length === 1 && corresponding[0] !== source))) {
    throw new Error(`${binding}: ambiguous correspondingMethodParams`);
  }
  if (source.encode !== undefined || source.clientDefaultValue !== undefined ||
      source.onClient === true || source.type?.kind !== param.type?.kind ||
      source.optional !== param.optional ||
      typeof source.optional !== "boolean" ||
      (location === "path" && source.optional)) {
    throw new Error(`${binding}: unsupported method parameter type, default, or optionality`);
  }
  const type = adaptType(source.type, binding);
  if (JSON.stringify(type) !== JSON.stringify(adaptType(param.type, binding))) {
    throw new Error(`${binding}: method and wire parameter types differ`);
  }
  if (!["string", "boolean", "int32", "int64", "float64", "enum"].includes(type.kind)) {
    throw new Error(`${binding}: unsupported request parameter type ${type.kind}`);
  }
  return {
    name: required(source.name, binding), wire_name: wireName, location,
    type, optional: source.optional, constant: null,
  };
}

function validatePathBindings(route, parameters, path) {
  const placeholders = [...route.matchAll(/\{([^{}]+)\}/g)].map((match) => match[1]);
  if (route.replaceAll(/\{[^{}]+\}/g, "").match(/[{}]/) ||
      placeholders.some((name) => !/^[a-zA-Z_][a-zA-Z_0-9]*$/.test(name))) {
    throw new Error(`${path}: unsupported path template`);
  }
  const bindings = parameters.filter((param) => param.location === "path");
  for (const name of new Set([...placeholders, ...bindings.map((param) => param.wire_name)])) {
    if (!placeholders.includes(name) || bindings.filter((param) => param.wire_name === name).length !== 1) {
      throw new Error(`${path}: path placeholder ${name} must have exactly one matching binding`);
    }
  }
}

function adaptResponseType(response, path) {
  const type = response.type ? adaptType(response.type, `${path}.response`) : null;
  if (response.contentTypes !== undefined && !Array.isArray(response.contentTypes)) {
    throw new Error(`${path}: unsupported response content types`);
  }
  const contentTypes = [...(response.contentTypes ?? []),
    ...(response.defaultContentType === undefined ? [] : [response.defaultContentType])];
  if (type && !contentTypes.length) {
    throw new Error(`${path}: typed response requires an explicit JSON content type`);
  }
  for (const contentType of contentTypes) {
    if (!type || typeof contentType !== "string" || contentType.toLowerCase() !== "application/json") {
      throw new Error(`${path}: unsupported response content type ${contentType}`);
    }
  }
  if (response.streamMetadata) throw new Error(`${path}: streaming responses are not supported`);
  return type;
}

function adaptOperation(method, clientPath) {
  const path = `${clientPath}.${required(method.name, clientPath)}`;
  if (method.kind !== "basic") throw new Error(`${path}: unsupported operation kind ${method.kind}`);
  const operation = method.operation;
  if (!operation || !["get", "put", "post", "patch", "delete", "head"].includes(operation.verb)) {
    throw new Error(`${path}: unsupported or missing HTTP verb ${operation?.verb}`);
  }
  if (operation.bodyParam) throw new Error(`${path}: request bodies are not yet supported`);
  const parameters = (operation.parameters ?? []).map((param) => adaptParameter(param, method, path));
  validatePathBindings(required(operation.path, path), parameters, path);
  if ((operation.exceptions ?? []).length || !(operation.responses ?? []).length) {
    throw new Error(`${path}: expected success responses and no explicit exceptions`);
  }
  const codes = [];
  let responseType;
  for (const response of operation.responses) {
    if (!Number.isInteger(response.statusCodes) ||
        response.statusCodes < 200 || response.statusCodes >= 300 ||
        codes.includes(response.statusCodes)) {
      throw new Error(`${path}: unsupported success status codes`);
    }
    codes.push(response.statusCodes);
    const type = adaptResponseType(response, path);
    if (responseType !== undefined && JSON.stringify(responseType) !== JSON.stringify(type)) {
      throw new Error(`${path}: success responses must have the same type`);
    }
    responseType = type;
  }
  if (method.response?.optional && responseType !== null) {
    throw new Error(`${path}: optional response bodies are not supported`);
  }
  return {
    name: method.name,
    cross_language_id: method.crossLanguageDefinitionId ?? null,
    doc: method.doc ?? null,
    http_method: operation.verb.toUpperCase(),
    path: required(operation.path, path),
    parameters,
    status_codes: codes,
    response_type: responseType,
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
    schema_version: 2,
    clients: sdkPackage.clients.map((client) => adaptClient(client)),
    models: sdkPackage.models.map(adaptModel),
    enums: (sdkPackage.enums ?? []).map(adaptEnum),
  };
}
