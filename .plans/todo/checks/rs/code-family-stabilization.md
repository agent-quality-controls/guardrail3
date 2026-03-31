# RS-CODE Family Stabilization Plan

> Superseded as the primary family plan by [`.plans/by_family/rs/code.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_family/rs/code.md).
> Keep this file as tactical stabilization history only.

Historical note: this file still tracks the family-stabilization lane, but earlier wording about a family-root `code/Cargo.toml` workspace is obsolete. The live family shape is the package group described in [code/README.md](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/README.md): `crates/runtime`, `crates/assertions`, and `test_support` under a non-package family root.

This plan is about stabilizing the `RS-CODE` family itself as a self-hosted Rust family.

It is not the old rule-hardening lane.
The rule inventory and prior adversarial rule notes still live in:

- [code.md](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/code.md)
- [.plans/todo/check_review/test_hardening/02-code.md](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/check_review/test_hardening/02-code.md)

This document is the stabilization plan for the family itself. It should not be read as proof that live repo-root `RS-CODE` debt is gone; that repo cleanup is separate from family-correctness work.

## Current Snapshot

As of the current checkpoint:

- the production runtime lives under [crates/runtime/src](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/runtime/src)
- sibling packages exist and are populated at:
  - [crates/assertions](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/assertions)
  - [test_support](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/test_support)
- the family already consumes `RsCodeRoute` in [lib.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lib.rs)
- recent work has been dominated by correctness fixes from [FIXES.md](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/FIXES.md), not by creating empty scaffolding
- older exact pass/fail counts in this plan are historical and should be re-verified before reuse

## Why `RS-CODE` Next

This was the reason `RS-CODE` was the next structural target:

Compared to the remaining unstabilized Rust families at the time:

- `code` has `61` `RS-TEST` errors
- `release` has `59`
- `garde` has `29`
- `deps` has `23`

`RS-CODE` is the best next target because:

- it has the broadest repo-wide leverage on everyday source quality
- it already uses shared routing, so the architecture gap is mostly structural
- once stabilized, it becomes the next family that can pressure cleaner source shape across the repo

## Target End State

The family should end in this self-hosted package-group shape:

```text
apps/guardrail3/crates/app/rs/families/code/
  README.md
  crates/
    runtime/
      Cargo.toml
      src/
        lib.rs
        discover.rs
        facts.rs
        inputs.rs
        parse.rs
        rs_code_01_*.rs
        ...
        rs_code_30_*.rs
        rs_code_01_*_tests/
          mod.rs
        ...
        rs_code_30_*_tests/
          mod.rs
    assertions/
      Cargo.toml
      src/
        lib.rs
        rs_code_01_*.rs
        ...
        rs_code_30_*.rs
  test_support/
    Cargo.toml
    src/
      lib.rs
```

Required end-state properties:

- passes `RS-TOPOLOGY`
- passes `RS-TEST`
- family-local README matches reality
- runtime owns orchestration only
- assertions own reusable semantic proof helpers
- test support stays generic
- route/placement wiring is external except for narrow family test entrypoints

## Main Risks

### 1. Repo debt can hide family progress

The family itself is no longer mostly about file movement. The confusing part now is that repo-root `RS-CODE` still reports on many ordinary source files, so repo debt can look like unfinished family work unless the two are kept separate.

### 2. Shared parser bugs were the real trust boundary

[FIXES.md](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/FIXES.md) showed that the worst defects were shared-model problems, not isolated rule bugs:

- fragmented test-context detection
- brittle `// reason:` parsing
- overconfident `cfg_attr` truth
- ambiguous `garde(skip)` exemptions
- public-API reachability false positives
- `OUT_DIR` include carveouts that could still hide traversal

### 3. Assertions layer exists only as a placeholder

This is no longer true as written. The sibling assertions crate is populated and in active use, but result-shape proof is still not uniformly extracted across every rule.

That means the remaining stabilization work is not about creating scaffolding. It is about finishing ownership cleanup where runtime sidecars still carry semantic proof or test-support debt.

## Migration Phases

## Phase 1 — Documentation And Contract Lock

1. Add [code/README.md](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/README.md).
2. Keep this plan file as the current structural source of truth.
3. Do not update rule inventory wording unless the rule contract itself changes.

## Phase 2 — Workspace Split

Status:

- complete

Completed work:

1. Created the split family packages:
   - `crates/runtime/Cargo.toml`
   - `crates/assertions/Cargo.toml`
   - `test_support/Cargo.toml`
2. Updated the main Rust workspace wiring so the family package points at `crates/runtime`.

## Phase 3 — Runtime Move

Status:

- complete

Completed work:

1. Moved current runtime files from `src/` into `crates/runtime/src/`.
2. Kept the current rule-file naming.
3. Kept the current rule-specific `*_tests/` directories under runtime.
4. Restored passing unit coverage after the move.

## Phase 4 — Generic Test Support Extraction

Current state:

- materially complete
- sibling `test_support` is populated and used
- remaining cleanup is about narrowing a few lingering helper boundaries, not first-time extraction

## Phase 5 — Assertions Split

Current state:

- materially complete
- sibling assertions crate exists and is populated
- remaining work is incremental extraction and consistency, not placeholder scaffolding

Target work:

1. Add one assertions module per production rule.
2. Start with the most reusable, result-shape-heavy rules:
   - suppression rules
   - public-surface rules
   - fail-closed rules
3. Move proof-bearing assertion helpers out of runtime sidecars and into `crates/assertions`.
4. Make sidecars prove through owned assertions rather than inline result-shape checking.

## Phase 6 — RS-TEST Closure

Status:

- complete

The family has already been migrated through `RS-TEST` closure. Any new `RS-TEST` findings at this point should be treated as regressions, not expected migration debt.

## Phase 7 — Family Attack Pass

Only after structural stabilization:

1. adversarially attack `RS-CODE` itself
2. compare README, rule inventory, and implementation
3. look for false greens, false positives, and hidden scope widening

That is separate work from the workspace split, and it is already underway. The shared-parser fixes from `FIXES.md` belong to this phase.

## Immediate Execution Order

The next concrete coding order is now:

1. finish the remaining `FIXES.md` tail that is about docs, later-rule coverage, and exact contract wording
2. keep later-numbered rule coverage converging toward exact owned hit sets
3. keep family-correctness commits separate from repo-root `RS-CODE` debt cleanup
4. run repeated adversarial passes after each substantive hardening batch

## Definition Of Done

`RS-CODE` is stabilized when:

- [code/README.md](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/README.md) matches the live implementation
- the family has the package-group shape above
- `guardrail3-app-rs-family-code` points at `crates/runtime`
- family tests pass
- `RS-TOPOLOGY` on the family root is clean
- `RS-TEST` on the family root is clean
- the family is ready for repeated adversarial rule-family review without reopening shared-parser trust gaps
