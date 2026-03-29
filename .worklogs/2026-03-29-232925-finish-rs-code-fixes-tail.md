# Finish RS-CODE Fixes Tail

**Date:** 2026-03-29 23:29
**Scope:** `apps/guardrail3/crates/app/rs/families/code/**`, `apps/guardrail3/crates/{adapters/inbound/cli,app/core,app/hooks,app/rs,bin/guardrail3,domain/config,domain/project-tree}`, `apps/guardrail3/crates/app/rs/families/{deny,garde,hexarch,hooks-rs,hooks-shared,release,test,toolchain,fmt}/**`, `.plans/todo/checks/rs/code*.md`, `.plans/todo/check_review/test_hardening/02-code.md`

## Summary
Closed the remaining `FIXES.md` tail for `RS-CODE`: aligned the live contract for import counting and file-length wording, added the missing later-rule regressions, fixed the parser-sidecar self-hosting regressions, updated stale `RS-CODE` docs, and ran a fresh adversarial pass against the family. In parallel, I kept pushing down repo-root `RS-CODE` debt by replacing weak test `expect(...)` messages, trimming filesystem-boundary violations in test support, and reducing import-count noise in obvious hotspots.

## Context & Problem
The earlier `RS-CODE` slice had already landed the critical shared-parser fixes from `apps/guardrail3/crates/app/rs/families/code/FIXES.md`, but the tail of the audit still needed closure:

- item 9: make the import-count rule and its wording/tests match
- item 12: decide whether alias-then-glob belongs to `RS-CODE-21` or is only accidental coverage
- item 13: align `RS-CODE-09` wording with the actual “effective code-bearing lines” metric
- items 14/15: strengthen later-numbered proof coverage where practical
- item 16: remove doc drift that still described the family as if it were mid-migration

At the same time, repo-root `RS-CODE` still had large ordinary-project debt in weak `expect(...)` messages, direct `std::fs` usage in helper crates, and import-count hotspots. Those are not family-correctness bugs, but they were the largest live `RS-CODE` buckets and were safe to keep reducing in the same lane.

## Decisions Made

### Separate family correctness from repo-root debt
- **Chose:** Treat `FIXES.md` completion and family self-hosting as the primary target, while opportunistically reducing live repo-root `RS-CODE` debt in touched files.
- **Why:** The user pointed directly at `FIXES.md`, and those items are about trust in the family implementation. Repo-root debt matters, but it should not be confused with whether the family itself is sound.
- **Alternatives considered:**
  - Drive repo-root `RS-CODE` to zero first — rejected because that would mix ordinary cleanup with unresolved family-contract work.
  - Stop after the shared-parser commit — rejected because the audit tail still had real contract/doc/test gaps.

### Keep `RS-CODE-21` ownership explicit instead of relying on `RS-CODE-15`
- **Chose:** Add a direct regression for `use std as s; use s::fs::*;` in `RS-CODE-21` and update rule docs to state that alias forms are owned there.
- **Why:** The implementation already handled alias-then-glob; the missing piece was proof and wording.
- **Alternatives considered:**
  - Document that `RS-CODE-15` subsumes alias forms — rejected because `21` already structurally owns the glob-import surface.

### Rename the `RS-CODE-09` contract to match the implemented metric
- **Chose:** Update messages/tests/docs from `effective lines` to `effective code-bearing lines`.
- **Why:** The implementation intentionally discounts comment-only lines and string-literal payload-only lines. Widening the parser to count those would have changed the rule; the audit asked for contract alignment.
- **Alternatives considered:**
  - Expand the metric toward human-review-load counting — rejected because there was no evidence the current structural metric was wrong, only that the wording was misleading.

### Fix parser-sidecar self-hosting instead of treating helper tests as a special case
- **Chose:** Move `parse/comments.rs` tests into an owned `comments_tests/` sidecar and add matching assertions modules at both the owned import surface and the physical path `crates/assertions/src/parse/comments.rs`.
- **Why:** `RS-TEST` was correct to reject the inline parser tests and then the incomplete sidecar/assertions shape. A parser helper should not get a bespoke exception just because it is not a numbered rule file.
- **Alternatives considered:**
  - Leave inline tests in `comments.rs` — rejected because the family root then fails `RS-TEST-01`.
  - Keep only a root-level assertions helper — rejected because `RS-TEST-03` also requires the physical owned assertions path.

