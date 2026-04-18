Goal

Normalize `packages/parsers/release-plz-toml-parser` to the current parser package shape so validation removes the structural package-debt errors and leaves only intentional warning inventory if any remains.

Approach

- Move the package from `crates/parser/{runtime,assertions,types}` to `crates/{runtime,assertions,types}`.
- Make the root facade minimal: parser behavior at the root, types under `release_plz_toml_parser::types`, with no backward-compat root type dump.
- Add the missing package policy files and `guardrail3-rs.toml`.
- Fix the parser sidecar ownership to the owned `parser_tests` shape and move final proof into the shared assertions crate.
- Make internal crates explicitly unpublished and add the release metadata expected for parser packages.
- Update any touched repo callers off removed root type exports instead of preserving aliases.

Key decisions

- Keep the centralized parser/fs allows visible as `RS-CODE-SOURCE-04` warnings if that is the only signal left. Do not add package-local band-aids to hide those inventory warnings.
- Keep the types crate passive. If a type-surface rule remains after the split, prefer exact waivers or schema-faithful restructuring over compatibility shims.
- Use the same clean parser shape already established in `deny-toml-parser`, `guardrail3-rs-toml-parser`, `hook-shell-parser`, and `nextest-toml-parser`.

Files to modify

- `packages/parsers/release-plz-toml-parser/Cargo.toml`
- `packages/parsers/release-plz-toml-parser/src/lib.rs`
- `packages/parsers/release-plz-toml-parser/src/types.rs`
- `packages/parsers/release-plz-toml-parser/clippy.toml`
- `packages/parsers/release-plz-toml-parser/deny.toml`
- `packages/parsers/release-plz-toml-parser/rustfmt.toml`
- `packages/parsers/release-plz-toml-parser/guardrail3-rs.toml`
- `packages/parsers/release-plz-toml-parser/crates/**`
