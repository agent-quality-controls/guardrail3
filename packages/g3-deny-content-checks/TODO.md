# g3-deny-content-checks TODO

## Known Issue

### Structural typed-parse failures can still go silent in the app boundary

- The package correctly assumes typed `DenyToml` input.
- The app deny family still skips package execution when typed parsing fails and
  does not always emit a structural finding for that case.
- This means malformed-schema cases can disappear instead of surfacing as deny
  findings.

Relevant files:

- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/facts/mod.rs`
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/run.rs`

Examples seen during attack:

- `[advisories].ignore = [{ id = "RUSTSEC-...", reason = 7 }]`
- `[bans].skip = [{ crate = "regex@1.0.0", reason = 7 }]`
- wrong-type containers such as `[sources].allow-git = "https://..."`

Follow-up:

- Add structural deny findings for typed parser/schema rejection before calling
  the package.
- Reassign malformed-schema expectations to app-level structural tests rather
  than package-level content tests.

## Test Status

- 121 package-level tests cover all 22 migrated content rules.
- Each migrated rule has package-local sidecar tests.

## Boundary Reminder

- `RS-DENY-23`, `RS-DENY-24`, and `RS-DENY-28` now operate on valid typed
  inputs only.
- Any malformed raw-schema expectations that used to be attached to those rules
  belong in the app once structural signaling is fixed.
