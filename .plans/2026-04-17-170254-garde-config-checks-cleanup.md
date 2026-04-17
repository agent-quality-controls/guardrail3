Goal

Clean `packages/rs/garde/g3rs-garde-config-checks` against the current rule set without changing rules.

Approach

- Add the missing root policy files at the package root so the workspace is covered by the required config families.
- Remove the fake local `crates/types` re-export crate and point runtime/root dependencies at `packages/rs/garde/g3rs-garde-types` directly.
- Normalize the runtime sidecar declarations:
  - `run.rs` uses `mod run_tests;`
  - each `rule.rs` uses `mod rule_tests;`
  - every `#[path]` keeps the same-line reason comment required by the current code rule.
- Move stale `run_tests/mod.rs` semantic result assertions into a shared assertions crate file and leave `mod.rs` facade-only.
- Normalize per-rule sidecars so helpers call only the owned production module and final result assertions live in shared assertions modules.
- Re-run package tests and `guardrail3-rs validate --path packages/rs/garde/g3rs-garde-config-checks`.

Key decisions

- Treat the local `crates/types` member as package debt, not architecture. Other cleaned families already use the shared family types package directly.
- Keep the current narrow `#[path = \"..._tests/mod.rs\"]` sidecar shape rather than trying to restructure these rules into directory modules.
- Stop and report if a remaining finding turns out to be a real rule contradiction after the stale package shape is removed.

Files to modify

- `packages/rs/garde/g3rs-garde-config-checks/Cargo.toml`
- `packages/rs/garde/g3rs-garde-config-checks/src/lib.rs`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/run.rs`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/run_tests/**`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/rs_garde_config_0*/rule.rs`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/rs_garde_config_0*/rule_tests/**`
- `packages/rs/garde/g3rs-garde-config-checks/crates/assertions/src/**`
- package root policy files under `packages/rs/garde/g3rs-garde-config-checks/`
- remove `packages/rs/garde/g3rs-garde-config-checks/crates/types/**` if no longer needed
