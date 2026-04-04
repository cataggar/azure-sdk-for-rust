// TCGC WASI Component — wraps TypeSpec + TCGC for the Rust code generator.
//
// This module is componentized into a WASI component using jco.
// The Rust host invokes `compile()` to get a JSON code model from .tsp files.
//
// Can also be run directly with Node.js for development:
//   node src/index.js <project-path> [emitter-options-json]

import { compile as tspCompile } from "@typespec/compiler";
import {
  createSdkContext,
  getAllModels,
  getClient,
  listClients,
  listOperationGroups,
  listOperationsInOperationGroup,
} from "@azure-tools/typespec-client-generator-core";

/**
 * Compile a TypeSpec project and return the TCGC code model as JSON.
 *
 * @param {string} projectPath - path to the TypeSpec project directory
 * @param {string} emitterOptions - JSON-encoded emitter options
 * @returns {string} JSON-serialized code model
 */
export async function compile(projectPath, emitterOptions) {
  const options = JSON.parse(emitterOptions || "{}");

  // Step 1: Compile TypeSpec
  const program = await tspCompile(projectPath, {
    noEmit: true,
  });

  if (program.diagnostics.some((d) => d.severity === "error")) {
    const errors = program.diagnostics
      .filter((d) => d.severity === "error")
      .map((d) => d.message)
      .join("\n");
    throw new Error(`TypeSpec compilation failed:\n${errors}`);
  }

  // Step 2: Create TCGC context
  const sdkContext = await createSdkContext(program, "@azure-tools/typespec-rust");
  const clients = listClients(sdkContext);

  // Step 3: Extract code model
  const codeModel = {
    name: options["crate-name"] || "azure_generated",
    version: options["crate-version"] || "0.1.0",
    service_type: detectServiceType(sdkContext),
    clients: clients.map((c) => adaptClient(sdkContext, c)),
    models: adaptModels(sdkContext),
    enums: adaptEnums(sdkContext),
    unions: [],
  };

  return JSON.stringify(codeModel, null, 2);
}

function detectServiceType(ctx) {
  // Check if any client uses ARM decorators
  return "azure-arm"; // TODO: detect from TypeSpec decorators
}

function adaptClient(ctx, client) {
  const operationGroups = listOperationGroups(ctx, client);
  return {
    name: client.name,
    doc: client.doc || null,
    endpoint: {
      name: "endpoint",
      default_value: null,
    },
    parameters: adaptClientParams(ctx, client),
    methods: [],
    sub_clients: operationGroups.map((og) => ({
      name: toSnakeCase(og.type.name),
      accessor_name: `get_${toSnakeCase(og.type.name)}_client`,
      client_name: og.type.name,
    })),
    credential_scopes: ["https://management.azure.com/.default"],
  };
}

function adaptClientParams(ctx, client) {
  const params = [];
  for (const param of client.initialization?.parameters || []) {
    if (param.kind === "credential" || param.isApiVersionParam) continue;
    params.push({
      name: toSnakeCase(param.name),
      doc: param.doc || null,
      param_type: adaptType(param.type),
      optional: param.optional || false,
    });
  }
  return params;
}

function adaptModels(ctx) {
  const models = [];
  for (const model of getAllModels(ctx)) {
    if (model.kind !== "model") continue;
    models.push({
      name: model.name,
      doc: model.doc || null,
      fields: (model.properties || []).map((p) => ({
        name: toSnakeCase(p.name),
        serialized_name: p.serializedName || p.name,
        doc: p.doc || null,
        field_type: adaptType(p.type),
        optional: p.optional || false,
        read_only: p.visibility?.includes("read") && !p.visibility?.includes("write"),
        flatten: p.flatten || false,
      })),
      parents: [],
      is_input: true,
      is_output: true,
    });
  }
  return models;
}

function adaptEnums(ctx) {
  const enums = [];
  for (const model of getAllModels(ctx)) {
    if (model.kind !== "enum") continue;
    enums.push({
      name: model.name,
      doc: model.doc || null,
      values: (model.values || []).map((v) => ({
        name: v.name,
        value: v.value,
        doc: v.doc || null,
      })),
      value_type: "string",
      extensible: model.isFixed === false,
    });
  }
  return enums;
}

function adaptType(type) {
  if (!type) return { kind: "Scalar", value: "string" };
  switch (type.kind) {
    case "string":
      return { kind: "Scalar", value: "string" };
    case "int32":
      return { kind: "Scalar", value: "i32" };
    case "int64":
      return { kind: "Scalar", value: "i64" };
    case "float32":
      return { kind: "Scalar", value: "f32" };
    case "float64":
      return { kind: "Scalar", value: "f64" };
    case "boolean":
      return { kind: "Scalar", value: "bool" };
    case "bytes":
      return { kind: "Scalar", value: "bytes" };
    case "url":
      return { kind: "Scalar", value: "url" };
    case "model":
      return { kind: "Model", value: type.name };
    case "enum":
      return { kind: "Enum", value: type.name };
    case "union":
      return { kind: "Union", value: type.name };
    case "array":
      return { kind: "Array", value: adaptType(type.valueType) };
    case "nullable":
      return { kind: "Option", value: adaptType(type.type) };
    default:
      return { kind: "Scalar", value: "string" };
  }
}

function toSnakeCase(str) {
  return str
    .replace(/([A-Z])/g, "_$1")
    .toLowerCase()
    .replace(/^_/, "");
}

// CLI mode: run directly with node
if (typeof process !== "undefined" && process.argv[1]?.endsWith("index.js")) {
  const args = process.argv.slice(2);
  if (args.length < 1) {
    console.error("Usage: node src/index.js <project-path> [emitter-options-json]");
    process.exit(1);
  }
  compile(args[0], args[1] || "{}").then(
    (json) => console.log(json),
    (err) => {
      console.error(err);
      process.exit(1);
    }
  );
}

