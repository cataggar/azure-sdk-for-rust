# generate_client

`generate_client` is the native Rust host for the TypeSpec Client Generator Core
(TCGC) WebAssembly component. Its Rust emitter is under development: normal
generation reads and validates a JSON code model but **refuses to write generated
source**. `--preview-models` emits the supported schema-2 model-only subset to a
separate scratch crate when used with `--model` and `--output`; it still rejects
all clients. `--preview-basic` accepts either a JSON model or a TypeSpec project
with `--output` and emits models, enums, and basic clients with operations into
`src/generated/`. Both previews use
`output::reconcile`, including `--check`. Existing `tsp-client` regeneration remains the
supported workflow until semantic parity has been verified.

The schema-2 TCGC adapter also records basic HTTP path, query, and header
bindings and explicit success status codes. `--preview-basic` supports GET,
PUT, POST, PATCH, DELETE, and HEAD; required path and
optional final string path segments; optional or required query/header primitive
parameters; typed string-enum query values; string constants;
verified JSON model request bodies; and JSON-typed or bodyless responses.
Direct model bytes fields with an explicit `base64url` JSON encoding use
Azure Core's URL-safe base64 serde helpers; other encodings fail closed.
Direct `utcDateTime` model fields with an explicit integer Unix-timestamp
format use Azure Core's Unix-time serde helpers.
Verified GET pagers return `azure_core::http::Pager` with the original response
wrapper and default-empty item arrays. Continuation links are resolved against
the first request URL and the client API version is re-injected, matching the
existing Key Vault client. As with that client, a service-provided continuation
URL can point to a different origin; configure the authentication pipeline
accordingly.
Path values are encoded as individual
URL segments (so embedded `/` cannot become a path separator). The caller
supplies an already configured `azure_core::http::Pipeline`; this preview does
not set up credentials or service-specific authentication challenges. OAuth2
scopes recorded in the model appear in client documentation but are not applied
to the pipeline by this preview. Client-owned API-version bindings use the
TypeSpec default and can be changed with `with_api_version`; same-named
method-owned parameters remain operation arguments. Nested clients use cloned
endpoints and pipelines.
Client names without a `Client` suffix receive one in the preview, so the
pinned TypeSpec `Secret` client renders as `SecretClient`; constructor and
options parity still requires a separate review.
Composite path placeholders, unsupported response statuses or types, and
ambiguous parameter constructs fail closed.
The pinned Key Vault catch-all JSON exception is retained in the code model;
the preview uses the existing SDK's `azure_core` success check and propagates
its HTTP error with the raw response rather than claiming typed error decoding.
It is **not** a replacement for generated SDK clients.
As with the current SDK emitter, optional nullable model fields represent both
a missing property and an explicit JSON `null` as `None`; they cannot preserve
that distinction when serialized again.

To render an offline fixture into a separate scratch crate, whose `Cargo.toml`
already exists:

```sh
cargo run --manifest-path eng/tools/Cargo.toml -p generate_client -- \
  --manifest-path sdk/keyvault/azure_security_keyvault_secrets/Cargo.toml \
  --model eng/tools/generate_client/tests/fixtures/basic-http.json \
  --output path/to/separate/scratch-crate --preview-basic
```

To preview a supported local TypeSpec project, substitute `--spec-dir` for
`--model` and supply `--component` and `--resources` as shown below. The
TypeSpec adapter rejects constructs not represented by the current schema.

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
`tests/fixtures/keyvault-secrets-8d521358.json` is a pinned Key Vault Secrets
regression model; its upstream commit, TCGC version, and SHA-256 are recorded
in the adjacent metadata file. It covers 12 operations, including three GET
pagers, but compiling this subset is **not** API or runtime parity with the
current SDK.
For a pinned TypeSpec input, `--export-model --output <scratch-crate>` writes
the validated model to a new `tcgc-model.json` in that separate scratch crate;
it refuses to overwrite an existing export. This makes repeatable offline
emitter checks possible without rerunning the Wasm compile.
For previews, `--check` compares owned files without writing;
missing, differing, or unexpected generated files fail. Never target an SDK
crate checkout with a preview.

Build the component with the pinned dependencies under `tcgc-component/`.
Node.js is required to build the component, not to run `generate_client`.
