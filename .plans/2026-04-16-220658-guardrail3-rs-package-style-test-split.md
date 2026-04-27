Goal
- Fix the app packages so test rules pass without changing rules.
- Reshape `cli`, `report`, and `validate-command` from single-crate packages with nested `assertions/` into package-style layouts with sibling member crates under `crates/`.
- Move remaining app sidecar proof into shared assertions crates so g3rs-test/real-proof-site stops firing.

Approach
- Read current manifests and source/tests for `cli`, `report`, and `validate-command`.
- Convert each component into `component/Cargo.toml` plus `component/crates/runtime` and `component/crates/assertions`.
- Update workspace members and dependency paths in `apps/guardrail3-rs/Cargo.toml` and any app callers.
- Keep source layout and sidecar tests under each runtime crate, but point tests at the sibling assertions crate.
- Run app workspace tests and app validation; continue with package-only fixes until the next real rule contradiction, then stop.

Key decisions
- Fix package layout, not rules.
- Keep changes local to the failing app components first.
- Treat new validator output after the package split as the next source of truth.

Files to modify
- apps/guardrail3-rs/Cargo.toml
- apps/guardrail3-rs/crates/io/inbound/cli/**
- apps/guardrail3-rs/crates/io/outbound/report/**
- apps/guardrail3-rs/crates/logic/validate-command/**
