// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import { compile as tspCompile, resolvePath } from "@typespec/compiler";
import { jsImports } from "../dist/wasi-entry.generated.js";
import { $lib, $onEmit, LIB_NAME, slot } from "./index.js";

const emitterPackage = `/node_modules/${LIB_NAME}`;
const emitterManifest = JSON.stringify({
  name: LIB_NAME, version: "0.1.0", type: "module",
  main: "src/index.js", tspMain: "src/index.js",
});

function fsError(path) {
  return Object.assign(new Error(`ENOENT: ${path}`), { code: "ENOENT" });
}

export function validateFiles(files) {
  if (!files || typeof files !== "object" || Array.isArray(files)) {
    throw new Error("__spec_files must be a map of virtual paths to UTF-8 strings");
  }
  const sources = new Map([[`${emitterPackage}/package.json`, emitterManifest]]);
  for (const [path, content] of Object.entries(files)) {
    if (!/^\/(spec|node_modules)\/[^\\]+$/.test(path) ||
        path.split("/").some((part) => part === "." || part === "..") ||
        path.includes("//") || typeof content !== "string" ||
        path === `${emitterPackage}/package.json`) {
      throw new Error(`invalid virtual file: ${path}`);
    }
    sources.set(path, content);
  }
  return sources;
}

function hostFor(sources) {
  return {
    async readUrl() { throw new Error("readUrl is unsupported"); },
    async readFile(path) {
      if (!sources.has(path)) throw fsError(path);
      return { text: sources.get(path), path, kind: path.endsWith(".tsp") ? "typespec" : "js" };
    },
    async writeFile() { throw new Error("writeFile is unsupported"); },
    async readDir(path) {
      const prefix = `${path}/`;
      if (![...sources.keys()].some((key) => key.startsWith(prefix))) throw fsError(path);
      return [...new Set([...sources.keys()].filter((key) => key.startsWith(prefix))
        .map((key) => key.slice(prefix.length).split("/")[0]))].sort();
    },
    async stat(path) {
      if (sources.has(path) || jsImports.has(path) || path === `${emitterPackage}/src/index.js`) {
        return { isFile: () => true, isDirectory: () => false };
      }
      if ([...sources.keys()].some((key) => key.startsWith(`${path}/`))) {
        return { isFile: () => false, isDirectory: () => true };
      }
      throw fsError(path);
    },
    async realpath(path) { return path; },
    getExecutionRoot: () => "/node_modules/@typespec/compiler",
    getLibDirs: () => ["/node_modules/@typespec/compiler/lib/std"],
    async getJsImport(path) {
      const module = jsImports.get(path.replace(/^file:\/\//, ""));
      if (!module) throw new Error(`JS module not bundled: ${path}`);
      return module;
    },
    getSourceFileKind: (path) => path.endsWith(".tsp") ? "typespec" : path.endsWith(".js") ? "js" : undefined,
    fileURLToPath: (url) => url.replace(/^file:\/\//, ""),
    pathToFileURL: (path) => `file://${path}`,
    logSink: { log() {} },
    async mkdirp() { throw new Error("mkdirp is unsupported"); },
    async rm() { throw new Error("rm is unsupported"); },
  };
}

export const tcgc = {
  async compile(projectPath, emitterOptions) {
    try {
      return await compileInner(projectPath, emitterOptions);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      // componentize-js marshals thrown plain payloads to result::err.
      throw { payload: message };
    }
  },
};

async function compileInner(projectPath, emitterOptions) {
  jsImports.set(`${emitterPackage}/src/index.js`, { $lib, $onEmit });
  if (typeof projectPath !== "string" || !/^\/spec(?:\/[^\\]+)?$/.test(projectPath) ||
      projectPath.split("/").some((part) => part === "." || part === "..")) {
    throw new Error(`invalid project path: ${projectPath}`);
  }
  const options = JSON.parse(emitterOptions);
  if (!options || typeof options !== "object" || Array.isArray(options) ||
      !Object.hasOwn(options, "__spec_files") || Object.keys(options).some((key) => key !== "__spec_files")) {
    throw new Error("emitter options must contain only __spec_files");
  }
  const sources = validateFiles(options.__spec_files);
  const host = hostFor(sources);
  let mainFile;
  for (const name of ["client.tsp", "main.tsp"]) {
    const path = resolvePath(projectPath, name);
    if (sources.has(path)) { mainFile = path; break; }
  }
  if (!mainFile) throw new Error(`missing client.tsp or main.tsp in ${projectPath}`);

  slot.model = null;
  slot.error = null;
  const program = await tspCompile(host, mainFile, {
    emit: [emitterPackage],
    options: { [LIB_NAME]: {} },
    noEmit: false,
    warningAsError: false,
  });
  const errors = program.diagnostics.filter((d) => d.severity === "error");
  if (errors.length) {
    throw new Error(`TypeSpec compilation failed:\n${errors.map((d) => `${d.code}: ${d.message}`).join("\n")}`);
  }
  if (slot.error) throw slot.error;
  if (!slot.model) throw new Error("TCGC emitter produced no code model");
  return JSON.stringify(slot.model);
}

export const compile = tcgc.compile;
