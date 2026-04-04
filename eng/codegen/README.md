# TypeSpec Rust Code Generator

A Rust-native code generator that produces Azure SDK crates from TypeSpec
specifications. It replaces the TypeScript-based emitter
([Azure/typespec-rust](https://github.com/Azure/typespec-rust)) with a Rust
implementation that consumes TypeSpec libraries via WASI components.

## Architecture

```text
.tsp files
    │
    ▼
┌──────────────────────┐
│  TCGC WASI Component │  ← TypeSpec + TCGC compiled to WASI via jco
│  (tcgc-component/)   │
└──────────┬───────────┘
           │ JSON code model
           ▼
┌──────────────────────┐
│  Rust Code Generator │  ← Native Rust binary
│  (codegen/)          │
└──────────┬───────────┘
           │
           ▼
   src/generated/*.rs
```

## Crates

| Crate | Description |
|-------|-------------|
| `codegen-types` | Code model types shared between the WASI component and generator |
| `codegen` | Rust code generator binary |

## WASI Component

The `tcgc-component/` directory contains a JavaScript wrapper around TypeSpec
TCGC that is compiled into a WASI component using
[jco](https://github.com/bytecodealliance/jco). The Rust host loads this
component to compile `.tsp` files and extract a typed code model.

## Building

```bash
# Build the Rust crates
cd eng/codegen && cargo build

# Build the WASI component (requires Node.js + jco)
cd eng/codegen/tcgc-component && npm install && npm run build
```

## Status

🚧 Early development — see [issue #90](https://github.com/cataggar/azure-sdk-for-rust/issues/90).
