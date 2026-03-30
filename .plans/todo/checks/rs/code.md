# RS-CODE — Rust code file checker (29 implemented rules + 5 next-wave planned rules)

> Superseded as the primary family plan by [`.plans/by_family/rs/code.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_family/rs/code.md).
> Keep this file as a detailed rule ledger and migration/history reference.

**Input:** *.rs files (syn AST parsed)
**Parser:** syn crate (Rust AST)
**Current code:** `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/` with family orchestration in `lib.rs`, routed discovery in `discover.rs`, normalized family facts in `facts.rs`, typed rule inputs in `inputs.rs`, and shared parsing under `parse/`

## Suppression rules (allow_checks.rs)

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-CODE-01 | R30 | Error (Info for test files) | Crate-level `#![allow(...)]` — suppresses lint for entire crate. Also flags inline module `#![allow]`. | Implemented |
| RS-CODE-02 | R31 | Info | Justified `#![allow(unused_crate_dependencies)]` — universally exempted | Implemented |
| RS-CODE-03 | R32 | Error | Item-level `#[allow(...)]` without `// reason:` comment. | Implemented |
| RS-CODE-04 | R33 | Info | Item-level `#[allow(...)]` WITH documented reason (audit trail inventory) | Implemented |
| RS-CODE-05 | R34 | Error | `#[garde(skip)]` on non-primitive WITHOUT comment | Implemented |
| RS-CODE-06 | R35 | Error | `#[garde(skip)]` on non-primitive WITH comment but no `// reason:` | Implemented |
| RS-CODE-07 | R36 | Info | EXCEPTION comments in config files (audit trail inventory) | Implemented |
| RS-CODE-08 | R37 | Info | `#[cfg_attr(..., allow(...))]` with genuinely conditional predicate (inventory) | Implemented |

## Structure rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-CODE-09 | R38 | Error | File >500 effective code-bearing lines. Comment-only lines and string-literal payload-only lines do not count. Test files exempt. | Implemented |
| RS-CODE-10 | R40 | Error | >20 use-imports (AST-counted). Test files exempt. | Implemented |
| RS-CODE-11 | R41 | Warn | 16-20 use-imports (warning threshold). Test files exempt. | Implemented |
| RS-CODE-12 | R53 | Error/Info | unsafe_code lint level in workspace lints (Info if forbid, Error if deny) | Implemented |

## Quality rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-CODE-13 | R43 | Warn/Info | todo!/unimplemented! macros (Warn). unreachable! in non-test (Info). AST-based. | Implemented |
| RS-CODE-15 | R58 | Error | Direct `std::fs` import or inline call. Skips test files and explicit filesystem-boundary modules (`src/fs.rs`, `src/fs/mod.rs`, or crate-root `fs/src/lib.rs`). AST-based. | Implemented |

## New rules from audit

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-CODE-16 | Warn | `panic!` macro in non-test code. Detected by AST walker but currently silently dropped (catch-all `_ => {}`). Strictly worse than `todo!` — crashes in production. Clippy has no lint for this. | Implemented |
| RS-CODE-17 | Error | Blanket `#[allow]` on `impl` block covering >3 methods. No legitimate use case — always apply `#[allow]` to individual methods. Invisible blast radius otherwise. | Implemented |
| RS-CODE-18 | Error | Always-true `cfg_attr` bypass. Currently only detects `all()` with empty args. Must also detect `any(unix, windows)`, `not(nonexistent_target)`. Disguised unconditional allows. | Implemented |
| RS-CODE-19 | Info | Large struct (>15 fields) or enum (>20 variants). Architectural smell inventory. Not error, just visibility. | Implemented |
| RS-CODE-20 | Error | `#[allow]` on `extern "C"` blocks. `item_attrs` returns `&[]` for ForeignMod — one-line fix to add `ForeignMod(f) => &f.attrs`. | Implemented |
| RS-CODE-21 | Error | `use std::fs::*` glob import bypass, including std-alias forms such as `use std as s; use s::fs::*;`. Variant of the filesystem hole that `RS-CODE-15` covers for direct imports/calls. | Implemented |

## New rules from audit round 2

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-CODE-22 | Error | `#[deny]`/`#[forbid]` attributes without `// reason:`. Undocumented lint level overrides — same class as `#[allow]`. `#![deny(warnings)]` is an anti-pattern. Exception: `#![forbid(unsafe_code)]` is Info (strengthens safety). | Implemented |
| RS-CODE-23 | Error | `include!()` pulls in unscanned code. Direct bypass of all code scanning. Exception: `include!(concat!(env!("OUT_DIR"), ...))` is Info (build-script pattern). Warn for `include_str!()`/`include_bytes!()` with path traversal (`..`). | Implemented |
| RS-CODE-24 | Error/Warn | `#[path = "..."]` redirects module paths. Error if path contains `..` (escaping directory). Warn for any `#[path]` usage (breaks standard file layout). Require `// reason:` for Warn case. Canonical `#[cfg(test)]` sidecar wiring to `<rule>_tests/mod.rs` is exempt. | Implemented |
| RS-CODE-25 | Warn | `Result<T, String>` or `Result<T, Box<dyn Error>>` in `pub fn` return types. Poor error discipline — forces callers to parse strings. **Library profile only.** | Implemented |
| RS-CODE-26 | Warn | `pub use foo::*` glob re-export in lib.rs. Unpredictable API surface — any change to inner module changes library API. **Library profile only.** | Implemented |
| RS-CODE-27 | Error | Facade-only lib.rs: should contain only `mod`, `pub use`, doc comments, type/const definitions. No inline module bodies, no function bodies, no impl blocks. **Library profile only.** | Implemented |
| RS-CODE-29 | Warn/Error | Trait with >8 methods (Warn) or >12 methods (Error). Nearly unimplementable traits. **Library profile only.** | Implemented |
| RS-CODE-30 | Error | Source/config input failures that would otherwise fail the family open: unreadable Rust source, unparsable Rust source, or unparsable code-family policy inputs (`Cargo.toml`, `guardrail3.toml`). | Implemented |
| RS-CODE-32 | Error | Test-only `expect(...)` messages must be useful string literals. Non-literal messages or trivial literals like `"ok"` are forbidden. Production `expect(...)` ownership belongs to Clippy/Cargo policy instead. | Implemented |

`RS-CODE-28` was merged into `RS-CODE-27` once `lib.rs` facade policy was tightened to reject all inline module bodies, not just public ones.

## Recently closed universal rules

These rules are now implemented in the live family. Keep the sections below as the detailed contract/history reference.

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-CODE-31 | Error | `pub struct` with named `pub` fields. Public structs should not expose field bags as their default API shape. | Implemented |
| RS-CODE-33 | Error | Public function returning obviously untyped public error forms: `Result<_, String>`, `Result<_, &str>`, `Result<_, anyhow::Error>`, or `Result<_, Box<dyn Error>>`. | Implemented |
| RS-CODE-34 | Error | More than 6 type/const generic parameters on a `struct`, `enum`, `trait`, or `fn`. Lifetimes do not count. | Implemented |
| RS-CODE-35 | Error | Per-crate source tree exceeds structural caps: module depth >6, sibling subdirectories >12, or sibling `.rs` files >20 in one Rust source directory. | Implemented |
| RS-CODE-36 | Error | One string-dispatch site has more than 10 string-literal branches. Applies to `match` and `if/else if` chains on the same expression. Test files exempt. | Implemented |

### RS-CODE-31 — Public fields on public structs

**Intent**
- preserve encapsulation in universally understandable Rust terms
- avoid turning public types into uncontrolled field bags

**Trigger surface**
- `pub struct Name { ... }`
- one or more named fields with `pub`

**Initial exclusions**
- tuple structs / newtypes
- private structs
- named fields without `pub`

**Severity**
- `Error`

**Open policy point**
- whether tuple structs / newtypes remain the only built-in exemption in v1

**Examples**

Should error:

```rust
pub struct User {
    pub id: String,
    pub email: String,
}
```

Should not error:

```rust
pub struct User {
    id: String,
    email: String,
}
```

```rust
pub struct UserId(pub String);
```

### RS-CODE-33 — Narrow banned public error forms

**Intent**
- prevent obviously bad public error contracts without requiring project-specific “ideal” error design

**Trigger surface**
- public functions returning:
  - `Result<_, String>`
  - `Result<_, &str>`
  - `Result<_, anyhow::Error>`
  - `Result<_, Box<dyn Error>>`

**Initial exclusions**
- non-public functions
- non-`Result` returns
- internal/private helpers

**Severity**
- `Error`

**Relationship to existing rule**
- likely replaces or broadens `RS-CODE-25`
- implementation should avoid leaving overlapping partially-duplicated public-error rules

**Examples**

Should error:

```rust
pub fn parse(input: &str) -> Result<Value, String> {
    // ...
}
```

```rust
pub fn parse(input: &str) -> Result<Value, anyhow::Error> {
    // ...
}
```

Should not error:

```rust
pub fn parse(input: &str) -> Result<Value, ParseError> {
    // ...
}
```

### RS-CODE-34 — Generic parameter count cap

**Intent**
- catch abstraction sludge and agentic over-generalization

**Trigger surface**
- more than 6 generic parameters on:
  - `struct`
  - `enum`
  - `trait`
  - `fn`

**Count**
- type parameters
- const parameters

**Do not count**
- lifetimes

**Initial exclusions**
- `impl` blocks directly

**Severity**
- `Error`

**Examples**

Should error:

```rust
pub fn build<A, B, C, D, E, F, G>(...) -> Output {
    // ...
}
```

```rust
pub struct Cache<A, B, C, D, E, F, const N: usize> {
    // ...
}
```

Should not error:

```rust
pub fn build<A, B, C, D, E, F>(...) -> Output {
    // ...
}
```

### RS-CODE-35 — Per-crate structural organization caps

**Intent**
- catch source-tree sprawl and uncontrolled agentic expansion

**Measured from**
- the crate root (`Cargo.toml`) downward
- within Rust source/module directories only

**Caps**
- module depth `> 6`
- sibling subdirectories in one Rust source directory `> 12`
- sibling `.rs` files in one Rust source directory `> 20`

**Severity**
- `Error`

**Examples**

Should error:
- a crate source path deeper than 6 nested module segments from the crate root
- a module directory with 13 sibling subdirectories
- a module directory with 21 sibling `.rs` files

### RS-CODE-36 — String dispatch cap

**Intent**
- catch large stringly typed switch blobs that should become typed models

**Trigger surface**
- more than 10 string-literal branches at one dispatch site:
  - a `match` on one string-like expression
  - an `if` / `else if` chain comparing the same expression to string literals

**Do not count**
- wildcard/default arms
- non-string-literal guards

**Initial exclusion**
- test code

**Severity**
- `Error`

**Examples**

Should error:
- one `match kind { ... }` with 11 string literal arms
- one `if action == "..." { } else if action == "..." { } ...` chain with 11 string literal comparisons on the same expression

## Relocated checks

| Old ID | What | New location |
|--------|------|-------------|
| R49 | CLAUDE.md exists at project root | RS-DEPS-05 (belongs with tool/project checks, not code scan) |

## Explicitly rejected audit findings

| Finding | Why rejected |
|---------|-------------|
| `dbg!` macro source scan | Division of labor — add `clippy::dbg_macro` to RS-CARGO expected lints instead. |
| `.unwrap()` / `.expect()` source scan | Division of labor — enforce `clippy::unwrap_used` and `clippy::expect_used` in RS-CARGO/RS-CLIPPY instead of duplicating Clippy in AST scanning. |
| `unsafe` inside `macro_rules!` | R53 `unsafe_code = "forbid"` catches at compile time. Known limitation, documented. |
| `.unwrap_or_default()` | Context-dependent, too many false positives. Code review territory. |
| Deep nesting (>4 levels) | Fragile from AST, file length limit covers symptoms. |
| Cognitive complexity | Clippy nursery lint — verify via RS-CARGO expected lints when stable. |
| `String` vs `&str` in library API | Too many valid `String` parameters, too many false positives. |
| Derive macro output `#[allow]` | Not exploitable — syn parses pre-expansion source. Supply chain risk covered by cargo-deny. |
| `Box<dyn Any>` source scan | Added to RS-CLIPPY-05 disallowed-types instead (cleaner: clippy catches at compile time with reason message). |
| `#[no_mangle]` in non-FFI | Triple-covered: clippy::no_mangle_with_rust_abi + Rust 2024 requires unsafe + unsafe_code=forbid. |
| `std::mem::transmute` | unsafe_code=forbid blocks it. |
| `lazy_static!` macro | Handle via RS-DENY crate bans (added to expected bans). |
| `as` casts | `as_conversions` lint in RS-CARGO expected lints (verified: line 87). |
| `mod` without `#[path]` | Deterministic resolution — no bypass without `#[path]`. Subsumed by RS-CODE-24. |
| `.clone()` on large types | No type info in syn. Needs MIR-level analysis. |
| `extern "C"` blocks in non-FFI | R53 + RS-CODE-20 + Rust 2024 edition cover this. |
| `pub(crate)` discipline | Too complex — requires export graph. Added `unreachable_pub` to RS-CARGO expected lints instead. |
| `std::process::abort` source scan | Added to RS-CLIPPY expected method bans instead (process-control module). |

## Cross-checker action items

| Target plan | What to add | Why |
|-------------|-------------|-----|
| RS-CLIPPY | `std::process::abort` to EXPECTED_METHOD_BANS | No clippy lint for abort(). Worse than exit() — no unwinding. |
| RS-CLIPPY | `std::any::Any` to EXPECTED_TYPE_BANS | `Box<dyn Any>` erases type safety. |
| RS-CARGO | `unreachable_pub` to expected Rust lints | Flags unreachable `pub` items. Library visibility discipline. |
| RS-DENY | `lazy_static` to EXPECTED_BANS | Legacy macro, `LazyLock` is the replacement. |

## Legacy carry-forward from archived parsing migration

The old top-level `migrate_to_ast_parsing.md` is being archived, but a few Rust-only hardening items remain live here:

- keep TypeScript parsing migration out of scope; only Rust residuals matter now
- `RS-CODE-03..06` still rely partly on source-line / comment heuristics to associate `// reason:` comments with attributes and `#[garde(skip)]`
- `RS-CODE-07` exception inventory is intentionally raw-line/config-text driven, but should stay isolated to explicit exception-comment auditing rather than spread back into semantic rule logic
- any remaining source rules that still depend on raw token strings instead of AST shape should be treated as hardening debt, not “done forever”

The active target is:

- semantic Rust source rules should prefer `syn` structure
- raw-line/text matching should be confined to comment/reason surfaces where ASTs do not preserve the needed information

## Target family shape

This family should be implemented as `rs/code`, not `rs/source`.

Target folder:

```text
apps/guardrail3/crates/app/rs/checks/rs/code/
├── mod.rs
├── facts.rs
├── inputs.rs
├── discover.rs
├── parse.rs
├── rs_code_01_*.rs
├── ...
├── rs_code_29_*.rs
└── rs_code_30_*.rs
```

### Family responsibilities

The `code` family is the main streamed-AST exception in the checker architecture:
- discovery comes from `ProjectTree`
- file content is streamed on demand by the orchestrator
- AST parsing happens once per file
- rules get small typed inputs, not `ProjectTree` and not raw filesystem access

The family should cover two input classes:
- per-file code checks
- one workspace-level lint-setting check for `unsafe_code`

### Discovery

The orchestrator should discover:
- all `*.rs` files from `ProjectTree.structure`
- whether each file is a test file
- the nearest Rust policy/profile context for each file when profile-gated rules matter
- the workspace/root Cargo lint facts needed for `RS-CODE-12`

### Suggested facts

`facts.rs` should define normalized, family-local facts only:

```rust
pub struct RustCodeFileFacts {
    pub rel_path: String,
    pub is_test: bool,
    pub profile_name: Option<String>,
    pub package_kind: Option<String>,
}

pub struct UnsafeCodeLintFacts {
    pub cargo_rel_path: String,
    pub lint_level: Option<String>,
}
```

Do not cache file content or ASTs in long-lived project-wide structures.

### Suggested inputs

`inputs.rs` should keep atomic surfaces small:

```rust
pub struct RustCodeFileInput<'a> {
    pub rel_path: &'a str,
    pub content: &'a str,
    pub ast: &'a syn::File,
    pub is_test: bool,
    pub profile_name: Option<&'a str>,
}

pub struct UnsafeCodeLintInput<'a> {
    pub cargo_rel_path: &'a str,
    pub lint_level: Option<&'a str>,
}
```

Rules 01-11 and 13, 15-29 should run on one `RustCodeFileInput`.

Rule 12 should run on one `UnsafeCodeLintInput`.

Rule 30 should run on one input-failure surface emitted by the orchestrator when source/config parsing would otherwise be skipped.

### Rule grouping inside the orchestrator

The orchestrator should parse each Rust file once and then fan the same `RustCodeFileInput` through the rule files that apply to that file.

Recommended execution buckets:
- suppression bucket:
  - `RS-CODE-01` through `RS-CODE-08`
- structure bucket:
  - `RS-CODE-09` through `RS-CODE-11`
- quality bucket:
  - `RS-CODE-13`
  - `RS-CODE-15` through `RS-CODE-29`

`RS-CODE-12` should run once from Cargo/workspace lint facts, outside the per-file loop.

`RS-CODE-30` should run from orchestrator-level read/parse failures and must never be skipped.

### Test strategy

Tests should follow the same strict pattern as the finished config families:
- one rule-specific `*_tests/` module directory per rule
- rule-local assertions, not family smoke tests
- direct typed inputs where possible
- orchestrator tests only for:
  - file discovery
  - test-file classification
  - profile/root resolution
  - one-parse-per-file fan-out

### Implementation order inside the family

Recommended order:

1. `discover.rs`
   - enumerate Rust files
   - classify test files
   - resolve per-file profile context
2. `parse.rs`
   - parse one file into `syn::File`
   - expose comment/raw-line helpers needed by allow/reason checks
3. `facts.rs` and `inputs.rs`
4. migrate the simplest file-local rules first:
   - `RS-CODE-09`
   - `RS-CODE-10`
   - `RS-CODE-11`
   - `RS-CODE-13`
5. add `RS-CODE-12` separately from Cargo lint facts
6. then migrate the attribute-heavy rules:
   - `RS-CODE-01` through `RS-CODE-08`
   - `RS-CODE-17` through `RS-CODE-24`
7. finish the profile-gated library rules:
   - `RS-CODE-25` through `RS-CODE-29`
