# g3-deny-content-checks TODO

## Known Issues

### Structural typed-parse failures go silent in the app boundary

- The package correctly assumes typed `DenyToml` input.
- The app deny family currently skips package execution when typed parsing fails and does not emit a structural finding for that case.
- This means malformed-schema cases can disappear instead of surfacing as deny findings.

Relevant files:

- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/facts/mod.rs`
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/run.rs`

Examples seen during attack:

- `[advisories].ignore = [{ id = "RUSTSEC-...", reason = 7 }]`
- `[bans].skip = [{ crate = "regex@1.0.0", reason = 7 }]`
- wrong-type containers such as `[sources].allow-git = "https://..."`

Follow-up:

- Add structural deny findings for typed parser/schema rejection before calling the package.
- Reassign malformed-schema expectations to app-level structural tests rather than package-level content tests.

### No direct package tests currently run

- `cargo test --manifest-path packages/g3-deny-content-checks/Cargo.toml --workspace -- --list`
  currently reports `0 tests`.
- The old app-side tests for migrated rules still exist on disk, but they are no longer compiled through the app module graph.

Follow-up:

- Add direct package tests for the migrated rules.
- Recreate parity coverage for the migrated content-rule set inside this package.

### Re-check parity for migrated malformed-shape behavior after structural signaling lands

- `RS-DENY-23`, `RS-DENY-24`, and `RS-DENY-28` were rewritten against typed parser inputs.
- That is the correct package boundary, but the old malformed-shape expectations need to be explicitly redistributed between:
  - app structural tests
  - package content tests over valid typed inputs

Follow-up:

- Audit old deny tests rule by rule.
- Keep only valid typed-input semantics here.
- Move malformed parse/schema expectations to the app family.
