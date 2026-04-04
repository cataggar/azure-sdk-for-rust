// TCGC WASI Component — wraps TypeSpec + TCGC for the Rust code generator.
//
// This module is componentized into a WASI component using jco.
// The Rust host invokes `compile()` to get a JSON code model from .tsp files.

/**
 * Compile a TypeSpec project and return the TCGC code model as JSON.
 *
 * @param {string} projectPath - path to the TypeSpec project directory
 * @param {string} emitterOptions - JSON-encoded emitter options
 * @returns {string} JSON-serialized code model
 */
export function compile(projectPath, emitterOptions) {
  // TODO: Phase 1 implementation
  //
  // 1. Use @typespec/compiler to compile the .tsp files
  // 2. Create TCGC SDK context with @azure-tools/typespec-client-generator-core
  // 3. Extract clients, models, enums, operations from the TCGC context
  // 4. Serialize to the code model JSON schema expected by codegen-types
  //
  // For now, return a stub to prove the component boundary works.

  const options = JSON.parse(emitterOptions || "{}");

  const codeModel = {
    name: options["crate-name"] || "azure_generated",
    version: options["crate-version"] || "0.1.0",
    service_type: "azure-arm",
    clients: [],
    models: [],
    enums: [],
    unions: [],
  };

  return JSON.stringify(codeModel, null, 2);
}
