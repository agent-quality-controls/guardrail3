# RS-CLIPPY Fixes

Companion status record for [`../clippy.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/clippy.md).

The original RS-CLIPPY attack backlog from the hardening sweep is closed. This file now records the implemented outcomes and the architectural decisions that replaced the old open backlog.

Primary implementation roots:

- [`README.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/README.md)
- [`facts.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs)
- [`facts/cargo.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts/cargo.rs)
- [`facts/configs.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts/configs.rs)
- [`facts/policy.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts/policy.rs)
- [`clippy_support.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/clippy_support.rs)
- [`domain/modules/clippy`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/domain/modules/clippy)

## Closed Fixes

### 1. `RS-CLIPPY-24` wrong-shape `.cargo/config*` now fails closed

- parseable but malformed `env` data now produces explicit `RS-CLIPPY-24` failures
- missing-content and syntax-error paths stay fail-closed
- sidecars cover syntax, shape, nested applicability, and missing-content branches

### 2. Malformed ban entries no longer disappear silently

- shared ban parsing now records malformed section shapes and malformed entries explicitly
- ban-driven rules do not emit clean inventory when a managed section is structurally invalid
- malformed-shape coverage exists across completeness, extra-ban, quality, duplicate, and macro rules

### 3. Assertion exactness is hardened

- the clippy assertion layer no longer relies on set-collapse where exact result counts matter
- parity and golden tests now prove exact count or exact-path expectations where the scenario implies them

### 4. Parity tests use canonical domain exports

- copied inventory tables were replaced with imports from [`domain/modules/clippy`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/domain/modules/clippy)
- generator/runtime parity is now pinned against the canonical policy surface instead of duplicated local lists

### 5. Wrong-type vs missing-value diagnostics are distinct

- thresholds distinguish missing from malformed non-integer values
- `RS-CLIPPY-16` distinguishes missing from non-bool `avoid-breaking-exported-api`
- `g3rs-clippy/avoid-breaking-exported-api` distinguishes missing from wrong-type managed test-relaxation keys

### 6. `RS-CLIPPY-04/05` completeness proofs are exact

- golden tests prove exact canonical count
- emitted managed paths are matched against the canonical set, not spot-checked

### 7. The previously missing fail-closed sidecars were added

- standalone coverage
- same-root precedence
- missing-content branches
- negative published-library classification
- malformed policy-context short-circuit ownership
- plain-string completeness-vs-quality cross-rule behavior

## Closed Decisions

### 8. Malformed routed `Cargo.toml` stays fail-closed inside RS-CLIPPY

- coverage ownership stays with `RS-CLIPPY-01`
- attached-config placement ownership stays with `RS-CLIPPY-12`
- the family facts layer records routed Cargo-root parse failures once and dependent rules consume that fact

### 9. Pure-layer service roots do not get library-only global-state bans

- canonical clippy generation was aligned to runtime expectations
- pure-layer service semantics remain owned by architecture checks, not the clippy baseline

### 10. Malformed allowed `clippy.toml` is single-owned by `RS-CLIPPY-25`

- threshold rules no longer fan out duplicate parse errors for one malformed config
- parseability is owned once at family orchestration time

### 11. `g3rs-clippy/local-policy-root` defers malformed policy-context ownership to `RS-CLIPPY-23`

- local-policy-root baseline checks do not invent a second malformed-policy owner
- sidecars prove that malformed policy context is single-owned by `RS-CLIPPY-23`

### 12. `g3rs-clippy/package-native-policy/07` emit positive clean inventory

- the extra-ban inventory rules now emit explicit clean-path info results when the managed section is parseable and contains no extras
- broken inputs still short-circuit to the owning malformed-input rule instead of inventing clean inventory

### 13. `RS-CLIPPY-19` keeps the typo heuristic with boundary proofs

- the heuristic was kept
- boundary sidecars were added so the managed-key typo surface is pinned against realistic false-positive boundaries

## Closed Test Gaps

The missing sidecar gaps identified by the attack pass are closed for:

- malformed policy-context short-circuit ownership
- malformed managed ban-section shapes
- published-library positive and negative boundaries
- wrong-type scalar branches
- string-form completeness-vs-quality interactions
- clean-path user-added bans with real reasons

## Closed Doc Drift

- [`clippy.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/clippy.md) now describes the live managed baseline, malformed-input ownership model, clean inventory contract, and pure-layer policy decision
- [`README.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/README.md) now matches the live family shape and ownership split

## Verification Snapshot

At the close of this sweep:

- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-clippy --lib` passes
- `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family clippy --format json` reports `0 errors`, `0 warnings`
- `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family code --format json` reports no error/warn findings in clippy-owned files
