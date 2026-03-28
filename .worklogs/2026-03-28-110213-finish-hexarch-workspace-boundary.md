# Finish Hexarch Workspace Boundary

**Date:** 2026-03-28 11:03
**Scope:** apps/guardrail3 workspace membership, nested family workspace removal, RS-TEST root discovery for family container layouts, RS-HEXARCH workspace-boundary assertions/tests, boundary handoff docs

## Summary
Removed the temporary nested Cargo workspaces from Rust family container directories under `apps/guardrail3`, restored top-level Cargo metadata health, and finished the `RS-HEXARCH` workspace-boundary handoff by proving the live app root is clean while synthetic reintroduced nested workspaces fail under `RS-HEXARCH-07` and `RS-HEXARCH-27`. To make the new layout self-host honestly, `RS-TEST` discovery was broadened to recognize family container roots that no longer have a parent `Cargo.toml`, and the affected `hexarch` assertions/tests were tightened so the family remains clean under `RS-TEST`.

## Context & Problem
The prior checkpoint only taught `hexarch` to detect nested family workspaces. It did not actually remove them, so the repo still violated the rule and top-level Cargo metadata remained broken with multiple workspace roots inside `apps/guardrail3`. The user explicitly called this out: the task was not detection, it was full migration plus adversarial verification until `hexarch` was either wrong or clean.

The family self-hosting migration earlier had introduced virtual workspace manifests like `apps/guardrail3/crates/app/rs/families/<family>/Cargo.toml` as a convenience so `cargo test --manifest-path .../families/<family>/Cargo.toml --workspace` worked. Once those family runtime/assertions/test_support crates were also members of the top-level `apps/guardrail3` workspace, those extra `[workspace]` manifests became architectural debt and a real `RS-HEXARCH-27` violation.

## Decisions Made

### Remove nested family workspaces instead of carving out a hexarch exemption
- **Chose:** Delete the family-root `Cargo.toml` workspace manifests under `apps/guardrail3/crates/app/rs/families/*` and keep only the actual crates as members of the top-level app workspace.
- **Why:** The handoff contract is explicit that app root must be the only workspace root inside the app boundary. Exempting family containers would make `RS-HEXARCH-27` dishonest and keep Cargo metadata broken.
- **Alternatives considered:**
  - Preserve nested family workspaces and special-case them in `RS-HEXARCH` — rejected because it would codify the architectural violation instead of fixing it.
  - Move family workspaces outside the app boundary immediately — rejected for this checkpoint because it is a broader package-grammar redesign, not the focused workspace-boundary fix the handoff asked for.

### Teach RS-TEST to recognize family container roots without a parent Cargo manifest
- **Chose:** Group `crates/runtime`, `crates/assertions`, `crates/assertions_common`, and `test_support` cargo roots back to their family container root in `rs/test` discovery, with fallback root facts sourced from `crates/runtime/Cargo.toml` when the container directory itself has no `Cargo.toml`.
- **Why:** Once the fake family workspace manifest is removed, `RS-TEST` must still understand that `families/<name>/` is the logical root owning the runtime/assertions split.
- **Alternatives considered:**
  - Restore placeholder family `Cargo.toml` files just for `RS-TEST` discovery — rejected because it recreates the very nested-workspace violation being removed.
  - Make every family test run target the runtime crate root directly — rejected because it would weaken the family-container ownership model already encoded in the `RS-TEST` contract.

### Keep rule-27 cross-rule ownership proof inside owned assertions
- **Chose:** Move the “rule 07 stays quiet/noisy in exactly this way” checks for `RS-HEXARCH-27` into the owned rule-27 assertions module, and make the rule-26 assertion helper directly proof-bearing instead of delegating proof entirely into `assertions_common`.
- **Why:** The workspace-boundary change exposed two real `RS-TEST` failures in `hexarch`. Fixing them inside owned assertions preserves the stricter test-family contract instead of weakening `RS-TEST`.
- **Alternatives considered:**
  - Let the sidecar import sibling rule-07 assertions directly — rejected because `RS-TEST-03` correctly forbids that boundary escape.
  - Relax proof detection for thin wrappers into `assertions_common` — rejected because that would reopen the exact false-green hole previously fixed in `RS-TEST`.

