# Fix RS-DENY Placement And Baseline

**Date:** 2026-03-29 21:01
**Scope:** `apps/guardrail3/deny.toml`, `apps/guardrail3/crates/app/rs/families/deny/README.md`, `apps/guardrail3/crates/app/rs/families/deny/deny.toml` removal, and deny-fixture renames under `apps/guardrail3/tests/fixtures/adversarial-configs`

## Summary
Fixed the remaining live `RS-DENY` repo-root errors by removing forbidden nested live deny configs and adding the missing canonical `lazy_static` ban to the real app-root deny policy. The deny family README was updated to stop claiming a self-hosted family-root `deny.toml` that is now forbidden by the active workspace-level deny model.

## Context & Problem
After the inventory-contract sweep, `RS-DENY` still had 9 live repo-root errors:
- `RS-DENY-02` / `RS-DENY-03` on a family-local `crates/app/rs/families/deny/deny.toml`
- the same placement/shadowing pair on three adversarial fixture directories under `tests/fixtures/adversarial-configs/*/deny.toml`
- `RS-DENY-09` because the app-root `apps/guardrail3/deny.toml` was missing `lazy_static`

The important constraint was to fix the actual deny-policy shape rather than special-casing the checker. The family plan already says deny configs are only allowed at validation roots, workspace roots, and standalone non-workspace package roots, and nested deny configs must not shadow parent policy.

## Decisions Made

### Remove live nested deny configs instead of weakening placement/shadowing rules
- **Chose:** Delete the family-local `apps/guardrail3/crates/app/rs/families/deny/deny.toml` and convert the three adversarial fixture deny files to inert `deny.fixture.toml` names.
- **Why:** The live errors were correct. Those files were genuine nested deny configs under the app root, so the right fix was to stop making them live policy roots.
- **Alternatives considered:**
  - Relax `RS-DENY-02` / `03` for family roots or fixture directories — rejected because that would weaken the deny model and re-open shadowing.
  - Keep the nested files and exclude their directories from routing — rejected because it would hide real policy files instead of fixing the shape.

### Patch the real app-root deny baseline instead of leaving a known managed gap
- **Chose:** Add `lazy_static` to `apps/guardrail3/deny.toml`.
- **Why:** `RS-DENY-09` explicitly said the root policy was missing a canonical ban that already exists in `crates/domain/modules/deny.rs`.
- **Alternatives considered:**
  - Ignore the mismatch because `RS-DENY-26` only inventories missing reasons anyway — rejected because the actual failing rule was ban completeness, not reason hygiene.
  - Remove `lazy_static` from the generator baseline — rejected because that would be policy drift, not a fix.

### Update the README to match the real deny ownership model
- **Chose:** Remove the README claim that the deny family still self-hosts a live family-root `deny.toml`.
- **Why:** After the workspace-boundary and placement hardening, that claim had become false and was directly at odds with the checker.
- **Alternatives considered:**
  - Leave the README stale until a broader deny pass — rejected because the file was already describing a configuration that now produces live errors.

## Architectural Notes
`RS-DENY` now matches the one-live-root deny model for this repo:
- the app-root `apps/guardrail3/deny.toml` is the actual live deny policy
- family-local and adversarial fixture deny content can still exist, but only as inert fixture artifacts, not live `deny.toml` filenames

This keeps placement/shadowing semantics honest without touching the rule logic.

## Information Sources
- Live repo-root `RS-DENY` error output from:
  - `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family deny --inventory --format json`
- Canonical deny baseline in:
  - `apps/guardrail3/crates/domain/modules/deny.rs`
- Current family documentation:
  - `apps/guardrail3/crates/app/rs/families/deny/README.md`
- Family test support and assertions indicating deny fixtures are generated/consumed in temp roots rather than from the app-root adversarial fixture files:
  - `apps/guardrail3/crates/app/rs/families/deny/test_support/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/deny/crates/assertions/src/facts.rs`

## Open Questions / Future Considerations
- The deny README still records the separate malformed-`guardrail3.toml` profile fail-open issue and the broader hardening backlog; this commit does not address those.
- The adversarial-config fixture directories are now inert from the deny checker’s perspective. If a future test harness wants them to be live deny roots again, it should materialize them in a temp tree rather than reintroduce repo-local nested `deny.toml` files.

## Key Files for Context
- `apps/guardrail3/deny.toml` — the real live app-root cargo-deny policy for the repo.
- `apps/guardrail3/crates/domain/modules/deny.rs` — canonical generator baseline that defines the expected ban set, including `lazy_static`.
- `apps/guardrail3/crates/app/rs/families/deny/README.md` — deny family status and remaining known issues.
- `apps/guardrail3/tests/fixtures/adversarial-configs/incomplete-deny-bans/deny.fixture.toml` — inert deny fixture after the rename.
- `apps/guardrail3/tests/fixtures/adversarial-configs/missing-deny-licenses/deny.fixture.toml` — inert deny fixture after the rename.
- `apps/guardrail3/tests/fixtures/adversarial-configs/missing-deny-sources/deny.fixture.toml` — inert deny fixture after the rename.
- `.worklogs/2026-03-29-204728-restore-inventory-contracts.md` — previous commit that cleaned the finished-family inventory contract and established the current baseline before this deny fix.

## Next Steps / Continuation Plan
1. Continue with the remaining live families in priority order: `code`, then `release`.
2. For `RS-CODE`, use the parallel allow-suppression audit to separate removable `#[allow]`s from the smaller set that need real `// reason:` comments.
3. Revisit `RS-DENY` later for the still-open profile fail-open issue in `facts.rs` and the deny hardening matrix, but do not reintroduce nested live deny configs to make self-hosting “look complete.”
