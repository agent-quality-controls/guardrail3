Summary
- Added exact waiver support for `g3rs-clippy/max-struct-bools` so schema-mirror parser packages can keep the generated clippy baseline and suppress only the exact `max-struct-bools` key check.
- Finished the clean-shape rewrite for `packages/parsers/clippy-toml-parser`, verified it validates clean, and removed the last real `pub type` compatibility alias found in package code.

Decisions made
- Threaded clippy waivers through clippy ingestion and config-check inputs instead of reintroducing local lint escape hatches on schema structs.
- Made the waiver exact-match only: rule id + file + selector `key:max-struct-bools`.
- Preserved invalid `guardrail3-rs.toml` behavior in clippy ingestion by only parsing waivers when the policy file already parses cleanly.
- Kept parser public shape strict: root exports `parse`, `from_path`, and `Error`; schema types stay under `types`.
- Removed the remaining actual type alias in `g3rs-release-repo-root-checks` by switching callers to the real shared type name `G3RsReleaseConfigRepo`.

Key files for context
- packages/rs/clippy/g3rs-clippy-types/src/types.rs
- packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/run.rs
- packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/parse.rs
- packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/support.rs
- packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_01_max_struct_bools/rule.rs
- packages/parsers/clippy-toml-parser/guardrail3-rs.toml
- packages/parsers/clippy-toml-parser/src/lib.rs
- packages/parsers/clippy-toml-parser/src/types.rs
- packages/rs/release/g3rs-release-repo-root-checks/crates/types/src/lib.rs

Next steps
- Continue package-by-package under `packages/parsers` from the next unchecked parser package.
- After parsers, continue through `packages/shared` with the same clean-shape sweep.
