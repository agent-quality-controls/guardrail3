# Tighten Hexarch Attack Coverage

**Date:** 2026-03-26 22:48
**Scope:** `.plans/todo/checks/rs/hexarch.md`, `apps/guardrail3/crates/app/rs/families/hexarch/README.md`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/*`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/*`

## Summary
Adversarially attacked `RS-HEXARCH` against the live `guardrail3` repo and `/Users/tartakovsky/Projects/steady-parent`, then fixed the concrete false greens and one live false positive that surfaced. The family now fails closed on malformed member manifests, respects explicit Cargo source entrypoints, limits dependency/workspace discovery to owned app scope, recovers missing `rust.apps.*` boundary warnings through schema-invalid but syntactically valid config, and no longer warns on ports crates just because helper-only traitless modules contain impl blocks.

## Context & Problem
The user asked for a serious attack pass on whether `hexarch` actually enforces hex architecture correctly, first on this repo and then on `steady-parent`, which is known to be highly non-compliant. Earlier work had already moved the family onto routed root scope and made it pass `RS-TEST`, but that did not prove the rule family itself was complete or correct. The attack goal here was to look for false greens, false positives, scope leaks, and contract drift between implementation and the `RS-HEXARCH` plan.

## Decisions Made

### Fail closed on dependency analysis blockers
- **Chose:** Add explicit member-manifest failure facts and a new runtime rule `RS-HEXARCH-26` to surface malformed workspace-member `Cargo.toml` files instead of silently skipping dependency checks.
- **Why:** The dependency collector was previously skipping parse failures, which created real false greens for rules `13`, `14`, `16`, `17`, `18`, `19`, `20`, `21`, `24`, and `25`.
- **Alternatives considered:**
  - Silently keep skipping malformed members — rejected because it hides exactly the dependency checks the family claims to enforce.
  - Fold the failure into an existing rule — rejected because the blocked-verification condition is cross-cutting and deserved its own stable fail-closed ID.

### Scope workspace/member discovery to owned app hex roots
- **Chose:** Restrict dependency/workspace discovery to the routed app root itself and descendants under `<app>/crates/...`.
- **Why:** Attack review showed the collector could wander into fixture-like or incidental workspaces under an owned app root, which is not valid `hexarch` scope.
- **Alternatives considered:**
  - Keep broad recursive discovery and rely on exclusions — rejected because that recreates the same scope leak class that `placement`/`FamilyMapper` were designed to prevent.
  - Push all inner discovery into `FamilyMapper` — rejected because member/dependency/source discovery is still family-local work inside routed roots.

### Recover boundary-config coverage from schema-invalid but parseable TOML
- **Chose:** Separate raw TOML parse success from typed config validation, salvage `rust.apps.*` keys from syntactically valid TOML, and still emit missing-boundary warnings for routed apps when typed config validation fails.
- **Why:** `steady-parent` had a real schema-invalid `guardrail3.toml` that was masking missing app-boundary findings even though the raw file still exposed the relevant `rust.apps.*` structure.
- **Alternatives considered:**
  - Abort all boundary checks on any config validation failure — rejected because it loses materially useful fail-closed information.
  - Ignore validation errors and rely only on raw TOML — rejected because typed config validation is still necessary and should still surface an explicit blocking warning.

### Respect explicit Cargo target entrypoints for source rules
- **Chose:** Teach source collection to read `[lib].path` and `[[bin]].path` from member manifests and use those as source entrypoints before falling back to `src/lib.rs` / `src/main.rs`.
- **Why:** Attack repros showed that `RS-HEXARCH-22/23` were skipping valid crates that use explicit target paths, including a live adapter violation in this repo.
- **Alternatives considered:**
  - Keep hardcoded `src/` entrypoints and narrow the contract — rejected because that would make the checker less correct than Cargo’s real crate model.
  - Scan all `.rs` files under the crate root — rejected because it would overcount orphan/unreachable files and create new false positives.

### Narrow ports trait-dominance to trait-bearing modules
- **Chose:** Count impl blocks only inside reachable module subtrees that actually define traits; traitless helper/DTO/error modules do not make the crate impl-heavy on their own.
- **Why:** The live repo produced a false-positive `RS-HEXARCH-22` on `apps/guardrail3/crates/ports/outbound/traits` entirely because helper data types in `fs_types.rs` had impl blocks. The plan explicitly says DTOs and error types are okay.
- **Alternatives considered:**
  - Keep counting all impl blocks — rejected because it directly contradicted the documented rule intent.
  - Try to classify DTO/error types by name — rejected because that would be heuristic-heavy and brittle.

### Bring plan/docs back into parity with the code
- **Chose:** Update the hexarch rule inventory and family README to describe 26 rules and the narrowed `RS-HEXARCH-22` heuristic.
- **Why:** After adding `RS-HEXARCH-26` and tightening rule 22, the docs were materially stale and would mislead future audits.
- **Alternatives considered:**
  - Leave docs for later — rejected because this attack round was explicitly about enforcement-vs-contract parity.

## Architectural Notes
The final shape keeps the intended architecture boundary intact:
- shared `placement` and `FamilyMapper` still decide outer Rust root scope
- `hexarch` still owns family-local discovery inside routed app roots
- repo-level support surfaces remain explicit and narrow (`RS-HEXARCH-11` root `Cargo.toml`, `RS-HEXARCH-15` root `guardrail3.toml`)

The key architectural outcome of this round is that `hexarch` no longer widens scope accidentally and no longer hides blocked verification for member dependency analysis. The source rules also now follow Cargo’s real target-entrypoint model instead of a narrower `src/` convention.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/dependency_facts.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/source_facts.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_15_boundary_config.rs`
- `.plans/todo/checks/rs/hexarch.md`
- `apps/guardrail3/crates/app/rs/families/hexarch/README.md`
- Live repo run: `guardrail3 rs validate . --family hexarch --inventory --format json`
- Steady Parent run: `guardrail3 rs validate /Users/tartakovsky/Projects/steady-parent --family hexarch --inventory --format json`
- Prior worklogs:
  - `.worklogs/2026-03-26-221510-fix-hexarch-route-fail-closed.md`
  - `.worklogs/2026-03-26-220354-fix-arch-runtime-fail-closed.md`
  - `.worklogs/2026-03-26-205131-hexarch-rs-test-fallout-fix.md`

## Open Questions / Future Considerations
- `steady-parent` is still highly non-compliant, but the remaining open question there is no longer missing boundary warnings. The next audit could inspect whether any of its dependency-direction findings are still masked by the invalid config beyond boundary presence itself.
- `crates/assertions_common` remains documented as a current compromise, not a permanently blessed primitive. It still deserves a narrower future audit.
- `RS-HEXARCH-22` now uses a structural proxy that is substantially better than raw impl counting, but it is still a proxy. If future repos expose another false-positive/false-negative pattern, that rule may need another deterministic refinement.

## Key Files for Context
- `.plans/todo/checks/rs/hexarch.md` — current contract and rule inventory for `RS-HEXARCH`
- `apps/guardrail3/crates/app/rs/families/hexarch/README.md` — family-local architecture and ownership boundaries
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/dependency_facts.rs` — workspace/member/dependency collection and fail-closed handling
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/source_facts.rs` — source entrypoint resolution and ports/adapter source stats
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs` — rule wiring and orchestrator fan-out
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_15_boundary_config.rs` — boundary-config fail-closed behavior
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_26_member_manifest_parse_error.rs` — new explicit dependency-blocker rule
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/dependency_facts_tests/scope_boundaries.rs` — regression for owned-scope discovery boundaries
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_15_boundary_config_tests/schema_invalid_partial_recovery.rs` — regression for schema-invalid but parseable config recovery
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_15_boundary_config_tests/zero_member_workspace.rs` — regression for empty-workspace boundary coverage
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_22_ports_trait_dominance_tests/source_layout.rs` — regression for helper-only traitless modules not triggering impl-heaviness
- `.worklogs/2026-03-26-221510-fix-hexarch-route-fail-closed.md` — previous route/fail-closed checkpoint for `hexarch`

## Next Steps / Continuation Plan
1. Run the same adversarial completeness audit on the remaining Rust families that still have scope-heavy logic, especially `deps` and `release`, now that `test`, `arch`, and `hexarch` have all had dedicated attack rounds.
2. Revisit `crates/assertions_common` in `hexarch` specifically: verify it remains assertions-only and does not become a new backdoor for route construction or scenario generation.
3. If another repo is used as an external probe target, preserve the same method from this round: compare the live result set to the repo’s real structure, explicitly hunt false greens, and only then widen or tighten rules.
