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

export function adaptType(type, path, unions = new Map()) {
  if (!type || !type.kind) throw new Error(`${path}: missing type`);
  switch (type.kind) {
    case "string":
    case "boolean":
    case "int32":
    case "int64":
    case "float64":
      return { kind: type.kind };
    case "bytes":
      throw new Error(`${path}: bytes require a verified model field encoding`);
    case "utcDateTime":
      throw new Error(`${path}: utcDateTime requires a verified model field encoding`);
    case "model":
    case "enum":
      return { kind: type.kind, name: required(type.name, path) };
    case "array":
    case "dict":
      return { kind: type.kind, value_type: adaptType(type.valueType, path, unions) };
    case "nullable": {
      if (type.name && !unions.has(type)) {
        throw new Error(`${path}: unsupported named nullable union ${type.name}`);
      }
      return { kind: "nullable", value_type: adaptType(type.type, path, unions) };
    }
    default:
      throw new Error(`${path}: unsupported type kind ${type.kind}`);
  }
}

function adaptModel(model, unions) {
  const path = `model ${required(model.name, "model")}`;
  if (model.baseModel || model.additionalProperties || model.discriminatorProperty) {
    throw new Error(`${path}: inheritance, additional properties, and discriminators are not supported`);
  }
  return {
    name: model.name,
    cross_language_id: model.crossLanguageDefinitionId ?? null,
    doc: model.doc ?? null,
    fields: (model.properties ?? []).map((field) => {
      const fieldPath = `${path}.${required(field.name, path)}`;
      const wireName = required(field.serializedName ?? field.name, fieldPath);
      let type;
      if (["bytes", "utcDateTime"].includes(field.type?.kind)) {
        const serialization = field.serializationOptions;
        if (field.encode !== undefined || field.clientDefaultValue !== undefined ||
            field.isMultipartFileInput === true ||
            !serialization || Object.keys(serialization).length !== 1 ||
            !serialization.json || Object.keys(serialization.json).length !== 1 ||
            serialization.json.name !== wireName) {
          throw new Error(`${fieldPath}: unsupported ${field.type.kind} field encoding`);
        }
        if (field.type.kind === "bytes") {
          if (field.type.encode !== "base64url") {
            throw new Error(`${fieldPath}: unsupported bytes field encoding`);
          }
          type = { kind: "bytes", encoding: "base64url" };
        } else {
          if (field.type.encode !== "unixTimestamp" ||
              field.type.wireType?.kind !== "int32" ||
              field.type.wireType.name !== "int32") {
            throw new Error(`${fieldPath}: unsupported utcDateTime field encoding`);
          }
          type = { kind: "utcDateTime", format: "unixTime", wire_type: "int32" };
        }
      } else {
        type = adaptType(field.type, fieldPath, unions);
      }
      return {
        name: field.name,
        wire_name: wireName,
        type: field.optional === true && unions.has(field.type) ? type.value_type : type,
        optional: field.optional === true,
        read_only: field.readOnly === true,
        doc: field.doc ?? null,
      };
    }),
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

function adaptParameter(param, method, path, unions, apiVersion) {
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
  if (param.onClient === true || param.isApiVersionParam === true) {
    const segments = param.methodParameterSegments;
    const source = segments?.[0]?.[0];
    if (!apiVersion || param.kind !== "query" || param.serializedName !== "api-version" ||
        param.name !== apiVersion.name || param.type?.kind !== "string" ||
        param.optional !== false || param.onClient !== true || param.isApiVersionParam !== true ||
        param.clientDefaultValue !== apiVersion.clientDefaultValue ||
        !Array.isArray(segments) || segments.length !== 1 ||
        !Array.isArray(segments[0]) || segments[0].length !== 1 ||
        source !== apiVersion || method.parameters?.includes(source) ||
        !Array.isArray(param.correspondingMethodParams) ||
        param.correspondingMethodParams.length !== 1 ||
        param.correspondingMethodParams[0] !== source) {
      throw new Error(`${binding}: unsupported client api-version binding`);
    }
    return {
      name: apiVersion.name, wire_name: wireName, location: "query",
      type: { kind: "string" }, optional: false, constant: null, client_owned: true,
    };
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
      client_owned: false,
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
      typeof source.optional !== "boolean") {
    throw new Error(`${binding}: unsupported method parameter type, default, or optionality`);
  }
  if (location === "path" && source.optional &&
      (source.type.kind !== "string" || param.clientDefaultValue !== undefined ||
       source.defaultValue !== undefined || param.defaultValue !== undefined)) {
    throw new Error(`${binding}: unsupported optional path binding`);
  }
  const type = adaptType(source.type, binding, unions);
  if (JSON.stringify(type) !== JSON.stringify(adaptType(param.type, binding, unions))) {
    throw new Error(`${binding}: method and wire parameter types differ`);
  }
  if (!["string", "boolean", "int32", "int64", "float64", "enum"].includes(type.kind)) {
    throw new Error(`${binding}: unsupported request parameter type ${type.kind}`);
  }
  return {
    name: required(source.name, binding), wire_name: wireName, location,
    type, optional: source.optional, constant: null, client_owned: false,
  };
}

function validatePathBindings(route, parameters, path) {
  if (!route.startsWith("/")) {
    throw new Error(`${path}: unsupported path template`);
  }
  const segments = route.slice(1).split("/");
  const placeholders = [];
  for (const [index, segment] of segments.entries()) {
    if (!segment) {
      if (index !== segments.length - 1) throw new Error(`${path}: unsupported path template`);
    } else if (/^\{[a-zA-Z_][a-zA-Z_0-9]*(?:-[a-zA-Z_0-9]+)*\}$/.test(segment)) {
      placeholders.push(segment.slice(1, -1));
    } else if (segment === "." || segment === ".." ||
        !/^[a-zA-Z0-9._~-]+$/.test(segment)) {
      throw new Error(`${path}: unsupported path template`);
    }
  }
  const bindings = parameters.filter((param) => param.location === "path");
  for (const binding of bindings.filter((param) => param.optional)) {
    if (!route.endsWith(`/{${binding.wire_name}}`)) {
      throw new Error(`${path}: optional path binding must be the final segment`);
    }
  }
  for (const name of new Set([...placeholders, ...bindings.map((param) => param.wire_name)])) {
    if (placeholders.filter((placeholder) => placeholder === name).length !== 1 ||
        bindings.filter((param) => param.wire_name === name).length !== 1) {
      throw new Error(`${path}: path placeholder ${name} must have exactly one matching binding`);
    }
  }
}

function adaptResponseType(response, path, unions) {
  const type = response.type ? adaptType(response.type, `${path}.response`, unions) : null;
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

function adaptBody(body, method, operation, models, path) {
  if (!body) return null;
  const segments = body.methodParameterSegments;
  const source = segments?.[0]?.[0];
  const serialization = body.serializationOptions;
  const contentTypeHeaders = (operation.parameters ?? []).filter(
    (param) => param.serializedName?.toLowerCase() === "content-type");
  if (body.kind !== "body" || body.type?.kind !== "model" ||
      !models.includes(body.type) || !Array.isArray(segments) || segments.length !== 1 ||
      !Array.isArray(segments[0]) || segments[0].length !== 1 ||
      source?.kind !== "method" || !method.parameters?.includes(source) ||
      !Array.isArray(body.correspondingMethodParams) ||
      body.correspondingMethodParams.length !== 1 ||
      body.correspondingMethodParams[0] !== source ||
      body.type !== source.type || body.name !== source.name ||
      body.serializedName !== body.name ||
      typeof body.optional !== "boolean" || body.optional !== source.optional ||
      body.onClient !== false || source.onClient !== false ||
      body.clientDefaultValue !== undefined || source.clientDefaultValue !== undefined ||
      body.encode !== undefined || source.encode !== undefined ||
      body.flatten !== source.flatten ||
      (body.flatten !== true && body.flatten !== false && body.flatten !== undefined) ||
      !serialization || Object.keys(serialization).length !== 1 ||
      !serialization.json || Object.keys(serialization.json).length !== 1 ||
      serialization.json.name !== body.serializedName ||
      !Array.isArray(body.contentTypes) || body.contentTypes.length !== 1 ||
      body.contentTypes[0] !== "application/json" ||
      body.defaultContentType !== "application/json" ||
      contentTypeHeaders.length > 1 ||
      (contentTypeHeaders.length === 1 &&
        (contentTypeHeaders[0].kind !== "header" ||
         contentTypeHeaders[0].type?.kind !== "constant" ||
         contentTypeHeaders[0].type.value !== "application/json" ||
         contentTypeHeaders[0].optional !== false)) ||
      (operation.parameters ?? []).some((param) =>
        param.methodParameterSegments?.some((segment) => segment.includes(source)))) {
    throw new Error(`${path}: unsupported request body binding or JSON encoding`);
  }
  return {
    name: required(source.name, `${path}.body`),
    type: { kind: "model", name: required(source.type.name, `${path}.body`) },
    optional: source.optional,
  };
}

function adaptExceptions(exceptions, models, path) {
  if (exceptions === undefined) return [];
  if (!Array.isArray(exceptions) || exceptions.length > 1) {
    throw new Error(`${path}: unsupported exception responses`);
  }
  return exceptions.map((exception) => {
    const serialization = exception?.serializationOptions;
    if (!exception || exception.kind !== "http" || exception.statusCodes !== "*" ||
        exception.type?.kind !== "model" || !models.includes(exception.type) ||
        !Array.isArray(exception.contentTypes) || exception.contentTypes.length !== 1 ||
        exception.contentTypes[0] !== "application/json" ||
        exception.defaultContentType !== "application/json" ||
        !Array.isArray(exception.headers) || exception.headers.length !== 0 ||
        exception.streamMetadata !== undefined ||
        !serialization || Object.keys(serialization).length !== 1 ||
        !serialization.json || Object.keys(serialization.json).length !== 1 ||
        serialization.json.name !== "" ||
        Object.keys(exception).some((key) => ![
          "__raw", "type", "headers", "contentTypes", "defaultContentType", "apiVersions",
          "description", "streamMetadata", "serializationOptions", "kind", "statusCodes",
        ].includes(key))) {
      throw new Error(`${path}: unsupported exception response or JSON metadata`);
    }
    return {
      status_codes: "*", content_type: "application/json",
      type: { kind: "model", name: required(exception.type.name, `${path}.exception`) },
    };
  });
}

function adaptPaging(method, operation, models, parameters, responseType, apiVersion, path) {
  if (method.kind !== "paging") {
    if (method.pagingMetadata !== undefined) {
      throw new Error(`${path}: unexpected paging metadata on basic operation`);
    }
    return null;
  }
  const metadata = method.pagingMetadata;
  const response = operation.responses?.[0];
  const wrapper = response?.type;
  const link = metadata?.nextLinkSegments?.[0];
  const items = metadata?.pageItemsSegments?.[0];
  const jsonProperty = (property, wireName) =>
    property?.name === wireName && property.serializedName === wireName &&
    property.kind === "property" && property.optional === true &&
    property.clientDefaultValue === undefined &&
    property.serializationOptions &&
    Object.keys(property.serializationOptions).length === 1 &&
    property.serializationOptions.json &&
    Object.keys(property.serializationOptions.json).length === 1 &&
    property.serializationOptions.json.name === wireName;
  if (!metadata || Object.keys(metadata).some((key) => ![
    "__raw", "nextLinkSegments", "nextLinkOperation", "nextLinkVerb",
    "nextLinkReInjectedParametersSegments", "continuationTokenParameterSegments",
    "continuationTokenResponseSegments", "pageItemsSegments", "pageSizeParameterSegments",
  ].includes(key)) ||
      metadata.nextLinkOperation !== undefined || metadata.nextLinkVerb !== "GET" ||
      metadata.continuationTokenParameterSegments !== undefined ||
      metadata.continuationTokenResponseSegments !== undefined ||
      !Array.isArray(metadata.nextLinkReInjectedParametersSegments) ||
      metadata.nextLinkReInjectedParametersSegments.length !== 0 ||
      !Array.isArray(metadata.pageSizeParameterSegments) ||
      metadata.pageSizeParameterSegments.length !== 0 ||
      !Array.isArray(metadata.nextLinkSegments) || metadata.nextLinkSegments.length !== 1 ||
      !Array.isArray(metadata.pageItemsSegments) || metadata.pageItemsSegments.length !== 1 ||
      !jsonProperty(link, "nextLink") || link.type?.kind !== "string" ||
      !jsonProperty(items, "value") || items.type?.kind !== "array" ||
      items.type.valueType?.kind !== "model" ||
      !models.includes(items.type.valueType) ||
      method.response?.kind !== "method" || method.response.optional !== false ||
      method.response.type !== items.type ||
      wrapper?.kind !== "model" || !models.includes(wrapper) ||
      !wrapper.properties?.includes(link) || !wrapper.properties.includes(items) ||
      operation.responses.length !== 1 || response.statusCodes !== 200 ||
      responseType?.kind !== "model" || responseType.name !== wrapper.name ||
      operation.bodyParam !== undefined ||
      operation.verb !== "get" || !apiVersion ||
      parameters.filter((param) => param.client_owned && param.location === "query" &&
        param.wire_name === "api-version" && param.name === apiVersion.name).length !== 1 ||
      parameters.filter((param) => param.location === "header").length !== 1 ||
      !parameters.some((param) => param.location === "header" &&
        param.wire_name.toLowerCase() === "accept" && param.constant === "application/json")) {
    throw new Error(`${path}: unsupported paging metadata or continuation binding`);
  }
  return {
    next_link: { name: link.name, wire_name: link.serializedName, optional: link.optional },
    items: {
      name: items.name, wire_name: items.serializedName, optional: items.optional,
      type: { kind: "array", value_type: {
        kind: "model", name: required(items.type.valueType.name, `${path}.paging.items`),
      } },
    },
    next_link_verb: "GET", reinject_api_version: true,
  };
}

function adaptOperation(method, clientPath, unions, apiVersion, models) {
  const path = `${clientPath}.${required(method.name, clientPath)}`;
  if (!["basic", "paging"].includes(method.kind)) {
    throw new Error(`${path}: unsupported operation kind ${method.kind}`);
  }
  const operation = method.operation;
  if (!operation || !["get", "put", "post", "patch", "delete", "head"].includes(operation.verb)) {
    throw new Error(`${path}: unsupported or missing HTTP verb ${operation?.verb}`);
  }
  const body = adaptBody(operation.bodyParam, method, operation, models, path);
  const parameters = (operation.parameters ?? []).map(
    (param) => adaptParameter(param, method, path, unions, apiVersion));
  for (const clientOwned of parameters.filter((param) => param.client_owned)) {
    if (parameters.some((param) => param !== clientOwned &&
        (param.name === clientOwned.name ||
         (param.location === clientOwned.location && param.wire_name === clientOwned.wire_name)))) {
      throw new Error(`${path}: ambiguous client api-version bindings`);
    }
  }
  validatePathBindings(required(operation.path, path), parameters, path);
  const exceptions = adaptExceptions(operation.exceptions, models, path);
  if (!(operation.responses ?? []).length) {
    throw new Error(`${path}: expected success responses`);
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
    const type = adaptResponseType(response, path, unions);
    if (responseType !== undefined && JSON.stringify(responseType) !== JSON.stringify(type)) {
      throw new Error(`${path}: success responses must have the same type`);
    }
    responseType = type;
  }
  if (method.response?.optional && responseType !== null) {
    throw new Error(`${path}: optional response bodies are not supported`);
  }
  const paging = adaptPaging(method, operation, models, parameters, responseType, apiVersion, path);
  return {
    name: method.name,
    kind: method.kind,
    cross_language_id: method.crossLanguageDefinitionId ?? null,
    doc: method.doc ?? null,
    http_method: operation.verb.toUpperCase(),
    path: required(operation.path, path),
    parameters,
    body,
    status_codes: codes,
    exceptions,
    response_type: responseType,
    paging,
  };
}

function adaptClient(client, unions, models, parent = null) {
  const path = `client ${required(client.name, "client")}`;
  const init = client.clientInitialization?.parameters;
  if (!Array.isArray(init) || !init.some((p) => p.kind === "endpoint")) {
    throw new Error(`${path}: endpoint initialization is required`);
  }
  const unsupported = init.filter((p) => !["endpoint", "credential"].includes(p.kind) &&
    !(p.kind === "method" && p.isApiVersionParam === true));
  if (unsupported.length) {
    throw new Error(`${path}: unsupported initialization parameter ${unsupported[0].name} (${unsupported[0].kind})`);
  }
  const credentials = init.filter((p) => p.kind === "credential");
  let authentication = null;
  if (credentials.length) {
    const credential = credentials[0];
    const scheme = credential.type?.scheme;
    const flow = scheme?.flows?.[0];
    if (credentials.length !== 1 || credential.name !== "credential" ||
        credential.type?.kind !== "credential" ||
        credential.optional !== false || credential.onClient !== true ||
        scheme?.type !== "oauth2" || !Array.isArray(scheme.flows) ||
        scheme.flows.length !== 1 || flow?.type !== "implicit" ||
        Object.keys(flow).some((key) => !["type", "authorizationUrl", "scopes"].includes(key)) ||
        typeof flow.authorizationUrl !== "string" ||
        !/^https:\/\//.test(flow.authorizationUrl) ||
        !Array.isArray(flow.scopes) || flow.scopes.length !== 1 ||
        !flow.scopes[0] || Object.keys(flow.scopes[0]).some((key) => key !== "value") ||
        typeof flow.scopes[0].value !== "string" || !flow.scopes[0].value) {
      throw new Error(`${path}: unsupported credential scheme`);
    }
    authentication = {
      kind: "oauth2", flow: "implicit", authorization_url: flow.authorizationUrl,
      scopes: [flow.scopes[0].value],
    };
  }
  const versions = init.filter((p) => p.kind === "method" && p.isApiVersionParam === true);
  const apiVersion = versions[0];
  if (versions.length > 1 || (apiVersion && (apiVersion.type?.kind !== "string" ||
      apiVersion.optional !== true || apiVersion.onClient !== true ||
      typeof apiVersion.clientDefaultValue !== "string" || !apiVersion.clientDefaultValue ||
      apiVersion.encode !== undefined))) {
    throw new Error(`${path}: unsupported api-version initialization`);
  }
  return {
    name: client.name,
    parent,
    cross_language_id: client.crossLanguageDefinitionId ?? null,
    doc: client.doc ?? null,
    endpoint_name: required(init.find((p) => p.kind === "endpoint").name, path),
    authentication,
    api_version: apiVersion ? {
      name: required(apiVersion.name, path), default: apiVersion.clientDefaultValue,
    } : null,
    operations: (client.methods ?? []).map((method) =>
      adaptOperation(method, path, unions, apiVersion, models)),
    children: (client.children ?? []).map((child) => adaptClient(child, unions, models, client.name)),
  };
}

export function adaptPackage(sdkPackage) {
  if (!sdkPackage || !Array.isArray(sdkPackage.clients) || !Array.isArray(sdkPackage.models)) {
    throw new Error("TCGC did not provide clients and models");
  }
  const unions = new Map();
  const names = new Set([...sdkPackage.models, ...(sdkPackage.enums ?? [])].map((type) => type.name));
  for (const union of sdkPackage.unions ?? []) {
    const name = union?.name ?? "<anonymous>";
    if (!union?.name || union.kind !== "nullable" || union.isGeneratedName !== false ||
        union.type?.kind !== "model" || !sdkPackage.models.includes(union.type) ||
        !union.type.name || name === union.type.name || names.has(name)) {
      throw new Error(`union ${name}: unsupported or colliding nullable model union`);
    }
    names.add(name);
    unions.set(union, union.type);
  }
  return {
    schema_version: 2,
    clients: sdkPackage.clients.map((client) => adaptClient(client, unions, sdkPackage.models)),
    models: sdkPackage.models.map((model) => adaptModel(model, unions)),
    enums: (sdkPackage.enums ?? []).map(adaptEnum),
  };
}
