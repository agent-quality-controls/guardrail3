# Summary

Hardened the extracted `hexarch` source lane until the second adversarial pass found no blocker. The main fixes were in source ingestion: app-workspace member discovery now matches the old app family more closely, per-crate failures stay local, and the package tests now cover the old reachable-module and visibility attack surface.

# Decisions made

- Moved `hexarch` source discovery from "every Cargo.toml under apps/" to workspace-member discovery.
  - Why: the old app family only analyzed real workspace members, and the broader selection was inventing fake source-crate opportunities.
  - Rejected: keeping the looser scan and filtering later in rules.

- Kept member-manifest failures inside per-crate source-rule fail-closed output.
  - Why: one bad member should not abort unrelated good crates in the same lane.
  - Rejected: lane-wide ingestion errors for invalid or unreadable selected member manifests.

- Matched old entrypoint behavior.
  - Why: crates with no `src/` should be skipped, but crates with `src/` and no `lib.rs` or `main.rs` should fail closed.
  - Rejected: treating all missing entrypoints the same.

- Added attack-style tests where the behavior now lives.
  - Why: traversal, member discovery, module resolution, visibility, and fail-closed behavior are ingestion concerns now.
  - Rejected: leaving those cases as legacy app-only coverage.

# Key files for context

- `.plans/2026-04-10-211242-harden-hexarch-source-lane.md`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/view.rs`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/ingest_tests/selection.rs`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/ingest_tests/reachability.rs`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/hexarch/g3rs-hexarch-source-checks/crates/runtime/src/rs_hexarch_22_ports_trait_dominance_tests/mod.rs`
- `packages/rs/hexarch/g3rs-hexarch-source-checks/crates/runtime/src/rs_hexarch_23_adapter_pub_trait_tests/mod.rs`

# Next steps

- Build `g3rs-hexarch-config-checks`.
- Implement `g3rs-hexarch-ingestion::ingest_for_config_checks(...)`.
- If `hexarch` grows further, add stronger shared assertion helpers so the family stops using plain `assert_eq!` and substring checks.
