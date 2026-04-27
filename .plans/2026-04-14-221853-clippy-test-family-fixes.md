# Goal
Make `packages/rs/clippy/g3rs-clippy-config-checks` clean under the `test` family by moving all proof logic into the shared assertions crate and keeping sidecars as setup-and-run harnesses only.

# Approach
1. Fix weak `test` rule messages before package changes.
   - Rewrite `g3rs-test/real-proof-site` so it says the exact missing proof problem instead of `real proof site`.
   - Audit the live `g3rs-test/owned-sidecar-shape`, `g3rs-test/runtime-assertions-split`, and `g3rs-test/assertions-modules-prove` messages against the message format plan and tighten the vague ones.
2. Fix the package wiring first.
   - Add the missing sibling dependencies between `crates/runtime` and `crates/assertions`.
   - Stop `crates/assertions` from reaching local private code.
3. Move rule-specific proof code into the shared assertions crate.
   - Create rule-specific assertion modules under `crates/assertions/src/...`.
   - Remove local `rule_tests/assertions.rs` proof wrappers from sidecars.
4. Shrink sidecars to setup and execution only.
   - Keep `rule_tests/mod.rs` as a dispatcher.
   - Keep `helpers.rs` only for building inputs or running the rule.
   - Move direct `CheckResult` shape assertions out of `cases.rs` and into the shared assertions crate.
5. Re-run `guardrail3-rs validate --family test` on the package and close the remaining gaps.

# Key Decisions
- Keep `g3rs-test/real-proof-site` broad. It should catch both tests with no proof step and tests that route proof through local sidecar code.
- Do not guess exact destination files in rule messages when the rule does not know them.
- Keep the shared assertions crate as the single proof layer for both internal sidecars and external harnesses.

# Files To Modify
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/tests/mod.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_02_owned_sidecar_shape/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/assertions/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/assertions/src/...`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/...`
