# Refresh package TODOs to current extracted state

**Date:** 2026-04-05 20:40
**Scope:** package TODO files under packages/g3-*-content-checks

## Summary
Updated the content-check package TODO files so they reflect the current extracted boundaries and real remaining issues instead of stale parser-drift and already-fixed notes. The TODOs now record only live deferred rule splits, current package limits, and boundary guards that still matter.

## Context & Problem
Several package TODO files had gone stale as extraction work progressed. `fmt` and `toolchain` still described parser API drift that had already been fixed. `deps` did not record the newly discovered family-view limitation around external local Cargo targets. The user asked to document current issues in the corresponding package TODOs, so the package-local files needed to become the authoritative lightweight record of what is still deferred or constrained.

## Decisions Made

### Replace stale issue notes with live boundary/deferred-work notes
- **Chose:** Rewrite the TODOs around current app/package boundaries and live limitations.
- **Why:** A TODO file should tell the next agent what is still true, not preserve historical bugs that are already closed.
- **Alternatives considered:**
  - Appending more notes under stale sections — rejected because it leaves misleading resolved problems in place.
  - Leaving history only in worklogs — rejected because the user explicitly wanted package-local TODOs to carry these issues.

### Keep TODOs focused on package-local concerns
- **Chose:** Record only deferred boundary work, current extracted slices, and real package limitations.
- **Why:** These files should help the next agent continue package work without rereading all history.
- **Alternatives considered:**
  - Turning each TODO into a mini changelog — rejected because worklogs already carry that history.

## Architectural Notes
The TODO cleanup reinforces the active architecture:
- app families own structural rules, discovery, and malformed-input ownership
- content-check packages own pure validation over parsed files only
- deferred moves are documented only where the package boundary is intentionally incomplete

This also makes the package-local TODOs consistent with the current extracted family state:
- `fmt`: content package only, waiver/placement rules still app-side
- `toolchain`: content package only, structural rules still app-side
- `clippy`: typed `clippy.toml` slice only, parse gate and cargo-config override still app-side
- `cargo`: single-file `Cargo.toml` content package only
- `deny`: structural typed-parse signaling gap still in app
- `deps`: local-path family-view limitation documented, structural ownership still app-side

## Information Sources
- Current package TODO files under `packages/g3-*-content-checks/TODO.md`
- Current package input contracts in each `crates/types/src/lib.rs` or `src/input.rs`
- Current package/runtime state from recent deps parity worklog `.worklogs/2026-04-05-203355-deps-local-path-parity.md`
- Earlier extraction worklogs in `.worklogs/2026-04-05-200711-wire-deps-content-checks.md` and related package extraction logs

## Open Questions / Future Considerations
- `cargo` still has policy-sensitive rules deferred until a full parsed policy-file boundary is ready.
- `clippy` may need more family bridge smoke if future work touches multi-result migrated rules.
- `deny` still needs app-side structural signaling for typed schema rejection.
- `deps` still has the scoped family-view limitation for sibling manifests outside the selected route surface.

## Key Files for Context
- `packages/g3rs-cargo-config-checks/TODO.md` — current cargo package boundary and deferred rules
- `packages/g3rs-clippy-config-checks/TODO.md` — current clippy extracted slice and app-side deferred rules
- `packages/g3rs-deny-config-checks/TODO.md` — live deny structural parse-gap reminder
- `packages/g3rs-deps-config-checks/TODO.md` — deps local-path/family-view limitation and boundary guard
- `packages/g3rs-fmt-config-checks/TODO.md` — fmt package-only remaining deferred boundaries
- `packages/g3rs-toolchain-config-checks/TODO.md` — toolchain package-only remaining deferred boundaries
- `.worklogs/2026-04-05-203355-deps-local-path-parity.md` — latest deps hardening context

## Next Steps / Continuation Plan
1. Commit this TODO cleanup so the package-local status files are current before more family work starts.
2. Continue with `cargo` by reading the current package/runtime and app/runtime split, then decide which remaining cargo rules can move without violating the parsed-files-only boundary.
3. If cargo work stalls on policy-file ownership, record that explicitly in the cargo package TODO instead of inventing subset helper inputs.
