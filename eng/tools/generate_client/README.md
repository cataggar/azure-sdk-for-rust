# generate_client

`generate_client` is the native Rust host for the TypeSpec Client Generator Core
(TCGC) WebAssembly component. Its Rust emitter is under development: normal
generation reads and validates a JSON code model but **refuses to write generated
source**. `--preview-models` emits the supported schema-2 model-only subset to a
separate scratch crate when used with `--model` and `--output`; it still rejects
all clients. `--preview-basic` uses the same guard and emits models, enums, and
basic clients with operations into `src/generated/`. Both previews use
`output::reconcile`, including `--check`. Existing `tsp-client` regeneration remains the
supported workflow until semantic parity has been verified.

The schema-2 TCGC adapter also records basic HTTP path, query, and header
bindings and explicit success status codes. `--preview-basic` supports GET,
PUT, POST, PATCH, DELETE, and HEAD without request bodies; required path and
optional or required query/header primitive parameters; string constants;
and JSON-typed or bodyless responses. Path values are encoded as individual
URL segments (so embedded `/` cannot become a path separator). The caller
supplies an already configured `azure_core::http::Pipeline`; this preview does
not set up credentials. Child clients, composite path placeholders, unsupported
response statuses or types, and ambiguous parameter constructs fail closed.
It is **not** a replacement for generated SDK clients.

To render an offline fixture into a separate scratch crate, whose `Cargo.toml`
already exists:

```sh
cargo run --manifest-path eng/tools/Cargo.toml -p generate_client -- \
  --manifest-path sdk/keyvault/azure_security_keyvault_secrets/Cargo.toml \
  --model eng/tools/generate_client/tests/fixtures/basic-http.json \
  --output path/to/separate/scratch-crate --preview-basic
```

Run the CLI from the repository root:

```sh
cargo run --manifest-path eng/tools/Cargo.toml -p generate_client -- \
  --manifest-path sdk/keyvault/azure_security_keyvault_secrets/Cargo.toml \
  --spec-dir path/to/typespec/project \
  --component eng/tools/generate_client/tcgc-component/dist/tcgc.wasm \
  --resources eng/tools/generate_client/tcgc-component/dist/resources
```

`--resources` points to a directory with `node_modules/` containing the
version-matched TypeSpec package sources, such as
`tcgc-component/dist/resources`. The host checks the sibling `resources.json`
manifest and reads the sources and project into a virtual filesystem before
invoking the component. For a tracked SDK crate, `--spec-dir` must refer to
the exact repository commit and project directory in its `tsp-location.yaml`.
`--sync` fetches that pinned commit into a local cache instead of taking
`--spec-dir`. `--model` accepts a JSON code model without loading a component.
For previews, `--check` compares owned files without writing;
missing, differing, or unexpected generated files fail. Never target an SDK
crate checkout with a preview.

Build the component with the pinned dependencies under `tcgc-component/`.
Node.js is required to build the component, not to run `generate_client`.
