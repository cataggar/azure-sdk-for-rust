# generate_client

`generate_client` is the native Rust host for the TypeSpec Client Generator Core
(TCGC) WebAssembly component. Its Rust emitter is under development: normal
generation reads and validates a JSON code model but **refuses to write generated
source**. A `--preview-models` flag emits the supported model-only subset to a
separate scratch crate when used with `--model` and `--output`; it rejects
clients and unsupported types. Existing `tsp-client` regeneration remains the
supported workflow until semantic parity has been verified.

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
For model-only previews, `--check` compares owned files without writing;
missing, differing, or unexpected generated files fail. Never target an SDK
crate checkout with a preview.

Build the component with the pinned dependencies under `tcgc-component/`.
Node.js is required to build the component, not to run `generate_client`.
