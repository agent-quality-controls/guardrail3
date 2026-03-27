# Close Clippy Cargo Config Override Gap

**Date:** 2026-03-27 22:17
**Scope:** `apps/guardrail3/crates/app/core/project_walker.rs`, `apps/guardrail3/crates/app/core/project_walker_tests.rs`, `apps/guardrail3/crates/app/rs/families/clippy/**`, `.plans/todo/checks/rs/clippy.md`

## Summary
Added `RS-CLIPPY-24` to forbid applicable `.cargo/config.toml` / legacy `.cargo/config` surfaces that set `CLIPPY_CONF_DIR`, cached those files into `ProjectTree`, and expanded attack coverage to include member-local override surfaces plus an unrelated nested non-hit. Updated the live clippy plan and family README to document the new rule and 24-rule inventory.

## Context & Problem
The clippy family had already been hardened around same-root precedence and policy-context parsing, but one real completeness gap remained: Clippy honors `CLIPPY_CONF_DIR`, and repo-local Cargo config files can set that environment variable. The family previously modeled only `clippy.toml` / `.clippy.toml` placement and coverage, so a routed workspace could still redirect Clippy config discovery through `.cargo/config.toml`.

An initial implementation of `RS-CLIPPY-24` handled validation-root and routed-root `.cargo/config*` files, but attack review found the first version was still too narrow. It missed member-local `.cargo/config*` under a routed workspace, which is exactly the kind of per-crate override surface that can bypass the routed policy-root model while leaving top-level configs looking clean.

## Decisions Made

### Add a dedicated clippy-owned rule for `CLIPPY_CONF_DIR`
- **Chose:** Add `RS-CLIPPY-24` in the clippy family rather than pushing this into cargo or generic input-failure handling.
- **Why:** The problem is specifically about Clippy config discovery being redirected. `RS-CARGO` owns lint-baseline policy in `Cargo.toml`, but it does not own Clippy config search semantics.
- **Alternatives considered:**
  - Treat all `.cargo/config*` parsing as cargo-family debt — rejected because the concrete bypass here is Clippy-specific.
  - Ignore `.cargo/config*` and document the gap — rejected because it leaves a real false negative in a family we are actively trying to harden.

### Cache `.cargo/config.toml` and legacy `.cargo/config` in `ProjectTree`
- **Chose:** Extend `project_walker` caching to include both cargo config file variants.
- **Why:** The clippy family needs those files available from `ProjectTree` to stay within the current shared discovery architecture.
- **Alternatives considered:**
  - Re-read files directly from disk in the clippy family — rejected because it violates the current `ProjectTree`-first discovery model.
  - Cache only `config.toml` — rejected because legacy `.cargo/config` is still a valid Cargo config surface and can carry the same override.

### Model “applicable cargo config surface” by in-scope Cargo roots, not only routed roots
- **Chose:** Treat a `.cargo/config*` as relevant when its owning directory is an ancestor of any in-scope Cargo root, including member package roots inside a routed workspace.
- **Why:** Cargo config under `workspace/member/.cargo/` can affect that member even when the routed family input is the workspace root. Limiting the rule to routed roots or their ancestors misses exactly that case.
- **Alternatives considered:**
  - Only inspect validation-root and routed-root `.cargo` directories — rejected because it leaves member-level false negatives.
  - Inspect every `.cargo/config*` anywhere under the repo — rejected because it would over-report unrelated nested docs/tooling directories with no in-scope Cargo roots below them.

### Fail closed on malformed applicable cargo config surfaces
- **Chose:** Emit `RS-CLIPPY-24` when an applicable `.cargo/config*` cannot be parsed or is missing from `ProjectTree`.
- **Why:** If the family cannot prove whether `CLIPPY_CONF_DIR` is present on an applicable cargo-config surface, it should not silently assume the override is absent.
- **Alternatives considered:**
  - Ignore malformed cargo config here and rely on other families — rejected because it would reintroduce the exact “unmodeled override surface” problem the rule is meant to close.

## Architectural Notes
This change stays within the current Rust family split:

- `placement` still decides which Rust roots exist.
- `FamilyMapper` still routes roots to `clippy`.
- `RS-CLIPPY` now does family-local discovery of two input surfaces inside routed scope:
  - `clippy.toml` / `.clippy.toml`
  - applicable `.cargo/config.toml` / `.cargo/config`

The important boundary choice is that the family does **not** treat every cargo config in the repo as relevant. It projects applicability through discovered in-scope Cargo roots, which keeps the rule deterministic and prevents unrelated nested tool directories from becoming false positives.

## Information Sources
- Live family implementation and tests:
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_24_forbid_clippy_conf_dir_override.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/test_support/src/lib.rs`
- Shared discovery layer:
  - `apps/guardrail3/crates/app/core/project_walker.rs`
  - `apps/guardrail3/crates/app/core/project_walker_tests.rs`
- Live rule inventory and family docs:
  - `.plans/todo/checks/rs/clippy.md`
  - `apps/guardrail3/crates/app/rs/families/clippy/README.md`
- Prior clippy hardening worklogs:
  - `.worklogs/2026-03-27-215144-fix-clippy-same-root-precedence.md`
  - `.worklogs/2026-03-27-215839-harden-clippy-doc-drift-and-dotfile-coverage.md`

## Open Questions / Future Considerations
- The strongest remaining clippy completeness gap is likely no longer same-root precedence or `CLIPPY_CONF_DIR`. The next attack pass should look for other repo-local override surfaces or rule/document drift.
- Direct top-level validator reruns for `RS-CLIPPY` and `RS-TEST` are still blocked by unrelated in-flight workspace breakage in other families. Nested clippy workspace tests remain the trustworthy checkpoint for now.

## Key Files for Context
- `apps/guardrail3/crates/app/core/project_walker.rs` — shared `ProjectTree` cache policy, now including cargo config files
- `apps/guardrail3/crates/app/core/project_walker_tests.rs` — regression proving cargo config files are cached
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs` — clippy fact collection, including applicable cargo-config override discovery
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_24_forbid_clippy_conf_dir_override.rs` — new rule implementation
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_24_forbid_clippy_conf_dir_override_tests/mod.rs` — rule attack coverage, including member-root and unrelated-nested cases
- `apps/guardrail3/crates/app/rs/families/clippy/test_support/src/lib.rs` — family fixture builders for policy-root and cargo-config scenarios
- `.plans/todo/checks/rs/clippy.md` — live clippy source of truth, now 24 rules
- `apps/guardrail3/crates/app/rs/families/clippy/README.md` — family-local contract and current shape
- `.worklogs/2026-03-27-215144-fix-clippy-same-root-precedence.md` — prior same-root precedence fix
- `.worklogs/2026-03-27-215839-harden-clippy-doc-drift-and-dotfile-coverage.md` — prior dotfile/documentation hardening

## Next Steps / Continuation Plan
1. Once the outer app workspace is healthy again, rerun top-level `RS-TEST` and `RS-CLIPPY` validation on `apps/guardrail3/crates/app/rs/families/clippy` to confirm the nested-workspace green state survives full validator entrypoints.
2. Run another adversarial clippy pass focused on any remaining config-discovery surfaces beyond `CLIPPY_CONF_DIR`, using `.plans/by_file/tools/edge-cases/clippy.md` only as historical research, not source of truth.
3. If no more concrete clippy bugs surface, shift the local primary lane to the next high-context family while leaving `deps` / `garde` handoffs to other agents.
