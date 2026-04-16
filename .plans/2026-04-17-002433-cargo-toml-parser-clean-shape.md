Goal
- Remove the root schema-type aliases from `packages/parsers/cargo-toml-parser` so the clean public shape is `cargo_toml_parser::types::...`, while keeping parse entrypoints at the root.
- Update all current repo callers to the clean shape and verify the affected workspaces still compile and validate.

Approach
- Change `packages/parsers/cargo-toml-parser/src/lib.rs` to expose only `pub mod types;` plus `Error`, `parse`, and `from_path` from runtime.
- Update every current root-type import and root-type path usage across `packages/rs/**` and `apps/**` to use `cargo_toml_parser::types::...` instead.
- Re-run `cargo test` for `cargo-toml-parser`, then run `cargo check` or `cargo test` on the directly affected workspaces, and re-run validation on the parser package.

Key decisions
- No backward-compat aliases at the root. That old shape is architectural debt.
- Keep parser functions at the root. Only schema types move under `types`.

Files to modify
- `packages/parsers/cargo-toml-parser/src/lib.rs`
- all Rust callers currently importing root schema names from `cargo_toml_parser`
