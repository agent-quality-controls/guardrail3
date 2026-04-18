Goal

Normalize `packages/parsers/rust-toolchain-toml-parser` to the current parser package shape so validation removes the structural package-debt errors and leaves only intentional warning inventory if any remains.

Approach

- Move the package from `crates/parser/{runtime,assertions,types}` to `crates/{runtime,assertions,types}`.
- Make the root facade minimal: parser behavior at the root, types under `rust_toolchain_toml_parser::types`, with no backward-compat root type dump.
- Add the missing package policy files and `guardrail3-rs.toml`.
- Fix the parser sidecar ownership to the owned `parser_tests` shape and move final proof into the shared assertions crate.
- Make internal crates explicitly unpublished and add the release metadata expected for parser packages.
- Update any touched repo callers off removed root type exports instead of preserving aliases.

Key decisions

- Keep the centralized parser/fs allows visible as `RS-CODE-SOURCE-04` warnings if that is the only signal left.
- Keep the types crate passive. If a type-surface rule remains after the split, prefer exact waivers or schema-faithful restructuring over compatibility shims.
- Use the same clean parser shape already established in the other cleaned parser packages.

Files to modify

- `packages/parsers/rust-toolchain-toml-parser/Cargo.toml`
- `packages/parsers/rust-toolchain-toml-parser/src/lib.rs`
- `packages/parsers/rust-toolchain-toml-parser/src/types.rs`
- `packages/parsers/rust-toolchain-toml-parser/clippy.toml`
- `packages/parsers/rust-toolchain-toml-parser/deny.toml`
- `packages/parsers/rust-toolchain-toml-parser/rustfmt.toml`
- `packages/parsers/rust-toolchain-toml-parser/guardrail3-rs.toml`
- `packages/parsers/rust-toolchain-toml-parser/crates/**`
