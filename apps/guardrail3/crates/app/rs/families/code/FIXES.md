# RS-CODE Fixes

This document captures the implementation fixes identified during the adversarial `RS-CODE` audit.

These are not policy expansions.
Each item below describes a place where the live family is either:

- violating its own written contract
- internally inconsistent across rules
- too brittle to trust as enforcement
- missing obvious regression coverage for behavior it already claims to own

## How To Read This

- `Critical` means the bug weakens trust in the family across multiple rules or allows major silent escapes.
- `High` means the behavior is materially wrong for at least one live rule and should be fixed before more hardening passes.
- `Medium` means the behavior is wrong or too weak, but less central than the shared-model failures.

The main theme from the audit was simple:

- the worst problems live in shared parsers and shared classifiers
- individual rule files then inherit those mistakes
- fixing the shared model is better than patching rule-by-rule symptoms

## Critical Fixes

### 1. Unify test-context detection across the family

- `Severity`: `Critical`
- `Problem`: the family currently uses incompatible notions of “test code.”
  - path heuristics in [`discover.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/discover.rs)
  - `cfg(test)` parsing in [`helpers.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/helpers.rs)
  - AST visitors with partial nested test-awareness in [`visitors.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/visitors.rs) and [`fs_visitors.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/fs_visitors.rs)
- `Observed failures`:
  - false greens for production files such as `src/tests.rs` or `src/test/...`
  - misses for bare `#[test] fn` in non-test paths
  - misses or false positives for `cfg(any(test, ...))`
  - inconsistent behavior across `RS-CODE-13`, `15`, `16`, `21`, and `32`
- `Why this matters`: this is a family-wide classification bug, not a local rule defect.
- `Robust fix`:
  - define a single canonical test-context model
  - include file-level test roots, `#[cfg(test)]`, `#[test]`, and nested test ancestry
  - treat `cfg` predicate analysis consistently instead of path-substring guessing
  - make all rules consume the same normalized test facts
- `Do not do`: add more ad hoc path suffixes or one-off per-rule exceptions

### 2. Replace `same_line_reason()` with token-aware comment parsing