## Architectural Notes
The end state is now consistent with the hexarch handoff and the broader Rust routing architecture:

- `apps/guardrail3` is the only Cargo workspace root inside the app boundary.
- Family container directories are structural containers only; they are not nested Cargo workspaces.
- `RS-HEXARCH` owns the app-local workspace boundary honestly:
  - every live app-local Cargo root must be covered by the app workspace (`RS-HEXARCH-07`)
  - nested workspaces are forbidden (`RS-HEXARCH-27`)
- `RS-TEST` now models the family-container layout without relying on a fake parent Cargo manifest.

This also restores the intended separation between product topology and self-hosting ergonomics: the app workspace remains the real build topology, while runtime/assertions/test_support remain the family-internal test architecture.

## Information Sources
- `.plans/todo/hexarch-workspace-boundary-handoff.md` — explicit contract for this checkpoint
- `apps/guardrail3/crates/app/rs/families/hexarch/README.md` — live family contract and ownership split
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover.rs` — root discovery logic that had to accept container roots without a parent Cargo manifest
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_27_nested_workspace_forbidden_tests/ownership.rs` — live-repo ownership proof for the app boundary
- `apps/guardrail3/Cargo.toml` — top-level workspace membership that now fully replaces the deleted nested family workspaces
- CLI verification runs:
  - `cargo metadata --manifest-path apps/guardrail3/Cargo.toml --no-deps`
  - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family hexarch --inventory --format json`
  - temp-repo `rs validate ... --family hexarch` attacks with reintroduced nested family workspaces, package-only nested roots, and excluded `target/` workspaces

## Open Questions / Future Considerations
- Several family READMEs and handoff docs outside `hexarch` still mention family-root `Cargo.toml` workspaces. They are now stale and should be updated in follow-up cleanup.
- Repo-root `rs validate . --family hexarch` still reports broader hexarch findings and many `RS-HEXARCH-14` infos under family crates. That is separate from the workspace-boundary fix and should be reviewed on its own merits.
- The broader “what package grammar is actually allowed inside an app?” discussion remains intentionally out of scope for this checkpoint.

## Key Files for Context
- `apps/guardrail3/Cargo.toml` — top-level workspace members after nested family workspace removal
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover.rs` — family-container root discovery without parent Cargo manifests
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split_tests/boundaries.rs` — regression proving the new family container shape is valid under `RS-TEST`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/rs_hexarch_26_member_manifest_parse_error.rs` — proof-bearing fix for rule 26 assertions
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/rs_hexarch_27_nested_workspace_forbidden.rs` — owned rule-27 assertions, including cross-rule workspace-membership checks
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_27_nested_workspace_forbidden_tests/ownership.rs` — live repo and ownership boundary tests for nested workspaces
- `apps/guardrail3/crates/app/rs/families/hexarch/README.md` — updated current shape: family container is not a nested workspace root
- `.plans/todo/hexarch-workspace-boundary-handoff.md` — updated verification and acceptance criteria for the finished state
- `.worklogs/2026-03-27-233207-hexarch-workspace-boundary-enforcement.md` — prior checkpoint that stopped too early at detection

## Next Steps / Continuation Plan
1. Update the stale family READMEs and handoff docs that still describe deleted family-root `Cargo.toml` workspaces, starting with `toolchain`, `clippy`, and `deny`.
2. Review repo-root `RS-HEXARCH-14` infos under `apps/guardrail3/crates/app/rs/families/*` and decide whether inventorying those family-internal path dependencies is the intended top-level behavior or another architectural leak.
3. Continue the broader `hexarch` architecture audit from this now-clean baseline: workspace boundary is closed, so the next attack surface is hidden repo/global inputs and any remaining implicit scope inside `facts.rs` / `dependency_facts.rs`.
