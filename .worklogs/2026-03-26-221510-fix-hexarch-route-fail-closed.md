# Fix Hexarch Route Fail-Closed

**Date:** 2026-03-26 22:15
**Scope:** `apps/guardrail3/crates/app/rs/runtime.rs`, `apps/guardrail3/crates/app/rs/runtime_tests.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/facts.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/dependency_facts.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_20_dev_dependency_direction.rs`, `apps/guardrail3/crates/app/rs/README.md`, `apps/guardrail3/crates/app/rs/families/hexarch/README.md`

## Summary
Aligned `RS-HEXARCH` with the routed-family contract by making repo-root `Cargo.toml` and `guardrail3.toml` explicit `RsHexarchRoute` inputs instead of hidden runtime reads. Also fixed the top-level Rust runtime so explicit `--family hexarch` runs fail closed on malformed `guardrail3.toml` instead of aborting before `RS-HEXARCH-15` can report.

## Context & Problem
After the `arch` fail-closed fix, the next audit target was `hexarch`. The family README said `hexarch` only consumed routed app roots from `FamilyMapper`, but the runtime collectors were still reading repo-root `Cargo.toml` for `RS-HEXARCH-11` and repo-root `guardrail3.toml` for `RS-HEXARCH-15` without those surfaces appearing in `RsHexarchRoute`. That meant the family contract and the implementation were drifting apart again.

While tracing those reads, I also reproduced an end-to-end bug: `guardrail3 rs validate <repo> --family hexarch` aborted on malformed `guardrail3.toml` at the top-level runtime before `RS-HEXARCH-15` could warn fail closed. Family tests passed only because they bypassed the outer runtime and called `hexarch::check(...)` directly with a test tree.

## Decisions Made

### Make repo-level support files explicit route inputs
- **Chose:** Extend `RsHexarchRoute` with `repo_root_cargo_rel_path` and `guardrail_config_rel_path`.
- **Why:** `RS-HEXARCH-11` and `RS-HEXARCH-15` really do own repo-level surfaces. Hiding those reads inside runtime collectors violated the `FamilyMapper` contract and made the family look narrower than it is.
- **Alternatives considered:**
  - Keep the hidden reads and weaken the README — rejected because it would reintroduce the same “family quietly decides extra scope” bug class we just spent time removing.
  - Move rules 11 and 15 out of `hexarch` — rejected because that is a larger inventory change and the current family/tests clearly treat those as `RS-HEXARCH` responsibilities.

### Keep root-workspace/config parsing inside the family, but only when routed
- **Chose:** Gate repo-root `Cargo.toml` parsing in `facts.rs` behind `route.repo_root_cargo_rel_path`, and gate repo-root `guardrail3.toml` parsing plus root-workspace discovery in `dependency_facts.rs` behind route fields.
- **Why:** This preserves family-local parsing/fail-closed behavior while making the allowed surfaces explicit in the route contract.
- **Alternatives considered:**
  - Parse those files in `FamilyMapper` and pass parsed facts directly — rejected because the mapper is supposed to map, not become another family-specific parser.
  - Let the runtime continue to scan the whole tree and filter later — rejected because it keeps the contract blurry and makes future audits harder.

### Allow malformed config to fall through for explicit `hexarch` runs
- **Chose:** Generalize the top-level runtime exception from `requested_families == [Arch]` to “all explicitly requested families are fail-closed-safe on malformed config”, currently `Arch` and `Hexarch`.
- **Why:** `hexarch` rule 15 is explicitly designed to warn on malformed `guardrail3.toml`; aborting earlier was wrong.
- **Alternatives considered:**
  - Continue aborting for `hexarch` and rely on family-only tests — rejected because it hides a real CLI/runtime bug.
  - Never abort on malformed config for any Rust run — rejected for now because selection/applicability for broader multi-family runs still depends on parsed config and that larger policy change was not part of this checkpoint.

### Centralize `hexarch` test-route construction
- **Chose:** Make the remaining test-only helper paths reuse `family_route_for_tests(...)` instead of rebuilding mapper/placement wiring locally.
- **Why:** It reduces one more place where `hexarch` test helpers could drift from the live route contract.
- **Alternatives considered:**
  - Leave the duplicate test-only mapper calls in place — rejected because they make the family harder to audit and defeat the point of the route cleanup.

## Architectural Notes
This checkpoint narrows a specific gap in the shared Rust architecture:

- `placement` still owns live Rust root discovery.
- `FamilyMapper` now explicitly maps `hexarch` not just to app roots, but also to the two repo-level support files the family genuinely owns.
- `hexarch` runtime still parses those files itself, but only because the route tells it those inputs are in-bounds.

This keeps the mapper as a mapper while making the family’s true input surface visible and testable. It also avoids a false purity rule where repo-level checks exist but the route contract pretends they do not.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/hexarch/README.md` — stated contract that triggered the mismatch review
- `apps/guardrail3/crates/app/rs/README.md` — shared Rust placement/mapper contract
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/facts.rs` — hidden repo-root `Cargo.toml` read for rule 11
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/dependency_facts.rs` — hidden repo-root `guardrail3.toml` read and workspace discovery
- `apps/guardrail3/crates/app/rs/runtime.rs` — top-level malformed-config abort behavior
- `apps/guardrail3/crates/app/rs/runtime_tests.rs` — runtime-level regression coverage
- `.worklogs/2026-03-26-220354-fix-arch-runtime-fail-closed.md` — prior fail-closed fix for `arch`
- `.worklogs/2026-03-26-212058-document-arch-and-hexarch-readmes.md` and `.worklogs/2026-03-26-214746-tighten-arch-and-hexarch-readme-wording.md` — doc context leading into this audit

## Open Questions / Future Considerations
- The top-level runtime still aborts on malformed `guardrail3.toml` for broader multi-family runs. That is probably still correct today, but the long-term plan may want a cleaner capability-based policy instead of the current explicit family allowlist.
- `hexarch` still performs family-local workspace/member discovery using `ProjectTree::dirs_with_file("Cargo.toml")` and then filters to routed surfaces. That is acceptable under the current contract, but it remains an area worth re-auditing if the route layer becomes even stricter.
- `assertions_common` remains documented as current shape under audit. This checkpoint does not bless it further; it only resolves the route/fail-closed mismatch.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/runtime.rs` — top-level Rust runtime dispatch and malformed-config handling
- `apps/guardrail3/crates/app/rs/runtime_tests.rs` — runtime-level regressions for `arch` and `hexarch`
- `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs` — route view definitions, including `RsHexarchRoute`
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — mapper logic that now explicitly routes repo-level `hexarch` support files
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/facts.rs` — routed root-workspace collector for rule 11
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/dependency_facts.rs` — routed config/workspace dependency collector for rules 13-25 and rule 15 fail-closed
- `apps/guardrail3/crates/app/rs/families/hexarch/README.md` — family-local contract after this change
- `apps/guardrail3/crates/app/rs/README.md` — shared placement/mapper contract after this change
- `.worklogs/2026-03-26-220354-fix-arch-runtime-fail-closed.md` — prior related work

## Next Steps / Continuation Plan
1. Re-run the adversarial architecture audit on `hexarch` now that route surfaces are explicit, with special attention on any remaining hidden repo/global reads outside `RsHexarchRoute`.
2. Continue the shared Rust architecture cleanup by finding the next family that still owns hidden root/config scope and either route those surfaces explicitly or move the rule ownership.
3. Revisit top-level malformed-config handling for multi-family runs once more families have explicit fail-closed contracts, so runtime policy is not a one-off allowlist forever.