- `Severity`: `Critical`
- `Problem`: the current reason parser in [`comments.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/comments.rs) uses raw `split_once("//")`, lowercases, and trims.
- `Observed failures`:
  - accepts `// REASON:` and `//reason:` even though the rules say exact `// reason:`
  - can misread `//` inside string literals
  - can turn forged source text into false documentation
  - all rules that depend on same-line reasons inherit the same flaw
- `Affected rules`:
  - `RS-CODE-03`
  - `RS-CODE-04`
  - `RS-CODE-06`
  - `RS-CODE-22`
  - `RS-CODE-24`
- `Why this matters`: this is the shared explanation channel for the family’s documented escape hatches.
- `Robust fix`:
  - parse actual trailing comments, not raw string substrings
  - distinguish comments from string literals
  - enforce one exact accepted syntax
  - keep malformed alternatives as failures, not “close enough”
- `Do not do`: keep broadening accepted spellings to match existing brittle tests

### 3. Rebuild `cfg_attr` predicate classification and recursion

- `Severity`: `Critical`
- `Problem`: the “always true” model in [`analysis_helpers.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/analysis_helpers.rs) is both incomplete and overconfident.
- `Observed failures`:
  - unknown cfg names are treated as effectively false
  - valid cfgs like `doc` can produce false positives
  - `any(unix, windows)` is treated as exhaustive even though non-unix non-windows targets exist
  - nested `cfg_attr` is not handled consistently across all attribute kinds
- `Affected rules`:
  - `RS-CODE-08`
  - `RS-CODE-18`
  - `RS-CODE-20`
  - `RS-CODE-22`
  - `RS-CODE-24`
- `Why this matters`: the family’s main suppression/bypass lane is fragmented around incorrect predicate reasoning.
- `Robust fix`:
  - model `known_true`, `known_false`, and `unknown` separately
  - recurse through nested `cfg_attr` everywhere, not only for allow-collection
  - stop treating unknown cfg names as false
  - keep rule ownership on top of one shared normalized cfg-facts layer
- `Do not do`: patch only `RS-CODE-18` while leaving the shared helpers unsound

## High Fixes

### 4. Add `#[expect(...)]` handling anywhere lint-policy override logic is owned

- `Severity`: `High`
- `Problem`: the family sees `allow` but not `expect`, even though both are local lint-policy override surfaces for the purposes of guardrail auditing.
- `Observed failures`:
  - item-level `#[expect(...)]` bypasses `RS-CODE-03/04`
  - extern-block `#[expect(...)]` bypasses `RS-CODE-20`
  - other allow-style ownership checks may be missing the same form
- `Why this matters`: it creates a modern suppression escape hatch across multiple rules.
- `Robust fix`:
  - decide once that `expect` belongs in the same ownership lane as `allow`
  - extend shared collectors rather than patching one rule at a time
  - add direct and family-level regression tests for both `allow` and `expect`

### 5. Clarify and fix `garde(skip)` ownership before relying on its tests

- `Severity`: `High`
- `Problem`: the AST layer in [`ast_visitors.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/ast/src/ast_visitors.rs) treats maps, sets, references, trait objects, `Option<...>`, and `*Map/*Set` aliases as skip-safe.
- `Observed failures`:
  - live behavior is broader than the “non-primitive” rule wording
  - tests currently lock in that broader behavior instead of questioning it
- `Affected rules`:
  - `RS-CODE-05`
  - `RS-CODE-06`
- `Why this matters`: this is a real semantic hole, not just a wording problem.
- `Robust fix`:
  - choose the intended exemption set explicitly
  - make the rule wording and helper semantics match
  - add exact positive and negative tests for each exempted type class
- `Do not do`: preserve the loophole just because the current tests are green

### 6. Fix `RS-CODE-25` to measure reachable public API, not local `pub` tokens

- `Severity`: `High`
- `Problem`: [`attrs.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/attrs.rs) flags `pub fn` in impl blocks without checking whether the enclosing type is publicly reachable.
- `Observed failures`:
  - `pub fn` on a private type is treated as public API
- `Why this matters`: this creates semantic noise in a rule meant to talk about public contracts.
- `Robust fix`:
  - track enclosing type visibility and reachability
  - only report weak error contracts that are actually externally reachable
  - add regression tests for public methods on private types, private modules, and true public API
- `Implemented outcome`:
  - reachable-public-API classification landed in the shared parser
  - broader weak public error-form ownership now fires only through `RS-CODE-33`
  - legacy `RS-CODE-25` stays silent so one weak public error case has one finding path

### 7. Fix `RS-CODE-24` path-escape detection to use path semantics, not substring matching

- `Severity`: `High`
- `Problem`: [`rs_code_24_path_attr.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_24_path_attr.rs) treats any `..` substring as a directory escape.
- `Observed failures`:
  - strings like `generated..rs` are treated as path traversal
  - strings containing `//` can interfere with reason parsing
- `Why this matters`: this produces both false positives and brittle reasoning around the same rule.
- `Robust fix`:
  - parse path segments structurally
  - only treat a real parent-directory segment as escaping
  - keep path text and trailing reason text independent

### 8. Resolve `RS-CODE-08` vs `RS-CODE-20` ownership overlap on foreign-mod `cfg_attr(..., allow(...))`

- `Severity`: `High`
- `Problem`: conditional allows on `extern` blocks can currently double-report under both rules.
- `Why this matters`: overlapping ownership makes diagnostics noisy and hides rule-boundary confusion.
- `Robust fix`:
  - define whether `ForeignMod` cfg-allow belongs exclusively to `20`
  - exclude it cleanly from `08` if so
  - add an explicit regression test for that ownership boundary

### 9. Fix `RS-CODE-10` / `RS-CODE-11` to match the stated counting contract

- `Severity`: `High`
- `Problem`: [`core.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/core.rs) counts top-level `use` items, not imported leaves.
- `Observed failures`:
  - one grouped import with many names counts as one
- `Why this matters`: the rule is trivially bypassable if the intended contract is really about import surface size.
- `Robust fix`:
  - either count `UseTree` leaves recursively
  - or rename the rule and docs to “top-level use statements”
- `Do not do`: leave code and wording mismatched

### 10. Tighten `RS-CODE-32` test-context and quality logic together

- `Severity`: `High`
- `Problem`: `RS-CODE-32` currently misses some real test contexts and has a low quality floor.
- `Observed failures`:
  - misses bare `#[test]` in non-test paths
  - weak multi-word messages can still pass
  - the blocklist is functionally redundant under the current ordering
- `Why this matters`: the rule becomes easy to evade while appearing present.
- `Robust fix`:
  - inherit the shared test-context fix
  - make message-quality checks intentionally ordered and meaningful
  - test real negative examples, not just the shortest trivial strings

## Medium Fixes

### 11. Harden `RS-CODE-23` build-script include handling

- `Severity`: `Medium`
- `Problem`: `OUT_DIR` concat currently short-circuits into benign handling even when the appended path appears to escape upward.
- `Why this matters`: generated-code exceptions should not silently bless suspicious path shapes.
- `Robust fix`:
  - keep the build-script carveout
  - but still inspect the appended path shape before declaring it benign

### 12. Decide whether `RS-CODE-21` should own alias-then-glob or explicitly defer to `RS-CODE-15`

- `Severity`: `Medium`
- `Problem`: the family can still catch some alias-based shapes via `15`, but `21` does not own them directly.
- `Why this matters`: rule boundaries should be explicit rather than accidental.
- `Robust fix`:
  - either teach `21` alias tracking
  - or document that `15` intentionally subsumes those forms
- `Verdict`:
  - `RS-CODE-21` already owns alias-then-glob directly.
  - The live visitor tracks std aliases, and direct tests now pin both `extern crate std as s; use s::fs::*;` and `use std as s; use s::fs::*;`.
  - No fallback ownership from `RS-CODE-15` is needed for that contract.

### 13. Improve `RS-CODE-09` effective-line semantics or rename the contract

- `Severity`: `Medium`
- `Problem`: large raw-string bodies are discounted heavily by the current effective-line counter.
- `Why this matters`: the current metric may understate review burden.
- `Robust fix`:
  - decide whether the rule measures executable structure or human review load
  - then make the counter and wording match that decision
- `Verdict`:
  - The implementation is intentionally structural, not human-review-load based.
  - `RS-CODE-09` counts effective code-bearing lines and ignores comment-only lines plus raw-string payload-only lines.
  - The fix is wording alignment, not parser expansion.

### 14. Strengthen inventory tests to assert content, not just counts

- `Severity`: `Medium`
- `Problem`: several inventory tests assert counts or ranges without checking titles, messages, paths, or ownership.
- `Why this matters`: correct counts can still hide wrong reasons.
- `Robust fix`:
  - where practical, assert exact findings or exact owned hit sets
  - keep range/count-only assertions for genuinely broad inventory sweeps only

### 15. Add bypass-style sidecar coverage for later-numbered rules

- `Severity`: `Medium`
- `Problem`: early escape-hatch rules have dedicated adversarial files, later rules often do not.
- `Why this matters`: the testing pattern is uneven exactly where parser and ownership bugs still exist.
- `Robust fix`:
  - add rule-local bypass vectors for `20+`
  - keep them focused on one exploit class per test file

### 16. Clean up doc drift after correctness fixes land

- `Severity`: `Medium`
- `Problem`: several local docs disagree about family shape, stabilized status, and even which rule IDs are live.
- `Affected docs`:
  - [`code.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/code.md)
  - [`code-family-stabilization.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/code-family-stabilization.md)
  - [`02-code.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/check_review/test_hardening/02-code.md)
  - [`README.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/README.md)
- `Why this matters`: stale docs make future attacks less trustworthy and can preserve dead assumptions in tests.
- `Robust fix`:
  - update docs only after the live behavior is corrected
  - remove references to dead layouts and dead rule IDs

## Fixes I Would Not Overengineer

These still deserve thought, but they should not distract from the shared-model repairs:

- macro-expansion-grade detection of arbitrary wrapper macros around `include!`
- broad one-off heuristic tuning inside individual rule files before shared helpers are fixed
- documentation-only cleanups before correctness changes land

## Recommended Fix Order

1. Unify test-context detection.
2. Replace `same_line_reason()` parsing.
3. Rebuild `cfg_attr` classification and recursion.
4. Add `#[expect(...)]` handling.
5. Resolve `garde(skip)` contract mismatch.
6. Fix `RS-CODE-25` public reachability.
7. Fix `RS-CODE-24` path semantics and `RS-CODE-08/20` overlap.
8. Align `RS-CODE-10/11` counting with the written contract.
9. Tighten `RS-CODE-32`.
10. Expand regression coverage and then update docs.
