# Audit: Rust Source Code Scanning (R30-R44, R53, R58)

## CRITICAL: R42 and R53 are DEAD CODE — never called in production

**Severity: CRITICAL**

`structure_checks::check_unsafe()` (R42) and `structure_checks::check_unsafe_code_forbid()` (R53) are defined in `structure_checks.rs` but **NEVER called** from `source_scan::check()` or the orchestrator (`mod.rs`).

Evidence:
- `source_scan.rs` calls `structure_checks::check_file_length` and `structure_checks::check_use_count` but NOT `check_unsafe` or `check_unsafe_code_forbid`.
- `mod.rs` delegates to `source_scan::check()` and never calls structure_checks directly.
- `grep -r "check_unsafe" src/` finds only the function definitions, zero call sites.

**Impact:** R42 (unsafe block/fn detection) and R53 (unsafe_code=forbid in Cargo.toml) are advertised in CLAUDE.md and the check registry but silently produce zero results. A project with `unsafe` blocks or `unsafe_code = "deny"` (bypassable) will get a clean bill of health.

**Fix:** Wire both into `source_scan::check()`:
- `structure_checks::check_unsafe(path, &content, &mut results)` inside the per-file loop
- `structure_checks::check_unsafe_code_forbid(fs, workspace_root, &mut results)` outside the loop (workspace-level)

---

## R30-R31: Crate-level `#![allow]` — AST-based, solid

**Detection:** Uses `syn` to find `#![allow(...)]` inner attributes. Iterates `file.attrs` and checks for `AttrStyle::Inner`. Extracts lint names from the parsed attribute.

**Bypass vectors found:**

1. **NONE for formatting tricks.** Since this is syn-based, whitespace, line breaks, and comments inside the attribute are irrelevant. `#![   allow   (   clippy :: unwrap_used   ) ]` is still parsed correctly.

2. **`cfg_attr` bypass — PARTIALLY COVERED.** `#![cfg_attr(not(test), allow(clippy::unwrap_used))]` at crate level: The `check_cfg_attr_allow` function handles `cfg_attr` containing `allow`, but only reports as R37 Info, not R30 Error. An agent could use `cfg_attr` with an always-true condition to bypass R30's Error severity and get only an Info from R37.

3. **Proc macro bypass — NOT APPLICABLE.** Proc macros generate code at compile time, not inner attributes on the file. `syn` parses the source as-written, so proc-macro-generated allows won't appear in the file being scanned (and shouldn't, since they're generated code).

4. **`#![allow]` inside inline modules** — `file.attrs` only checks file-level (crate-level) attributes. `#![allow(...)]` inside `mod foo { #![allow(clippy::unwrap_used)] }` is an inner attribute on a module, not the file. The `ItemAllowVisitor` (R32-R33) does NOT visit inner attributes on modules — it only visits outer attributes via `collect_outer_allows`. **This means module-level `#![allow(...)]` inside inline modules is UNDETECTED by both R30 and R32.**

---

## R32-R33: Item-level `#[allow]` — comment detection is too permissive

**Detection:** AST-based via `ItemAllowVisitor`. For each `#[allow]`, checks if the SAME LINE contains `//` (any comment at all).

**Bypass vectors found:**

1. **ANY comment counts as "reason" — no `// reason:` required.** The code checks `l.contains("//")` (line 97 of allow_checks.rs). So `#[allow(clippy::unwrap_used)] // lol` is treated as R33 (Info, approved) instead of R32 (Error). The CLAUDE.md says "every `#[allow]` must have a `// reason:` comment" but the code does not enforce the `reason:` prefix.

   The R33 message says "documented reason" but the extracted text is just everything after `//`, so `// TODO fix later` counts as a valid reason.

2. **Block comments `/* reason: */` are NOT detected.** The check only looks for `//`. A `#[allow(clippy::unwrap_used)] /* reason: legitimate */` on the same line will be flagged as R32 Error (no reason) even though there IS a reason in a block comment. This is a false positive, not a bypass, but worth noting.

3. **Multi-line allow attributes.** If `#[allow(clippy::unwrap_used)]` spans multiple lines (e.g., with a long lint list), `raw_lines.get(line_1based.wrapping_sub(1))` gets the line where the attribute STARTS (from syn span). If the comment is on a different line than the span start, it won't be detected.

4. **`#[allow]` on expression statements is covered.** The `ItemAllowVisitor` has `visit_stmt` that catches expression-level allows. Good.

