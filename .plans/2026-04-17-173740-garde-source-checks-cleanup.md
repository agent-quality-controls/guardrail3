Goal

Clean `packages/rs/garde/g3rs-garde-source-checks` to the current package shape so package tests pass and `guardrail3-rs validate --path packages/rs/garde/g3rs-garde-source-checks` returns no findings without changing rules.

Approach

- Normalize the package root and member manifests:
  - make publish intent explicit
  - add root policy files
  - move runtime off the local `crates/types` re-export crate and onto `g3rs-garde-types`
  - keep `crates/types` as a thin API re-export boundary
- Normalize the assertions crate:
  - move flat per-rule assertion files into owned `.../mod.rs` plus `.../rule.rs` modules
  - add a shared `run` assertions module for runtime-side proof
  - feature-gate `lib.rs` exports
- Normalize the runtime test layout:
  - move `lib.rs` test ownership off `test_support`
  - convert `run.rs` and all rule files to owned `*_tests` sidecars with same-line reason comments
  - move the old `rs_garde_10_input_failures/tests/` tree to `rule_tests/`
  - replace sidecar imports of `crate::test_support` with local sidecar helpers
  - move semantic result assertions into the shared assertions crate
- Split `parse/mod.rs` into a facade-only `mod.rs` plus a sibling implementation file.
- Centralize direct filesystem calls into a local `fs.rs` boundary and delete the old runtime `test_support.rs`.
- Re-run package tests and validate. Only stop if the narrowed remainder is a real rule contradiction.

Key decisions

- Use the already-cleaned garde packages as the closest structural reference:
  - `g3rs-garde-config-checks`
  - `g3rs-garde-ingestion`
- Do not preserve the old flat assertions layout or root test helper module. Those are exactly what the current rules reject.

Files to modify

- `packages/rs/garde/g3rs-garde-source-checks/Cargo.toml`
- root policy files under `packages/rs/garde/g3rs-garde-source-checks/`
- `packages/rs/garde/g3rs-garde-source-checks/src/lib.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/assertions/**`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/**`
- `packages/rs/garde/g3rs-garde-source-checks/crates/types/**`
