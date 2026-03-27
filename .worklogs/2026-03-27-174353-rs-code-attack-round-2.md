# RS-CODE Attack Round 2

**Date:** 2026-03-27 17:43
**Scope:** `apps/guardrail3/crates/app/rs/ast/src/ast_helpers_tests.rs`, `apps/guardrail3/crates/app/rs/ast/src/ast_visitors.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/attrs.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/comments.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/helpers.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/analysis_helpers.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/types.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/visitors.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_05_garde_skip_without_comment.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_05_garde_skip_without_comment_tests/false_positives.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_06_garde_skip_with_comment.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_06_garde_skip_with_comment_tests/bypasses.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_06_garde_skip_with_comment_tests/false_positives.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_08_cfg_attr_allow_inventory_tests/inventory.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_09_file_length_tests/false_positives.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_18_always_true_cfg_attr_bypass_tests/direct.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_22_deny_forbid_without_reason_tests/direct.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_24_path_attr.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_24_path_attr_tests/direct.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_24_path_attr_tests/false_positives.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_26_lib_glob_reexport_tests/direct.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_28_inline_pub_mod_in_lib_tests/false_positives.rs`

## Summary
Continued the adversarial audit of `RS-CODE` and fixed another batch of real detector bugs rather than repo debt. The main fixes were in parser/state handling and missed visitor surfaces: multiline `#[path]` reasoning, grouped glob re-exports, test-only inline public modules, `cfg_attr(..., deny/forbid(...))`, garde-skip block comments, trait-item `cfg_attr(..., allow(...))`, and multiline raw-string handling in effective file-length counting.

## Context & Problem
The first `RS-CODE` attack passes had already fixed test-path classification, `std::fs` alias bypasses, multiline documented allows, and helper-`expect` false positives. The next step was to attack more of the untouched rule surface rather than stopping at the largest repo buckets.

Concrete issues surfaced from direct code reading and adversarial sampling:

- `RS-CODE-24` still recorded `#[path]` on the attribute start line, so multiline documented path attrs false-positiveed.
- `RS-CODE-24` also over-exempted `cfg_attr(..., path = ...)` when the extracted path looked like canonical sidecar wiring.
- `RS-CODE-26` missed grouped glob re-exports like `pub use foo::{Bar, *};`.
- `RS-CODE-28` flagged `#[cfg(test)] pub mod tests { ... }` even though that module is test-only surface, not public library API.
- `RS-CODE-22` completely missed `cfg_attr(..., deny(...))` / `cfg_attr(..., forbid(...))`.
- The `garde(skip)` split between `RS-CODE-05` and `RS-CODE-06` treated same-line block comments as “no comment”.
- `RS-CODE-08` and `RS-CODE-18` missed trait-item `cfg_attr(..., allow(...))`.
- `RS-CODE-09` still had a real false-positive path: multiline raw-string payload lines were being counted as effective code lines.
- The helper file that grew from these fixes tripped `RS-CODE-09` on the family itself, so the parser helpers needed to be split to keep the family self-hosting.

## Decisions Made

### Track `#[path]` on the closing line and distinguish direct `#[path]` from `cfg_attr(..., path = ...)`
- **Chose:** move path-attr line recording to `span_end_line(...)` and add `via_cfg_attr` on `PathAttrInfo`.
- **Why:** the rule contract says same-line `// reason:` belongs on the line where the attribute closes; also, only direct canonical sidecar `#[path]` wiring is exempt, not conditional `cfg_attr` indirection.
- **Alternatives considered:**
  - Keep start-line tracking and force single-line `#[path]` formatting — rejected because the family had already standardized closing-line reason handling for other multiline attrs.
  - Exempt all canonical-looking extracted paths regardless of source attr kind — rejected because it silently widens the hole the rule is supposed to police.

### Make glob re-export detection collect multiple targets from grouped `use` trees
- **Chose:** change `find_pub_use_glob_reexports()` to flatten `UseTree::Group` and emit every glob target.
- **Why:** `pub use foo::{Bar, *};` is semantically the same unstable surface as `pub use foo::*;`. The old single-target walker silently missed it.
- **Alternatives considered:**
  - Ignore grouped forms as “rare syntax” — rejected because grouped `use` is ordinary Rust and the bypass is real.
  - Detect only the first glob in a group — rejected because it complicates future behavior and still leaves odd missing cases.

### Treat test-only inline public modules as test surface, not library API surface
- **Chose:** skip `#[cfg(test)] pub mod ... { ... }` in `RS-CODE-28`.
- **Why:** the rule exists to keep library public API structure clean. A `cfg(test)` public module in `lib.rs` does not ship as public API.
- **Alternatives considered:**
  - Keep flagging it because it is lexically `pub` — rejected because the semantic surface is test-only, and the false-positive is easy to reproduce.

### Close trait-item cfg-attr holes in both the shared AST helper and the code-family always-true visitor
- **Chose:** add trait-item visitation to `CfgAttrAllowVisitor` in the shared AST crate and to `AlwaysTrueCfgAttrVisitor` in the code-family parser.
- **Why:** `cfg_attr(..., allow(...))` on trait items is valid syntax and belongs to the same bypass/inventory surface as functions and impl methods.
- **Alternatives considered:**
  - Patch only `RS-CODE-08` / `18` locally — rejected because the shared AST helper would stay semantically incomplete.
  - Treat trait items as out of scope — rejected because the contract is about AST policy surfaces, not only free functions.

### Re-split garde-skip comment handling around actual comment presence, not `//` only
- **Chose:** add `same_line_has_comment()` and use it in `RS-CODE-05` / `06`.
- **Why:** the rule split is “without comment” vs “with comment but no reason.” A same-line block comment clearly belongs to the latter bucket.
- **Alternatives considered:**
  - Keep `//` as the only recognized comment style — rejected because it misclassifies a real same-line comment form and makes the rule split inconsistent.

