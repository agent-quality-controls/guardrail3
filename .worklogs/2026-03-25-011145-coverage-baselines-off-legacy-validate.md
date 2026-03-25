# Move Rust coverage baselines off legacy validate ownership

**Date:** 2026-03-25 01:11
**Scope:** `apps/guardrail3/crates/adapters/inbound/cli/coverage/{clippy.rs,deny.rs}`, `apps/guardrail3/crates/app/rs/checks/rs/{clippy/mod.rs,deny/mod.rs,deny/deny_support.rs}`, `apps/guardrail3/crates/app/rs/families/{clippy,deny}/src/lib.rs`, `apps/guardrail3/crates/app/rs/validate/{clippy_coverage.rs,deny_audit.rs,deny_bans.rs}`

## Summary
Moved the clippy and deny coverage commands off legacy `app/rs/validate` baseline constants and onto the promoted Rust family owners. Also removed the stale duplicated deny ban baseline from the old validator path so coverage, family checks, and legacy validate now all read the same canonical sources.

## Context & Problem
After the AST split, the next review-backed backedge was inbound CLI coverage. `coverage/clippy.rs` still imported `EXPECTED_METHOD_BANS` / `EXPECTED_TYPE_BANS` from `app/rs/validate/clippy_coverage.rs`, and `coverage/deny.rs` still imported `EXPECTED_BANS` from `app/rs/validate/deny_audit.rs`. That meant the CLI was still coupled to the old validator owner even though the real family crates already existed.

There was also a correctness issue in the deny path: the old `deny_audit::EXPECTED_BANS` constant had drifted from the new deny family baseline derived from `domain/modules/deny`. Keeping both would preserve semantic divergence during the split.

## Decisions Made

### Expose coverage baselines through the family crates
- **Chose:** Re-export clippy and deny baseline access from the promoted family crates and point CLI coverage at those exports.
- **Why:** The family crates are the intended runtime owners for these rule families; the CLI should consume those owners directly rather than legacy validate helpers.
- **Alternatives considered:**
  - Add a new separate “coverage baselines” crate immediately — rejected because the family owners already had the required data and this cut is smaller.
  - Keep importing legacy validate constants until all coverage tools move together — rejected because clippy/deny were already clearly separable.

### Make old validate consume the family-owned deny baseline
- **Chose:** Update `validate/deny_bans.rs` to use `guardrail3_app_rs_family_deny::expected_ban_names(profile)` instead of a local duplicated constant.
- **Why:** The old validator still exists, but it should not own a second canonical deny baseline.
- **Alternatives considered:**
  - Leave old validate with its own constant and only switch the CLI — rejected because the constant had already drifted and would continue to rot.
  - Delete the old validator path entirely — rejected because too many callers still depend on it.

### Keep the public surface narrow
- **Chose:** Export only the exact baseline items needed at the family roots:
  - clippy: `EXPECTED_METHOD_BANS`, `EXPECTED_TYPE_BANS`
  - deny: `expected_ban_names(profile)`
- **Why:** This supports the CLI and old validator without leaking more private support structure than necessary.
- **Alternatives considered:**
  - Re-export private support modules wholesale — rejected because that would widen the public API for no reason.

## Architectural Notes
- `coverage/clippy.rs` now depends on `guardrail3_app_rs_family_clippy`.
- `coverage/deny.rs` now depends on `guardrail3_app_rs_family_deny`.
- `validate/clippy_coverage.rs` remains as a compatibility validator path, but its baseline constants are now re-exported from the clippy family owner.
- `validate/deny_audit.rs` no longer owns a ban baseline constant.
- `validate/deny_bans.rs` now resolves expected deny names from the deny family crate, which ultimately derives them from `domain/modules/deny`.

## Information Sources
- `.worklogs/2026-03-25-004019-runtime-applicability-and-rs-ast-split.md`
- `apps/guardrail3/crates/adapters/inbound/cli/coverage/{clippy.rs,deny.rs}`
- `apps/guardrail3/crates/app/rs/validate/{clippy_coverage.rs,deny_audit.rs,deny_bans.rs}`
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/clippy_support.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/deny/deny_support.rs`
- `apps/guardrail3/crates/domain/modules/deny.rs`

## Open Questions / Future Considerations
- Other coverage tools still need a pass to ensure they do not route through legacy owners accidentally.
- The old validator still has many runtime callers; this commit only removes the clippy/deny baseline ownership issue, not the broader validate surface.
- More family crates may need small root-level exports like this as adapters/CLI code is moved over.

## Key Files for Context
- `apps/guardrail3/crates/adapters/inbound/cli/coverage/clippy.rs` — CLI clippy coverage now reading family-owned baseline
- `apps/guardrail3/crates/adapters/inbound/cli/coverage/deny.rs` — CLI deny coverage now reading family-owned baseline
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/clippy_support.rs` — canonical clippy baseline owner inside the family
- `apps/guardrail3/crates/app/rs/checks/rs/deny/deny_support.rs` — canonical deny baseline owner inside the family
- `apps/guardrail3/crates/app/rs/families/clippy/src/lib.rs` — family-root export for CLI consumers
- `apps/guardrail3/crates/app/rs/families/deny/src/lib.rs` — family-root export for CLI/legacy consumers
- `apps/guardrail3/crates/app/rs/validate/deny_bans.rs` — old validator now consuming family-owned deny baseline
- `.worklogs/2026-03-25-004019-runtime-applicability-and-rs-ast-split.md` — prior split step that removed the AST backedge

## Next Steps / Continuation Plan
1. Audit the remaining coverage commands and CLI helpers for imports that still point at `app/rs/validate/**` or other root-facade compatibility paths.
2. Move the next smallest remaining shared legacy substrate out of `app/rs/validate`, likely config/coverage-related owners or other parser-free baseline helpers.
3. Continue reducing rootless family results so per-root runtime applicability has better fidelity across more families.
