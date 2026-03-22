# RS-SOURCE — Rust source file checker (29 rules)

**Input:** *.rs files (syn AST parsed)
**Parser:** syn crate (Rust AST)
**Current code:** `source_scan.rs` (orchestrator), `allow_checks.rs`, `structure_checks.rs`, `code_quality_checks.rs`

## Suppression rules (allow_checks.rs)

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-SOURCE-01 | R30 | Error (Info for test files) | Crate-level `#![allow(...)]` — suppresses lint for entire crate. Also flags inline module `#![allow]`. | Implemented |
| RS-SOURCE-02 | R31 | Info | Justified `#![allow(unused_crate_dependencies)]` — universally exempted | Implemented |
| RS-SOURCE-03 | R32 | Error | Item-level `#[allow(...)]` without `// reason:` comment. Also catches always-true `cfg_attr(all(), allow(...))` | Implemented |
| RS-SOURCE-04 | R33 | Info | Item-level `#[allow(...)]` WITH documented reason (audit trail inventory) | Implemented |
| RS-SOURCE-05 | R34 | Error | `#[garde(skip)]` on non-primitive WITHOUT comment | Implemented |
| RS-SOURCE-06 | R35 | Error | `#[garde(skip)]` on non-primitive WITH comment but no `// reason:` | Implemented |
| RS-SOURCE-07 | R36 | Info | EXCEPTION comments in config files (audit trail inventory) | Implemented |
| RS-SOURCE-08 | R37 | Info | `#[cfg_attr(..., allow(...))]` with genuinely conditional predicate (inventory) | Implemented |

## Structure rules (structure_checks.rs)

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-SOURCE-09 | R38 | Error | File >500 effective (non-comment) lines. Test files exempt. | Implemented |
| RS-SOURCE-10 | R40 | Error | >20 use-imports (AST-counted). Test files exempt. | Implemented |
| RS-SOURCE-11 | R41 | Warn | 16-20 use-imports (warning threshold). Test files exempt. | Implemented |
| RS-SOURCE-12 | R53 | Error/Info | unsafe_code lint level in workspace lints (Info if forbid, Error if deny) | Implemented |

## Quality rules (code_quality_checks.rs)

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-SOURCE-13 | R43 | Warn/Info | todo!/unimplemented! macros (Warn). unreachable! in non-test (Info). AST-based. | Implemented |
| RS-SOURCE-14 | R44 | Warn | .unwrap()/.expect() usage. AST-based. | Implemented |
| RS-SOURCE-15 | R58 | Error | Direct `std::fs` import or inline call. Skips src/fs.rs and test files. AST-based. | Implemented |

## New rules from audit

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-SOURCE-16 | Warn | `panic!` macro in non-test code. Detected by AST walker but currently silently dropped (catch-all `_ => {}`). Strictly worse than `todo!` — crashes in production. Clippy has no lint for this. | Planned (bug fix: add match arm) |
| RS-SOURCE-17 | Error | Blanket `#[allow]` on `impl` block covering >3 methods. No legitimate use case — always apply `#[allow]` to individual methods. Invisible blast radius otherwise. | Planned |
| RS-SOURCE-18 | Error | Always-true `cfg_attr` bypass. Currently only detects `all()` with empty args. Must also detect `any(unix, windows)`, `not(nonexistent_target)`. Disguised unconditional allows. | Planned |
| RS-SOURCE-19 | Info | Large struct (>15 fields) or enum (>20 variants). Architectural smell inventory. Not error, just visibility. | Planned |
| RS-SOURCE-20 | Error | `#[allow]` on `extern "C"` blocks. `item_attrs` returns `&[]` for ForeignMod — one-line fix to add `ForeignMod(f) => &f.attrs`. | Planned (code fix) |
| RS-SOURCE-21 | Error | `use std::fs::*` glob import bypass. Glob brings all std::fs functions into scope, bypassing clippy's disallowed_methods. Variant of the clippy hole that RS-SOURCE-15 exists for. | Planned |

## New rules from audit round 2

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-SOURCE-22 | Error | `#[deny]`/`#[forbid]` attributes without `// reason:`. Undocumented lint level overrides — same class as `#[allow]`. `#![deny(warnings)]` is an anti-pattern. Exception: `#![forbid(unsafe_code)]` is Info (strengthens safety). | Planned |
| RS-SOURCE-23 | Error | `include!()` pulls in unscanned code. Direct bypass of all source scanning. Exception: `include!(concat!(env!("OUT_DIR"), ...))` is Info (build-script pattern). Warn for `include_str!()`/`include_bytes!()` with path traversal (`..`). | Planned |
| RS-SOURCE-24 | Error/Warn | `#[path = "..."]` redirects module paths. Error if path contains `..` (escaping directory). Warn for any `#[path]` usage (breaks standard file layout). Require `// reason:` for Warn case. | Planned |
| RS-SOURCE-25 | Warn | `Result<T, String>` or `Result<T, Box<dyn Error>>` in `pub fn` return types. Poor error discipline — forces callers to parse strings. **Library profile only.** | Planned |
| RS-SOURCE-26 | Warn | `pub use foo::*` glob re-export in lib.rs. Unpredictable API surface — any change to inner module changes library API. **Library profile only.** | Planned |
| RS-SOURCE-27 | Error | Facade-only lib.rs: should contain only `mod`, `pub use`, doc comments, type/const definitions. No function bodies, no impl blocks. **Library profile only.** | Planned |
| RS-SOURCE-28 | Warn | `pub mod foo { ... }` with inline body in lib.rs. Public modules should be separate files for organization. | Planned |
| RS-SOURCE-29 | Warn/Error | Trait with >8 methods (Warn) or >12 methods (Error). Nearly unimplementable traits. **Library profile only.** | Planned |
| RS-SOURCE-21 | Error | `use std::fs::*` glob import bypass. Glob brings all std::fs functions into scope, bypassing clippy's disallowed_methods which only matches fully-qualified paths. Variant of the same clippy hole as RS-SOURCE-15. | Planned |

## Relocated checks

| Old ID | What | New location |
|--------|------|-------------|
| R49 | CLAUDE.md exists at project root | RS-DEPS-05 (belongs with tool/project checks, not source scan) |

## Explicitly rejected audit findings

| Finding | Why rejected |
|---------|-------------|
| `dbg!` macro source scan | Division of labor — add `clippy::dbg_macro` to RS-CARGO expected lints instead. |
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
| `mod` without `#[path]` | Deterministic resolution — no bypass without `#[path]`. Subsumed by RS-SOURCE-24. |
| `.clone()` on large types | No type info in syn. Needs MIR-level analysis. |
| `extern "C"` blocks in non-FFI | R53 + RS-SOURCE-20 + Rust 2024 edition cover this. |
| `pub(crate)` discipline | Too complex — requires export graph. Added `unreachable_pub` to RS-CARGO expected lints instead. |
| `std::process::abort` source scan | Added to RS-CLIPPY expected method bans instead (process-control module). |

## Cross-checker action items

| Target plan | What to add | Why |
|-------------|-------------|-----|
| RS-CLIPPY | `std::process::abort` to EXPECTED_METHOD_BANS | No clippy lint for abort(). Worse than exit() — no unwinding. |
| RS-CLIPPY | `std::any::Any` to EXPECTED_TYPE_BANS | `Box<dyn Any>` erases type safety. |
| RS-CARGO | `unreachable_pub` to expected Rust lints | Flags unreachable `pub` items. Library visibility discipline. |
| RS-DENY | `lazy_static` to EXPECTED_BANS | Legacy macro, `LazyLock` is the replacement. |
