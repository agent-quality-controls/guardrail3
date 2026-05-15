# Goal

Apply the Serde-first fixture output rule to every remaining Rust family type workspace that feeds ingestion output.

# Approach

- Keep the fmt family as the proven pattern.
- For each remaining `packages/rs/**/g3rs-*-types` workspace, inspect the owned public family structs and enums.
- Add `serde = { version = "1", features = ["derive"] }` only to type crates that need `Serialize`.
- Add `serde` to the same workspace's `guardrail3-rs.toml` `allowed_deps`.
- Derive `serde::Serialize` on owned family structs/enums used as rule or ingestion inputs.
- Do not add `serde_json` to type crates.
- Do not write adapters, exporters, fixture-only structs, or conversion layers.
- After each family, run `cargo check`, `cargo test`, and `g3rs validate` on the type workspace and its ingestion workspace.

# Families

- apparch
- arch
- cargo
- clippy
- code
- deny
- deps
- garde
- hooks
- release
- test
- toolchain
- topology

# Verification Per Family

- `cargo check --manifest-path <types>/Cargo.toml`
- `cargo test --manifest-path <types>/Cargo.toml`
- `cargo check --manifest-path <ingestion>/Cargo.toml`
- `cargo test --manifest-path <ingestion>/Cargo.toml`
- `g3rs validate --path <types>`
- `g3rs validate --path <ingestion>`

# Stop Condition

If a derive fails because a field type is not serializable:

- stop the family
- document the exact type path
- document the exact field
- do not add custom conversion code

# Code Family Stop

The `code` family cannot be completed with a Serde derive-only change.

- Type path: `g3rs_code_types::G3RsCodeParsedSourceState`
- Variant: `Parsed(syn::File)`
- Blocking field type: `syn::File`
- Evidence: `cargo info syn@2.0.117` lists no Serde feature; current dependency uses `syn = { version = "2", features = ["extra-traits", "full"] }`
- Decision: do not add a fixture adapter, exporter, manual serializer, `#[serde(skip)]`, or lossy fixture-only type.
- Required follow-up: redesign the code-family source replay boundary so fixture output does not require serializing a third-party AST value.
