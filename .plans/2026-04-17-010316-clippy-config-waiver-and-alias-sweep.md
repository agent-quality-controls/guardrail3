Goal
- Make `packages/parsers/clippy-toml-parser` clean without reintroducing local lint escape hatches.
- Add narrow waiver support for the exact clippy config rule/key needed by schema-mirror parser packages.
- Remove any remaining backward-compat root type aliases or wildcard facade aliases in packages/app.

Approach
- Read the clippy config ingestion/check pipeline to find the architecturally correct place to thread package waivers into config rule inputs.
- Add a failing test first that proves a package waiver for `g3rs-clippy/max-struct-bools` on `max-struct-bools` is currently ignored.
- Thread waivers through clippy ingestion/types/checks and implement exact-match suppression in the `max-struct-bools` rule only.
- Restore `packages/parsers/clippy-toml-parser/clippy.toml` to the repo baseline and add a package waiver entry instead of raising the baseline.
- Sweep packages/app for root alias shims like `pub type X = types::X`, `pub use ...::types::*`, or root parser schema reexports kept only for compatibility, and remove any found if the clean facade can expose `types` instead.
- Run targeted tests, then package validate for `clippy-toml-parser`, then broader grep checks for aliases.

Key decisions
- Prefer waiver support in clippy config inputs over package-local lint exceptions on schema structs.
- Waiver must be exact: rule id + config file + selector for the exact key.
- Keep parser root API minimal: parse/from_path/Error at root, schema under `types`.

Files to modify
- packages/rs/clippy/g3rs-clippy-types/src/types.rs
- packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/run.rs
- packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/parse.rs
- packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/support.rs
- packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_01_max_struct_bools/rule.rs
- related clippy ingestion/config tests
- packages/parsers/clippy-toml-parser/clippy.toml
- packages/parsers/clippy-toml-parser/guardrail3-rs.toml
- any package/app files found in the alias sweep
