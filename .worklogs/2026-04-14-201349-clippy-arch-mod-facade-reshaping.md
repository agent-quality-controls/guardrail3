# Summary
Removed the remaining `g3rs-arch/mod-facade-only` violations from `packages/rs/clippy/g3rs-clippy-config-checks` by making every runtime `mod.rs` a pure dispatcher. Fixed the underlying arch family bug too: restricted `use` wiring is now treated as valid facade structure instead of body logic.

# Decisions Made
- Kept `g3rs-arch/mod-facade-only` strict and reshaped the clippy runtime package to obey it. Rejected any package-local waiver or rule exemption because the user had already decided every `mod.rs` should be a pure dispatcher.
- Fixed the arch ingestion bug instead of forcing `pub use` in production `mod.rs`. Rejected the first attempt because it required widening visibility and immediately tripped `unreachable-pub`; `pub(crate) use` is valid facade wiring under the rule text.
- Moved inline sidecar assertion blocks and inline test bodies into sibling files. Rejected leaving bodies in `mod.rs` because that would preserve the exact arch violation.

# Key Files For Context
- `.plans/2026-04-14-201349-clippy-runtime-mod-facade-reshaping.md`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_01_max_struct_bools/mod.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_01_max_struct_bools/rule_tests/mod.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_09_missing_method_ban_tests/mod.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_09_missing_method_ban_tests/cases.rs`

# Next Steps
- The `arch` slice on `g3rs-clippy-config-checks` is clean.
- Remaining signals on this package are the already-deferred `test` family violations and the two `g3rs-code/exception-comment-inventory` inventory warnings on `deny.toml`.
- If continuing package-by-package, move to the next family/package and repeat the same validate -> decide -> fix loop.
