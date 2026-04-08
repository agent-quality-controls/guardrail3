# Migrate More Code AST Rules

**Date:** 2026-04-08 18:38
**Scope:** `packages/rs/code/g3rs-code-ast-checks`, `packages/rs/code/g3rs-code-ast-ingestion`

## Summary
Migrated the next definite single-file `code` AST rules into `g3rs-code-ast-checks`: `RS-CODE-17`, `18`, `20`, `21`, `23`, `32`, `34`, and `36`. Expanded the shared AST parse layer to support those rules, added rule-local tests, extended end-to-end pipeline coverage, and ran an adversarial test pass to harden the new slice.

## Context & Problem
The first `code` AST package only covered `RS-CODE-13`, `15`, `16`, and `30`. The next migration step was to move the rules that are clearly single-file AST rules without dragging in profile resolution, workspace config ownership, or file-tree structure logic.

The user explicitly wanted:
- the definitely single-file slice first
- real tests, not just compile-success
- an attack pass after migration
- a simple explanation later for why the remaining rules are more annoying

`RS-CODE-24` was not part of this slice because the old `code` runtime no longer owns it; it had already been moved out earlier.

## Decisions Made

### Migrate the clear single-file AST slice only
- **Chose:** move `17`, `18`, `20`, `21`, `23`, `32`, `34`, and `36`
- **Why:** these rules only need one parsed source file plus local test-context handling
- **Alternatives considered:**
  - pull in the more comment-heavy or policy-shaped rules too — rejected because that would mix in the “annoying” cases before the clean AST pattern was proven
  - move `RS-CODE-24` as part of this batch — rejected because legacy ownership had already moved away from `code`

### Expand the checks-runtime parse layer instead of pushing semantics into ingestion
- **Chose:** add the missing attribute analysis, cfg-truth analysis, include parsing, expect-call parsing, generic-cap analysis, and string-dispatch analysis into the AST checks runtime
- **Why:** ingestion should still only gather file contents and shape lane inputs; semantic AST mapping belongs in the checks runtime
- **Alternatives considered:**
  - parse and classify more inside ingestion — rejected because that would blur the package boundary and make the AST checks package thin and fake

### Strengthen tests in two layers
- **Chose:** add rule-local direct and false-positive tests, then add pipeline coverage for the new rules through `crawl -> ingest_for_ast_checks -> check`
- **Why:** rule-local tests catch branch behavior; pipeline tests prove the package wiring
- **Alternatives considered:**
  - only add rule-local tests — rejected because package wiring was a required part of this migration
  - only add pipeline tests — rejected because branch-level coverage would stay too weak

### Use the attack pass to widen edge-case coverage
- **Chose:** after the initial green pass, add more old high-risk scenarios for `20`, `21`, `23`, `32`, `34`, and `36`
- **Why:** the first migrated version worked, but some of the old branch coverage was still missing
- **Alternatives considered:**
  - stop after the first green test run — rejected because the user explicitly asked for a test attack and the first pass still had obvious coverage debt

## Architectural Notes
This keeps the intended AST split intact:

- `g3rs-workspace-crawl` stays neutral
- `g3rs-code-ast-ingestion` reads file contents and builds `G3RsCodeAstChecksInput`
- `g3rs-code-ast-checks` parses source, derives rule-local facts, and runs pure rules

The shared parse layer inside `g3rs-code-ast-checks` is now a real family AST support layer instead of a tiny stub. That is the right place for:
- cfg truth classification
- impl/extern attribute analysis
- include macro classification
- test-only `expect(...)` detection
- generic parameter counting
- string dispatch counting

One notable current behavior was locked in at the pipeline level: `use std::fs::*` currently triggers both `RS-CODE-15` and `RS-CODE-21`. The tests now document that instead of pretending it is one finding.

## Information Sources
- `.plans/todo/checks/rs/code.md` — rule ledger and current rule ownership
- `.plans/todo/checks/2026-04-08-ast-checks-package-architecture.md` — AST package contract
- `.worklogs/2026-04-08-171620-code-ast-checks-initial-slice.md` — first extracted code AST rules
- `.worklogs/2026-04-08-174415-build-code-ast-ingestion.md` — first AST ingestion package
- `.worklogs/2026-04-08-180202-harden-code-ast-lane-after-test-attack.md` — previous attack hardening pattern
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/*` — old parse helpers and visitors
- old legacy rule tests under `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/*/tests`

## Open Questions / Future Considerations
- The current pipeline tests for the new rules mostly lock rule IDs by file, not every exact title/message. That is acceptable for now, but it is still weaker than full per-rule exact pipeline assertions.
- `RS-CODE-21` currently overlaps with `RS-CODE-15` on `use std::fs::*`. That is now documented by tests, but the broader overlap policy remains a family design question.
- The remaining unmigrated `code` rules are the more annoying slice: comment/reason coupling, profile-sensitive behavior, config ownership, or file-tree scope.

## Key Files for Context
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/run.rs` — current list of migrated AST rules
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/mod.rs` — exported parse helpers used by migrated rules
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/attrs.rs` — attr/cfg/include analysis added for this batch
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/visitors/mod.rs` — test-expect and generic-cap analysis
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/visitors/string_dispatch.rs` — string-dispatch analysis
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/fs_visitors.rs` — `std::fs` import/glob analysis
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/pipeline.rs` — end-to-end pipeline coverage for the migrated AST slice
- `.plans/todo/checks/rs/code.md` — rule ledger and unmigrated remainder
- `.worklogs/2026-04-08-180202-harden-code-ast-lane-after-test-attack.md` — previous AST lane hardening context

## Next Steps / Continuation Plan
1. Split the remaining `code` rules into buckets before coding:
   - still single-file but comment/reason heavy
   - profile-sensitive
   - not really AST lane (`config` / `file-tree`)
2. Resolve `profile_name` in `g3rs-code-ast-ingestion` before migrating profile-sensitive rules like the library-only API-shape checks.
3. Decide whether `RS-CODE-15` and `RS-CODE-21` should continue to overlap or whether one should explicitly stand down on glob imports.
4. After the next `code` batch is stable, use the same AST support/runtime pattern for the first bounded multi-file family.