### Prefer direct removal/fixup over more “reason” exemptions
- **Chose:** Continue replacing weak `expect(...)` strings and direct `std::fs` usage with concrete messages/helpers instead of expanding justified-exception inventory.
- **Why:** Most remaining live `RS-CODE` debt was not principled policy exceptions; it was cleanup residue in tests and helper crates.
- **Alternatives considered:**
  - Add more `#[allow]` reasons or checker-side leniency — rejected because the user explicitly wanted actual removal/fixup where possible.

## Architectural Notes
- `RS-CODE` now self-hosts cleanly for:
  - `RS-ARCH --inventory`
  - `RS-TEST --inventory`
  - `RS-CODE --inventory`
- The code family’s “shared parser/model” contract is now backed by both direct parser-side tests and later-rule regressions:
  - exact token-aware `// reason:` parsing
  - unified test-context handling
  - recursive tri-state `cfg_attr` truth
  - reachable public API filtering
  - `#[expect(...)]` ownership
  - explicit alias/glob coverage for `RS-CODE-21`
- Later-numbered proof coverage is still somewhat uneven in file naming (`direct.rs` / `inventory.rs` versus dedicated `bypasses.rs`), but the adversarial pass did not find a surviving correctness bug in the family.

## Information Sources
- Audit backlog:
  - `apps/guardrail3/crates/app/rs/families/code/FIXES.md`
- Rule inventory and hardening docs:
  - `.plans/todo/checks/rs/code.md`
  - `.plans/todo/checks/rs/code-family-stabilization.md`
  - `.plans/todo/check_review/test_hardening/02-code.md`
- Prior worklogs:
  - `.worklogs/2026-03-29-221132-advance-rs-code-path-and-expect-cleanup.md`
  - `.worklogs/2026-03-29-225525-harden-rs-code-shared-parsers.md`
- Verification:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-code --lib`
  - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/code --family arch --inventory --format json`
  - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/code --family test --inventory --format json`
  - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/code --family code --inventory --format json`
- Adversarial review:
  - focused `test-attack`-style pass over `FIXES.md`, `code.md`, README, runtime tests, and live implementation

## Open Questions / Future Considerations
- The adversarial pass did not find a live `CHECK BUG`, but it still noted two proof-shape follow-ups:
  - broaden explicit `#[expect(...)]` regressions beyond the `RS-CODE-20` extern-block lane
  - continue normalizing later-numbered rules toward the earlier `bypasses.rs` naming/shape convention
- Repo-root `RS-CODE` debt still exists outside the family root. The remaining errors/warnings there are mostly ordinary source cleanup, not family-parser correctness.
- There is unrelated dirty `clippy`/`arch` work in the repo; this slice intentionally avoided bundling that lane.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/code/FIXES.md` — the audit backlog this slice finishes
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/comments.rs` — token-aware `// reason:` parsing plus owned parser sidecar wiring
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_20_extern_allow_tests/direct.rs` — explicit `#[expect(...)]` proof for later-numbered rule ownership
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_21_fs_glob_import_tests/direct.rs` — explicit std-alias glob coverage
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_09_file_length.rs` — aligned “effective code-bearing lines” contract
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lib.rs` — cleaned family-root import surface so the family root is `RS-CODE`-clean
- `apps/guardrail3/crates/app/rs/families/code/README.md` — live package-shape and correctness-status summary
- `.plans/todo/checks/rs/code-family-stabilization.md` — current structural/status narrative for the family
- `.plans/todo/check_review/test_hardening/02-code.md` — current hardening-lane view after the parser fixes
- `.worklogs/2026-03-29-225525-harden-rs-code-shared-parsers.md` — prior checkpoint that closed the critical shared-model bugs

## Next Steps / Continuation Plan
1. Continue repo-root `RS-CODE` cleanup as ordinary source debt, not as family-correctness work. The highest-value remaining buckets are still file-size caps, large import surfaces, and warning-heavy justified `#[path]` usage in non-family code.
2. If we re-open the code-family hardening lane, expand explicit `#[expect(...)]` regressions for shared ownership lanes like `RS-CODE-03/04/17`.
3. Normalize later-numbered rule test layouts where they still lean on generic `direct.rs` / `inventory.rs` instead of named bypass vectors.
4. Keep future `RS-CODE` commits narrow and avoid mixing in the active `clippy` and `arch` worktree lanes.