5. **`#[allow]` inside macro invocations is NOT visited.** syn does not expand macros. If a macro generates items with `#[allow]`, those won't be detected. Not really a bypass since the source doesn't contain the allow — but worth noting for completeness.

---

## R34-R35: `#[garde(skip)]` — reasonable but has gaps

**Detection:** AST-based via `GardeSkipTypedVisitor`. Checks field type against `SKIP_OK_TYPES` list. Primitives (bool, numerics) plus BTreeMap/HashMap/BTreeSet/HashSet are allowed to skip.

**Bypass vectors found:**

1. **Type aliases bypass primitive detection.** `type MyBool = bool; struct Foo { #[garde(skip)] x: MyBool }` — syn sees `MyBool`, not `bool`. It won't match `SKIP_OK_TYPES` and will flag it as R34/R35 Error. This is a false positive.

2. **`garde(skip)` on struct-level attributes.** The `GardeSkipTypedVisitor` visits fields, but `GardeSkipVisitor` also checks struct-level `#[garde(skip)]`. The typed visitor (`find_garde_skips_with_types`) is what R34-R35 uses, and it visits fields. Struct-level `#[garde(skip)]` isn't checked by R34-R35 — only by the simpler `find_garde_skips` which isn't used in the allow_checks path. **A `#[garde(skip)]` on the entire struct bypasses per-field type checking.**

3. **Comment detection same issue as R32-R33.** Uses `l.contains("//")` — any comment counts, not just `// reason:`.

4. **Garde is gated behind `garde_enabled` flag.** In `source_scan.rs` line 51: `if garde_enabled { allow_checks::check_garde_skip(...) }`. If garde is not detected/enabled, R34-R35 silently skip. This is by design but worth noting.

---

## R36: EXCEPTION comments in config files

**Detection:** String matching on config file lines for `// EXCEPTION:` or `# EXCEPTION:`.

**Bypass vectors found:**

1. **Only checks 4 files:** `clippy.toml`, `deny.toml`, `Cargo.toml`, `rustfmt.toml`. Missing: `rust-toolchain.toml`, `.guardrail3/overrides/*.toml`, any custom config files referenced in the project.

2. **Case-sensitive.** `// exception:` or `// Exception:` or `// EXCEPTION` (no colon) won't match. An agent could use `// Exception: relaxed this rule` and bypass detection.

3. **Only detects, doesn't enforce.** R36 is Info severity — it's an audit trail, not an error. This is by design per the CLAUDE.md but means exceptions can't block CI.

4. **TOML comments use `#`, not `//`.** The check looks for both `// EXCEPTION:` and `# EXCEPTION:`, which is correct. However, in `clippy.toml`, `deny.toml`, and `Cargo.toml`, only `#` comments are valid TOML. The `//` variant would only appear in the `rustfmt.toml` file if at all. Not a bug per se, but `// EXCEPTION:` in a TOML file would be a syntax error — it's dead code for 3 of 4 files.

---

## R37: `cfg_attr` allow — solid

**Detection:** AST-based. Parses `cfg_attr` token trees to find nested `allow(...)`.

**Bypass vectors:**

1. **Only crate-level + item-level.** The `CfgAttrAllowVisitor` visits items and impl items but not local/stmt/arm attributes. A `#[cfg_attr(test, allow(...))]` on a `let` binding or match arm wouldn't be detected. Low practical impact.

2. **Always Info severity.** Can't distinguish between benign `cfg_attr(test, ...)` and suspicious `cfg_attr(any(), allow(...))` (always-true condition). An agent could use `cfg_attr(all(), allow(clippy::unwrap_used))` to suppress a lint unconditionally while only getting an Info.

---

## R38: File length — edge cases in `filter_non_comment_lines`

**Detection:** Uses `filter_non_comment_lines(content).len()` to count effective lines (excluding comments and blank lines).

**Bypass vectors found:**

1. **String literals containing `/*` or `*/` — HANDLED.** The `strip_string_literals` function is called before comment detection. Strings like `"/* not a comment */"` won't trick the comment filter.

2. **Raw strings — HANDLED.** `strip_string_literals` handles `r"..."`, `r#"..."#`, etc.

3. **Doc comments (`///`, `//!`) — CORRECTLY EXCLUDED.** Lines starting with `///` are skipped by the `starts_with("///")` check. But `starts_with("//")` on line 188 would already catch them. Redundant but correct.

