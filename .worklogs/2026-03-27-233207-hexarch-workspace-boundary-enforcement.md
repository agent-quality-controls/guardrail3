# Hexarch Workspace Boundary Enforcement

**Date:** 2026-03-27 23:32
**Scope:** `.plans/todo/checks/rs/hexarch.md`, `.plans/todo/hexarch-workspace-boundary-handoff.md`, `apps/guardrail3/crates/app/rs/families/hexarch/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/hexarch/README.md`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/rs_hexarch_27_nested_workspace_forbidden.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/facts.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/inputs.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_07_workspace_members_match_crate_dirs.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_09_no_extra_workspace_members.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_07_workspace_members_match_crate_dirs_tests/*`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_27_nested_workspace_forbidden.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_27_nested_workspace_forbidden_tests/*`

## Summary
Closed the `RS-HEXARCH` workspace-boundary hole by switching workspace coverage from old hex-leaf discovery to all live app-local `Cargo.toml` roots, and added a new explicit nested-workspace rule. The family now proves on the actual `guardrail3` repo tree that nested family workspaces under `apps/guardrail3/crates/app/rs/families/*` are architectural violations instead of silently slipping through.

## Context & Problem
The handoff in `.plans/todo/hexarch-workspace-boundary-handoff.md` called out a specific miss: `hexarch` only compared app workspaces against discovered hex leaf crate dirs. That left a blind spot for live Cargo roots outside the old leaf discovery path, especially the current nested family workspaces under `apps/guardrail3/crates/app/rs/families/*`.

The practical failure was severe:
- the top-level app could accumulate nested family workspaces
- app workspaces could point only at child member crates
- `RS-HEXARCH-07/08/09/10` still looked green because the nested workspace roots themselves were invisible

The goal here was deliberately narrow: close the app-root workspace boundary gap without redesigning the broader inner package grammar.

## Decisions Made

### Replace leaf-based workspace coverage with all live app-local Cargo roots
- **Chose:** Extend `WorkspaceCoverageFacts` to carry `app_local_cargo_roots` and collect every live descendant `Cargo.toml` under each routed app root, excluding the same ignored surfaces placement excludes.
- **Why:** The handoff requirement is about all live app-local Cargo roots, not only old hex leaves. Using the same app-local Cargo-root inventory for coverage rules removes the hidden alternate universe.
- **Alternatives considered:**
  - Keep `discovered_crate_dirs` and bolt on one special nested-family exception — rejected because it would preserve the core blind spot and just patch one current symptom.
  - Move this detection into `RS-ARCH` — rejected because this is app-local workspace topology, not repo-global Rust root placement.

### Keep `RS-HEXARCH-07` as the coverage rule and add a new nested-workspace rule
- **Chose:** Widen `RS-HEXARCH-07` so it now means “every live app-local Cargo root must be owned by the app workspace,” and add `RS-HEXARCH-27` for nested workspaces under the app root.
- **Why:** Coverage and nested-workspace prohibition are related but distinct failure modes. Overloading `08` or `09` would blur ownership and make future attack work harder.
- **Alternatives considered:**
  - Force `RS-HEXARCH-08` to own nested workspaces too — rejected because `08` is specifically the app-root workspace rule.
  - Split package-only and malformed roots into separate new rules — rejected because that would add ids without improving the boundary contract; `07` can own missing coverage for both valid and malformed nested roots.

### Fail closed on unreadable nested Cargo roots
- **Chose:** Treat app-local `Cargo.toml` files that exist in the tree but have no cached content as explicit workspace-boundary failures via `RS-HEXARCH-07`.
- **Why:** Unreadable active inputs were still a silent hole after the initial patch. If a nested Cargo root exists but cannot be read, the boundary checker must not degrade into “probably just a package.”
- **Alternatives considered:**
  - Leave unreadable nested manifests unclassified and only rely on parse failures — rejected because unreadable files are a separate fail-open path.
  - Add another dedicated nested-input-failure rule — rejected because unreadable nested roots are still fundamentally “live Cargo root not safely owned by the app workspace.”

### Prove the real repo, not just fixtures
- **Chose:** Add a family test that walks the actual repo root and asserts the current `apps/guardrail3` tree now emits `RS-HEXARCH-27` and `RS-HEXARCH-07` for the nested family workspace pattern.
- **Why:** The user requirement was specifically about the live `apps/guardrail3` sprawl. Fixture-only coverage was not enough.
- **Alternatives considered:**
  - Rely on `cargo run -p guardrail3 -- rs validate ...` — rejected for this checkpoint because Cargo itself currently aborts on the nested workspace roots before the binary can run.
  - Skip live-repo proof and trust fixture parity — rejected because that would not prove the handoff goal was actually reached on this repo.

### Make the family workspace directly testable again
- **Chose:** Add local workspace package/lint/dependency metadata to `families/hexarch/Cargo.toml`, including `workspace.dependencies.glob`.
- **Why:** The family workspace had drifted enough that `cargo test --manifest-path .../families/hexarch/Cargo.toml --workspace` no longer worked. Restoring a local verification path was necessary to finish the attack loop without touching unrelated top-level workspace breakage.
- **Alternatives considered:**
  - Only test through the top-level app workspace — rejected because the current nested-workspace state makes that impossible.
  - Patch unrelated family workspaces first — rejected because that would expand this checkpoint far beyond the handoff scope.

## Architectural Notes
The important boundary is now:
- `FamilyMapper::map_rs_hexarch()` still routes only app roots and explicit repo-level support files
- `hexarch` still owns app-local discovery inside those routed roots
- but workspace coverage is no longer leaf-shaped; it is based on every live descendant Cargo root inside the app

Rule ownership after this patch:
- `RS-HEXARCH-08`: app root `Cargo.toml` must be a valid workspace
- `RS-HEXARCH-07`: every live app-local Cargo root must be covered by `[workspace].members`
- `RS-HEXARCH-09`: no extra workspace members without matching live app-local Cargo roots
- `RS-HEXARCH-10`: workspace members must stay inside the permitted app-local boundary
- `RS-HEXARCH-27`: nested workspace roots under the app root are forbidden

That split keeps the failure modes explicit instead of smearing them into one giant workspace rule.

## Information Sources
- `.plans/todo/hexarch-workspace-boundary-handoff.md`
- `.plans/todo/checks/rs/hexarch.md`
- `apps/guardrail3/crates/app/rs/families/hexarch/README.md`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/facts.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_07_workspace_members_match_crate_dirs.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_08_app_cargo_is_workspace.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_09_no_extra_workspace_members.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_10_members_within_app_boundary.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs`
- recent worklogs:
  - `.worklogs/2026-03-27-230557-stabilize-deps-family-and-harden-input-failures.md`
  - `.worklogs/2026-03-27-230533-toolchain-readme-and-host-suffix-hardening.md`
  - `.worklogs/2026-03-27-230141-toolchain-attack-followup.md`

## Open Questions / Future Considerations
- `RS-HEXARCH-10` still enforces the historical “workspace members must resolve inside the app’s `crates/` tree” shape. This patch deliberately did not broaden that contract, so live Cargo roots outside `crates/` remain an intentionally harsh area rather than a newly solved grammar question.
- Top-level `cargo run -p guardrail3 -- rs validate ...` is still blocked by Cargo’s own nested-workspace abort before the binary can start. The new live-repo family test is the proof point for this checkpoint.
- The broader next-step discussion remains out of scope here: whether the allowed package grammar inside an app should keep nested family workspaces, role slices, or something else.

## Key Files for Context
- `.plans/todo/hexarch-workspace-boundary-handoff.md` — the exact handoff this checkpoint implemented
- `.plans/todo/checks/rs/hexarch.md` — current rule inventory, now including `RS-HEXARCH-27`
- `apps/guardrail3/crates/app/rs/families/hexarch/README.md` — family contract and routing/ownership boundaries
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/facts.rs` — all-live app-local Cargo-root collection and fail-closed unreadable handling
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/inputs.rs` — widened workspace-coverage input surface
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_07_workspace_members_match_crate_dirs.rs` — coverage enforcement over live app-local Cargo roots
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_27_nested_workspace_forbidden.rs` — explicit nested-workspace enforcement
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_07_workspace_members_match_crate_dirs_tests/discovery_boundaries.rs` — attack coverage for malformed, unreadable, excluded, and non-leaf nested Cargo roots
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_27_nested_workspace_forbidden_tests/ownership.rs` — family-style nested workspace tests plus the live repo proof
- `.worklogs/2026-03-27-233207-hexarch-workspace-boundary-enforcement.md` — this checkpoint

## Next Steps / Continuation Plan
1. Once the broader workspace is in a state where `guardrail3` can build again, rerun the actual CLI entrypoint on `apps/guardrail3` and capture the live `RS-HEXARCH-07/27` output from the binary, not just the family test.
2. Revisit the `RS-HEXARCH-10` contract if the next architecture discussion decides that app-local Cargo roots outside `crates/` should be expressible rather than intentionally trapped between `07` and `10`.
3. After the workspace-boundary hole is closed, move back to the larger hexarch package-grammar discussion from the handoff’s “notable open follow-up” section instead of adding more speculative boundary rules.