### Replace line-local comment stripping with a stateful scanner for effective line count
- **Chose:** rewrite `filter_non_comment_lines()` as a byte-level scanner with persistent state for line comments, nested block comments, normal strings, byte strings, and raw strings.
- **Why:** line-local stripping could not handle multiline raw strings correctly and still had comment-state edge cases. A stateful scan is deterministic and keeps the rule local.
- **Alternatives considered:**
  - Add more line-based patches to the old helper — rejected because the raw-string bug is inherently cross-line state.
  - Use `syn` AST spans to estimate line counts — rejected because `RS-CODE-09` is file-text policy, not AST-node counting.

### Keep the family self-hosting by splitting parser helpers instead of weakening `RS-CODE-09`
- **Chose:** move the expression/meta/error-analysis helpers into a new [analysis_helpers.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/analysis_helpers.rs).
- **Why:** the helper file exceeded the family’s own file-length cap after the attack fixes. The right response is to split the file, not to carve out a self-exemption.
- **Alternatives considered:**
  - Exempt the file or special-case self-hosting — rejected because that would directly undercut `RS-CODE-09`.
  - Leave the file long and fix it later — rejected because the family must stay green under its own rule set.

## Architectural Notes
This round tightened `RS-CODE` without changing its ownership boundary:

- shared AST collection still lives in `apps/guardrail3/crates/app/rs/ast`
- code-family-local parsing still lives under `crates/runtime/src/parse*`
- the family still consumes routed roots and file scope from `FamilyMapper`

What changed is the internal fidelity of the parser surface:

- attribute line numbers now consistently use the closing line where same-line reasons are allowed
- grouped/glob and trait-item visitors are less syntactically brittle
- line counting no longer relies on line-local string stripping
- self-hosting pressure is now strong enough that parser helpers had to be split into structural modules

One open architectural ambiguity remains: `RS-CODE-27` plan wording is broader than the live `RS-CODE-27`/`28` split. The implementation currently allows private inline modules in `lib.rs` and only warns on public ones. That may need either a rule change or a doc clarification in a later pass.

## Information Sources
- `.plans/todo/checks/rs/code.md` — rule contracts for `05`, `06`, `08`, `09`, `18`, `22`, `24`, `26`, `27`, `28`, `30`
- `apps/guardrail3/crates/app/rs/families/code/README.md` — family-local contract and self-hosting expectations
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/helpers.rs` — previous monolithic helper surface
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/comments.rs` — line-counting and same-line comment logic
- `apps/guardrail3/crates/app/rs/ast/src/ast_visitors.rs` and `apps/guardrail3/crates/app/rs/ast/src/ast_helpers_tests.rs` — shared cfg-attr visitor behavior
- prior worklogs:
  - `.worklogs/2026-03-27-161428-rs-code-attack-fixes.md`
  - `.worklogs/2026-03-27-171025-rs-code-fs-attack-fixes.md`
  - `.worklogs/2026-03-27-172300-rs-code-attr-and-expect-attack-fixes.md`

## Open Questions / Future Considerations
- `RS-CODE-27` / `RS-CODE-28` still have a contract ambiguity around private inline modules in `lib.rs`. The code/tests currently allow them; the plan wording for `RS-CODE-27` reads more broadly.
- `RS-CODE-25` still lacks explicit coverage for public impl methods returning weak error types. The detector handles them, but the branch still is not pinned by tests.
- `RS-CODE-30` has some low-risk fail-closed branches that are still more “covered by reasoning” than by direct sidecar regression.
- Repo-wide top buckets (`RS-CODE-04`, `03`, `32`, `24`, `01`) remain large and currently look like real debt rather than detector drift. The next pass should keep sampling those buckets instead of blindly fixing code.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/comments.rs` — stateful effective-line scanner; critical for `RS-CODE-09`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/helpers.rs` — remaining syntax helpers after the split
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/analysis_helpers.rs` — newly extracted expression/meta/error helpers
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/attrs.rs` — attribute collectors, including trait-item and cfg-attr fixes
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/visitors.rs` — grouped glob re-export and inline-public-module detection
- `apps/guardrail3/crates/app/rs/ast/src/ast_visitors.rs` — shared cfg-attr allow visitor, now covering trait items
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_24_path_attr.rs` — direct-vs-cfg-attr canonical path exemption behavior
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_05_garde_skip_without_comment.rs` — comment presence branch for garde skip
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_06_garde_skip_with_comment.rs` — plain-comment-without-reason branch for garde skip
- `.worklogs/2026-03-27-172300-rs-code-attr-and-expect-attack-fixes.md` — immediate prior attack-fix context

## Next Steps / Continuation Plan
1. Decide the intended contract for `RS-CODE-27` / `RS-CODE-28`. Read `.plans/todo/checks/rs/code.md`, `rs_code_27_facade_only_lib.rs`, `rs_code_28_inline_pub_mod_in_lib.rs`, and the corresponding false-positive tests. Either tighten the implementation or clarify the docs so the two rules do not contradict each other.
2. Add direct coverage for the remaining unpinned branches that came up during attack review:
   - `RS-CODE-25` public impl-method weak error returns
   - any reachable `RS-CODE-30` fail-closed branches still not exercised directly
3. Re-run `rs validate . --family code --inventory --format json` and keep sampling the live `RS-CODE-24` and `RS-CODE-32` buckets. The counts are still large; the next pass should continue separating real repo debt from any remaining detector drift before touching repo code.