4. **Attribute macros generating code.** Proc macros can generate hundreds of lines not visible in the source. The effective line count only sees the source as-written. This means a file with `#[derive(many_things)]` could expand to 2000 lines but only show 100 in source. Not really a guardrail3 concern since it's a source scan, not a compiled output scan.

5. **Multi-line string literals.** A `r#"... 500 lines of content ..."#` string literal is counted as effective lines since each line inside the string won't start with `//` or be empty. This could inflate the count. Debatable whether this is a bug.

---

## R40-R41: Use count — grouped imports counted correctly

**Detection:** `count_use_statements` counts `syn::Item::Use` items. Uses `file.items.iter().filter(matches Item::Use)`.

**Bypass vectors found:**

1. **Grouped imports count as 1.** `use std::{fs, io, path};` is ONE `Item::Use` in syn. Three imports, one count. This is documented in the R40 message ("Consolidate with `use crate::{a, b, c};`") — it's by design. But it means a file with `use std::{a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u};` (21 imports from one use statement) would count as 1, not 21.

2. **Only top-level use statements.** `use` inside `mod { }` blocks, `fn` bodies, etc. are NOT counted (syn `file.items` only has top-level items). An agent could put imports inside inline modules to reduce the count.

3. **`extern crate` not counted.** `extern crate foo;` is `Item::ExternCrate`, not `Item::Use`. Not counted toward the use limit. Minor — `extern crate` is rare in modern Rust.

---

## R42: Unsafe detection — DEAD CODE (see critical finding above)

**Detection quality (if it were wired in):** The `UnsafeVisitor` covers:
- `unsafe {}` blocks (`visit_expr_unsafe`)
- `unsafe fn` top-level and impl methods (`visit_item_fn`, `visit_impl_item_fn`)
- `unsafe impl` (`visit_item_impl`)
- `unsafe trait` (`visit_item_trait`)

**Missing if it were wired in:**
1. **`unsafe` inside macros.** syn doesn't expand macros. `macro_rules! do_unsafe { () => { unsafe { ... } } }` and then `do_unsafe!()` — the unsafe block is inside the macro body which syn does parse, BUT the macro invocation expanding to unsafe wouldn't be caught. The macro definition itself would be caught (the unsafe block in the macro body).

2. **`unsafe extern` (FFI blocks).** `unsafe extern "C" { fn foo(); }` — the visitor doesn't have `visit_item_foreign_mod`. However, syn represents this as `Item::ForeignMod` with an `unsafety` field. Not visited.

---

## R53: `unsafe_code = "forbid"` — DEAD CODE (see critical finding above)

**Detection quality (if it were wired in):** Parses workspace root `Cargo.toml`, looks at `[workspace.lints.rust].unsafe_code`. Checks for "forbid" (Info/OK) vs "deny" (Error — can be bypassed with `#[allow]`).

**Bypass vectors (if it were wired in):**

1. **Per-crate override.** A member crate could have `[lints.rust] unsafe_code = "allow"` in its own `Cargo.toml`. The check only looks at the workspace root, not member Cargo.tomls. If a crate doesn't inherit workspace lints (no `workspace = true`), it can set its own level. **R29 (workspace lint inheritance) should catch this separately.**

2. **Missing `[workspace.lints.rust]` section.** The `_` catch-all arm (line 145-147) does nothing — "Already covered by R26 lint checks." If R26 is also dead or misconfigured, there's no safety net.

3. **Non-workspace projects.** Single-crate projects without `[workspace]` section: `table.get("workspace")` returns None, so the check silently returns. R53 only works for workspace layouts.

---

## R58: Direct `std::fs` usage — good coverage but has holes

**Detection:** Two-pronged AST scan:
1. `find_std_fs_imports` — detects `use std::fs` import variants
2. `find_inline_std_fs_calls` — detects `std::fs::read_to_string(...)` call expressions

**Bypass vectors found:**

1. **`use std::fs as x` — DETECTED.** `use_subtree_is_fs` handles `UseTree::Rename` where `r.ident == "fs"`. The rename check looks at the ORIGINAL ident, not the alias. So `use std::fs as filesystem;` IS detected.

