Summary
- Reshaped the remaining app components into package-style sibling crates where needed, cleaned the root app workspace debt, and removed stale root clippy allow entries.
- `apps/guardrail3-rs` now passes workspace tests and validates with `No findings.`.

Decisions made
- Kept the package-style split for `cli`, `report`, and `validate-command`: `crates/runtime` plus `crates/assertions`.
- Removed stale root lint allows only after proving the workspace passes with `-D clippy::module_name_repetitions` and `-D clippy::multiple_crate_versions`.
- Fixed remaining app debt directly in place: docs, explicit enum match arms, assertion messages, and the CLI main exit path.

Key files for context
- apps/guardrail3-rs/Cargo.toml
- apps/guardrail3-rs/guardrail3-rs.toml
- apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/main.rs
- apps/guardrail3-rs/crates/io/inbound/cli/crates/assertions/src/cli.rs
- apps/guardrail3-rs/crates/io/outbound/report/crates/runtime/src/plain_text.rs
- apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/execute.rs
- apps/guardrail3-rs/crates/logic/family-runner-quality/src/run.rs
- apps/guardrail3-rs/crates/types/app-types/src/lib.rs

Next steps
- If the app changes again structurally, rerun `cargo test -q --manifest-path apps/guardrail3-rs/Cargo.toml --workspace` and `guardrail3-rs validate --path apps/guardrail3-rs` first.
- The root clippy allow entries are gone; if they reappear, prove the concrete lint hit first instead of restoring the manifest allow blindly.
