// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import * as fs from "node:fs";
import * as path from "node:path";
import { fileURLToPath } from "node:url";
import { execFileSync } from "node:child_process";
import * as esbuild from "esbuild";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const dist = path.join(root, "dist");
const packageNames = [
  "@typespec/compiler", "@typespec/http", "@typespec/rest", "@typespec/versioning",
  "@typespec/openapi", "@typespec/xml", "@typespec/events", "@typespec/sse",
  "@typespec/streams", "@azure-tools/typespec-azure-core",
  "@azure-tools/typespec-azure-resource-manager",
  "@azure-tools/typespec-client-generator-core",
];

function* walk(dir) {
  if (!fs.existsSync(dir)) return;
  for (const entry of fs.readdirSync(dir, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const file = path.join(dir, entry.name);
    if (entry.isDirectory()) yield* walk(file);
    else if (entry.isFile()) yield file;
  }
}

fs.mkdirSync(dist, { recursive: true });
const imports = new Map();
const resources = [];
for (const name of packageNames) {
  const packageRoot = path.join(root, "node_modules", name);
  const manifest = JSON.parse(fs.readFileSync(path.join(packageRoot, "package.json"), "utf8"));
  const virtualRoot = `/node_modules/${name}`;
  imports.set(`${virtualRoot}/${(manifest.main ?? "dist/src/index.js").replace(/^\.\//, "")}`, name);
  for (const file of walk(path.join(packageRoot, "lib"))) {
    if (!file.endsWith(".tsp")) continue;
    const text = fs.readFileSync(file, "utf8");
    for (const match of text.matchAll(/^\s*import\s+"([^"]+\.js)"/gm)) {
      if (!match[1].startsWith(".")) continue;
      const target = path.resolve(path.dirname(file), match[1]);
      if (!target.startsWith(`${packageRoot}${path.sep}`) || !fs.existsSync(target)) {
        throw new Error(`unbundled TypeSpec JS import: ${target}`);
      }
      imports.set(`${virtualRoot}/${path.relative(packageRoot, target).replaceAll("\\", "/")}`, target);
    }
    resources.push(file);
  }
  resources.push(path.join(packageRoot, "package.json"));
}

// Resource files travel with the wasm; the native host reads them as UTF-8
// and supplies their virtual names in __spec_files. No runtime npm install.
const resourceRoot = path.join(dist, "resources", "node_modules");
fs.rmSync(resourceRoot, { recursive: true, force: true });
for (const file of resources) {
  const relative = path.relative(path.join(root, "node_modules"), file);
  const dest = path.join(resourceRoot, relative);
  fs.mkdirSync(path.dirname(dest), { recursive: true });
  fs.copyFileSync(file, dest);
}
fs.writeFileSync(path.join(dist, "resources.json"), JSON.stringify({
  schema_version: 1,
  root: "resources/node_modules",
  files: resources.map((file) => `/node_modules/${path.relative(path.join(root, "node_modules"), file).replaceAll("\\", "/")}`).sort(),
}, null, 2));

let entry = `import "../src/stubs/globals.js";\n`;
const specs = [...new Set(imports.values())];
for (let i = 0; i < specs.length; i++) {
  entry += `import * as pkg${i} from ${JSON.stringify(path.isAbsolute(specs[i]) ? path.relative(dist, specs[i]).replaceAll("\\", "/") : specs[i])};\n`;
}
entry += `export const jsImports = new Map();\n`;
for (const [virtual, spec] of imports) {
  entry += `jsImports.set(${JSON.stringify(virtual)}, pkg${specs.indexOf(spec)});\n`;
}
entry += `export { compile, tcgc } from "../src/wasi-host.js";\n`;
const entryPath = path.join(dist, "wasi-entry.generated.js");
fs.writeFileSync(entryPath, entry);

const stubs = path.join(root, "src", "stubs");
const stubFor = {
  "@babel/code-frame": "babel-code-frame.js",
  "change-case": "change-case.js",
  "is-unicode-supported": "is-unicode-supported.js",
  "vscode-languageserver": "vscode-languageserver.js",
  "vscode-languageserver-textdocument": "vscode-languageserver.js",
};
await esbuild.build({
  entryPoints: [entryPath],
  bundle: true,
  format: "esm",
  platform: "browser",
  conditions: ["browser", "import", "default"],
  mainFields: ["browser", "module", "main"],
  target: "es2022",
  outfile: path.join(dist, "bundled.js"),
  plugins: [{
    name: "wasi-stubs",
    setup(build) {
      build.onLoad({ filter: /[\\/]@typespec[\\/]compiler[\\/]dist[\\/]src[\\/]core[\\/]formatter\.js$/ },
        () => ({ contents: fs.readFileSync(path.join(stubs, "typespec-formatter.js"), "utf8"), loader: "js" }));
      build.onLoad({ filter: /[\\/]@typespec[\\/]compiler[\\/]dist[\\/]src[\\/]server[\\/]index\.js$/ },
        () => ({ contents: fs.readFileSync(path.join(stubs, "typespec-server.js"), "utf8"), loader: "js" }));
      build.onLoad({ filter: /[\\/]fast-uri[\\/]index\.js$/ }, (args) => ({
        contents: fs.readFileSync(args.path, "utf8").replaceAll("\\P{ASCII}", "[^\\x00-\\x7F]"),
        loader: "js",
      }));
      for (const [name, file] of Object.entries(stubFor)) {
        build.onResolve({ filter: new RegExp(`^${name.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}$`) },
          () => ({ path: path.join(stubs, file) }));
      }
      build.onResolve({ filter: /^prettier($|\/)/ }, () => ({ path: path.join(stubs, "empty.js") }));
      build.onResolve({ filter: /formatter|printTypeSpec/ }, (args) =>
        args.importer.includes("@typespec/compiler") && /formatter|printTypeSpec/.test(args.path)
          ? { path: path.join(stubs, "typespec-formatter.js") } : null);
      build.onResolve({ filter: /server/ }, (args) =>
        args.importer.includes("@typespec/compiler") && /\.\/server|\/server\//.test(args.path)
          ? { path: path.join(stubs, "typespec-server.js") } : null);
    },
  }],
});

const engine = process.env.STARLINGMONKEY_ENGINE;
if (engine && !fs.existsSync(engine)) throw new Error(`missing StarlingMonkey engine: ${engine}`);
const args = [
  "componentize", path.join(dist, "bundled.js"),
  "--wit", path.join(root, "wit", "component.wit"),
  "--world-name", "codegen", "--disable", "http", "fetch-event",
  ...(engine ? ["--engine", engine] : []),
  "--out", path.join(dist, "tcgc.wasm"),
];
execFileSync(process.execPath, [path.join(root, "node_modules", "@bytecodealliance", "jco", "src", "jco.js"), ...args], { stdio: "inherit" });
console.log(`built ${path.join(dist, "tcgc.wasm")} and ${resources.length} resource files`);