2. **`use std::fs::*` — PARTIALLY DETECTED.** `use_subtree_is_fs` does NOT match `UseTree::Glob`. The glob arm returns `false`. So `use std::fs::*;` is NOT detected. After the glob import, calling `read_to_string(...)` directly (without `std::fs::` prefix) would not be caught by the inline call detector either, because the path wouldn't start with `std::fs`.

   **This is a real bypass:** `use std::fs::*; read_to_string("foo")` evades both R58 checks.

3. **Re-exports.** If a third-party crate or another module re-exports `std::fs` functions (e.g., `pub use std::fs::read_to_string;` in some utils module), using that re-export won't have `std::fs` in the path. Not detectable by source scan without cross-module analysis.

4. **`use std::fs::File` — DETECTED.** `use_subtree_is_fs` recurses into `UseTree::Path` where `p.ident == "fs"`. So `use std::fs::File;` triggers R58.

5. **`use std::{fs, io}` — DETECTED.** `use_subtree_is_fs` handles `UseTree::Group` and recurses, finding `fs` inside.

6. **`path.ends_with("fs.rs")` skip — exploitable.** Any file named `fs.rs` is skipped (line 137). An agent could create `src/sneaky/fs.rs` that uses `std::fs` directly, and it would be exempt from R58. The intent is to skip the centralized fs module, but the check is too broad.

7. **Inline calls with partial paths after import.** If someone writes `use std::fs; ... fs::read_to_string(...)`, the `InlineStdFsVisitor` only catches paths with 3+ segments starting with `std::fs`. The `fs::read_to_string` call only has 2 segments (`fs`, `read_to_string`), so `path_is_std_fs_call` returns false. **This is the exact clippy hole R58 is supposed to fill, and it has the SAME hole.**

   Wait — re-reading `find_std_fs_imports`: it DOES catch `use std::fs;` as an import (returns the line). So the import itself is flagged as R58 Error. The agent would have to remove the import to fix R58, which means `fs::read_to_string` would then fail to compile. So the import-level detection covers this case. The inline call detector is belt-and-suspenders for `std::fs::read_to_string(...)` without an import.

---

## Summary of findings by severity

### CRITICAL (blocks production correctness)

| ID | Finding |
|----|---------|
| R42 | **DEAD CODE** — `check_unsafe` is never called. Unsafe blocks/fns undetected. |
| R53 | **DEAD CODE** — `check_unsafe_code_forbid` is never called. Workspace lint level unchecked. |

### HIGH (bypass vectors that defeat the check's purpose)

| ID | Finding |
|----|---------|
| R32-R33 | Comment detection accepts ANY `//` comment, not `// reason:` — trivially bypassed with `// lol`. |
| R30/R37 | `cfg_attr(all(), allow(...))` gets Info (R37) instead of Error (R30) — unconditional suppression disguised as conditional. |
| R58 | `use std::fs::*;` (glob import) is NOT detected — direct bypass. |
| R30 | Module-level `#![allow(...)]` inside inline `mod foo { #![allow(...)] }` is undetected by both R30 and R32. |

### MEDIUM (edge cases or design gaps)

| ID | Finding |
|----|---------|
| R58 | `path.ends_with("fs.rs")` skip is too broad — any file named `fs.rs` is exempt. |
| R36 | Case-sensitive `EXCEPTION:` — `exception:` or `Exception:` not detected. |
| R36 | Only checks 4 config files — misses `rust-toolchain.toml`, `.guardrail3/overrides/*.toml`. |
| R40-R41 | Grouped imports `use {a, b, c}` count as 1, not 3. By design, but exploitable. |
| R40-R41 | Only counts top-level use statements — imports inside inline modules not counted. |
| R34-R35 | Struct-level `#[garde(skip)]` bypasses per-field type checking in the typed visitor. |
| R53 | Per-crate `Cargo.toml` can override workspace `unsafe_code` level if lint inheritance is missing. |
| R53 | Non-workspace (single-crate) projects silently skip the check. |
| R42 | `unsafe extern "C" { }` (FFI blocks) not visited by `UnsafeVisitor`. |

### LOW (theoretical or minor)

| ID | Finding |
|----|---------|
| R32-R33 | Block comments `/* reason: */` not recognized as justification. |
| R32-R33 | Multi-line `#[allow(...)]` — comment must be on span-start line. |
| R37 | `cfg_attr` on `let` bindings and match arms not detected. |
| R38 | Multi-line string literals inflating effective line count. |
| R34-R35 | Type aliases (e.g., `type MyBool = bool`) cause false positives. |
