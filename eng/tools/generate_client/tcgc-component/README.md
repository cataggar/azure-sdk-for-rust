# TypeSpec code-model component

Run `npm ci` and `npm run build` **from this directory**. Node.js is needed only
to build the component, not to run the native Rust generator. Dependencies
are exact-versioned in `package.json` and `package-lock.json`.

The build produces `dist/tcgc.wasm`, `dist/resources.json`, and
`dist/resources/node_modules/**`. Distribute the Wasm and resource tree
together. The Rust host reads every path listed in `resources.json` from the
resource tree and passes its UTF-8 contents as `__spec_files` alongside the
TypeSpec project's files. Do not load files from a developer's npm installation
at runtime. The resources manifest has `schema_version: 1` and virtual paths
starting with `/node_modules/`; project files must use `/spec/`. Pass `/spec`
(or a directory below it) as `project-path`. `emitter-options` must be a JSON
object with only the `__spec_files` property; that property maps **absolute**
virtual paths to UTF-8 strings. The host must validate real paths and limit
input sizes before constructing the map; this component validates virtual
paths and rejects relative and traversing paths.

The export is `azure:codegen/tcgc.compile` in the `codegen` world, returning
`result<string,string>`. Success contains a `schema_version: 1` Rust-oriented
JSON object with `clients`, `models`, and `enums`. The deliberately small
Phase 1 contract supports simple HTTP operations without user request
bindings, constant string headers, one numeric success status, and a subset
of model types. Unsupported constructs return an error rather than guessed
code. Phase 2 must expand the contract before generating production clients.

The component is built with the default StarlingMonkey engine. Set
`STARLINGMONKEY_ENGINE` to an existing engine Wasm with a larger GC heap for
large ARM specs; the default heap is not sufficient for such specs. The
generated component imports WASI Preview 2 interfaces at version `0.2.10`;
HTTP and fetch-event are disabled together so there is no `wasi:http`
import. The native host must provide the remaining matching imports.

Run `npm test` **after** `npm run build`. The test suite exercises the adapter,
virtual host, a TypeSpec compile in Node, and an actual invocation of the
Wasm export through `jco transpile`. The latter is a build smoke test, not a
substitute for the native Wasmtime integration test.
