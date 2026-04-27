# Goal
Make `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src` satisfy `g3rs-arch/mod-facade-only` by turning every offending `mod.rs` into a pure dispatcher without weakening the rule.

# Approach
- Prove the current package still fails `g3rs-arch/mod-facade-only` with the CLI validator before editing.
- Read the runtime tree and identify every `mod.rs` that contains inline test bodies or inline helper modules.
- For rule directories `rs_clippy_config_01..08`, keep the sidecar directory pattern but move inline `assertions` modules out of `rule_tests/mod.rs` into sibling `assertions.rs` files, leaving `mod.rs` as declarations only.
- For test-only directories `rs_clippy_config_09..21`, move inline `#[test]` bodies out of `mod.rs` into sibling files, leaving `mod.rs` as declarations only.
- Re-run the package validator and crate tests to prove `g3rs-arch/mod-facade-only` is gone while keeping behavior intact.

# Key Decisions
- Keep `g3rs-arch/mod-facade-only`. The package changes, not the rule.
- Keep the sidecar directory layout. Remove inline bodies, not the pattern.
- Do not touch the broader `test` family slice. Only remove the `arch` facade violations.

# Files To Modify
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/**/mod.rs`
- New sibling files under `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/**/`
- Possibly `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/lib.rs` if module wiring needs adjustment
