Goal
- Bring `packages/rs/test/g3rs-test-config-checks` to the current internal package shape and eliminate all findings.

Approach
- Normalize the package shell:
  - add root policy files and `guardrail3-rs.toml`
  - mark the facade and member crates non-publishable
  - add missing include/docs/shared metadata
  - split root features into `types`, `runtime`, and `api`
- Clean the crate boundaries:
  - switch runtime off the local `crates/types` facade onto `g3rs-test-types`
  - feature-gate the root and local types facade exports
  - add the allowed dependency to `guardrail3-rs.toml`
- Reduce runtime structural complexity:
  - group the rule directories under `nextest/` and `mutants/`
  - mirror those groups in the assertions crate
- Replace the old test harness shape:
  - delete `crates/runtime/src/test_helpers.rs`
  - split each `rule_tests/mod.rs` into facade-only `mod.rs` plus `behavior.rs` and `helpers.rs`
  - create matching shared assertions modules for each rule

Key decisions
- Fix the runtime complexity by grouping rules into domain submodules instead of waiving `g3rs-arch/structural-split`.
- Keep the cleanup package-local. Do not change any rules unless the grouped shape exposes a real contradiction.

Files to modify
- `packages/rs/test/g3rs-test-config-checks/Cargo.toml`
- `packages/rs/test/g3rs-test-config-checks/README.md`
- `packages/rs/test/g3rs-test-config-checks/src/lib.rs`
- `packages/rs/test/g3rs-test-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/test/g3rs-test-config-checks/crates/runtime/src/lib.rs`
- `packages/rs/test/g3rs-test-config-checks/crates/runtime/src/run.rs`
- `packages/rs/test/g3rs-test-config-checks/crates/types/Cargo.toml`
- `packages/rs/test/g3rs-test-config-checks/crates/types/src/lib.rs`
- `packages/rs/test/g3rs-test-config-checks/crates/assertions/Cargo.toml`
- `packages/rs/test/g3rs-test-config-checks/crates/assertions/src/lib.rs`
- runtime rule and sidecar files under new `nextest/` and `mutants/` groups
- matching assertions files under new `nextest/` and `mutants/` groups
- new root policy files and `guardrail3-rs.toml`
