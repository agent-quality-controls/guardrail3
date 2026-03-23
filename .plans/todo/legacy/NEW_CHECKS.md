Eugene Tartakovsky, [20/03/2026 18:16]
3. No error discipline — anyhow, String errors, panics, or Box<dyn Error> in
  public APIs

Eugene Tartakovsky, [20/03/2026 18:16]
2. No API surface control — everything is pub, consumers reach into internals,
   so you can't refactor without breaking everything

Eugene Tartakovsky, [20/03/2026 18:16]
4. No organizational decomposition — 5 distinct concerns in one module

Eugene Tartakovsky, [20/03/2026 18:16]
Rule: Facade-only lib.rs                                                      
  What it prevents: God files, unclear API surface                           
  How to check: Parse lib.rs — it should contain only mod declarations, pub use 
    re-exports, and doc comments. No function bodies, no impl blocks, no logic.

Eugene Tartakovsky, [20/03/2026 18:16]
Rule: Typed error module                                                   
  What it prevents: String/anyhow errors in public API                          
  How to check: Check that the crate either has an error.rs/error/ module or 
    lists thiserror in deps

Eugene Tartakovsky, [20/03/2026 18:16]
Rule: pub(crate) discipline                                                   
  What it prevents: Everything-is-pub, leaky internals
  How to check: Check that non-lib.rs modules don't have bare pub items (should 
    be pub(crate) unless re-exported through lib.rs)

Eugene Tartakovsky, [20/03/2026 18:17]
Rule: No pub mod with inline code                                             
  What it prevents: Random module exposure
  How to check: If a module is pub mod foo in lib.rs, foo should be a file/dir, 
    not an inline mod foo { ... } block

    ---

## Internal Dependency Graph Constraints

Rule: Cycle detection
  What it prevents: Circular internal module dependencies (always a design problem)
  How to check: Parse `use crate::` and `mod` statements, build a directed graph,
    reject cycles. A module that needs something from a module that needs something
    from it means the boundary is wrong.

Rule: Fan-out limit (5 sibling imports)
  What it prevents: God-modules that orchestrate everything
  How to check: Count distinct `use crate::` imports per module. In a library
    (no I/O, no composition root), there's no legitimate reason for one module to
    touch everything. Cap at 4-5 sibling imports.

Rule: Fan-in concentration warning
  What it prevents: One module doing too much, becoming a refactoring bottleneck
  How to check: If every module imports one specific module, that module is
    overloaded. Warning threshold, not hard error.

## No Library-Inappropriate Behavior

Rule: No stdout/stderr
  What it prevents: Debug prints left in library code (println!, eprintln!, dbg!)
  How to check: AST scan or clippy print_stdout/print_stderr deny. Libraries
    should never write to stdio — that's the caller's job.
    Log crates excluded.

Rule: No process control
  What it prevents: Libraries killing the process (std::process::exit, abort)
  How to check: AST scan for std::process::exit() and std::process::abort().
    A library should never kill the process. Return an error.

Rule: No global mutable state
  What it prevents: Concurrency landmines (static mut, LazyLock<Mutex<...>>,
    OnceLock with mutation)
  How to check: AST scan for `static mut`, LazyLock/OnceLock with interior
    mutability. Read-only statics (const, LazyLock<String>) are fine.

Rule: No todo!/unimplemented!
  What it prevents: Runtime bombs left as placeholders
  How to check: AST scan for todo!() and unimplemented!() macro invocations.

## Public API Discipline

Rule: #[non_exhaustive] on public enums
  What it prevents: Adding a variant being a breaking change
  How to check: Scan for `pub enum` without `#[non_exhaustive]` attribute.
    Warn level — some enums are intentionally exhaustive.

Rule: #[must_use] on Result-returning public functions
  What it prevents: Callers ignoring return values
  How to check: Scan for `pub fn` returning Result or Option without
    `#[must_use]`. Warn level.

Rule: Required Debug derive on public types
  What it prevents: Consumers unable to inspect values in tests or logs
  How to check: Scan for `pub struct` / `pub enum` without Debug derive.

## Dependency Hygiene (Cargo.toml level)

Rule: No wildcard versions
  What it prevents: Unpinned dependency versions
  How to check: Parse Cargo.toml, reject `*` or missing version specs.

Rule: Direct dependency count cap (10)
  What it prevents: Dependency bloat in libraries
  How to check: Count `[dependencies]` entries (excluding workspace path deps).
    Forces authors to consider whether they really need each dependency.

Rule: No dependency types leaked in public API
  What it prevents: Coupling consumers to your dependency choices forever
  How to check: Check that pub fn signatures only reference types from std,
    core, alloc, or the crate itself. If your public API returns
    serde_json::Value or takes chrono::DateTime, you've coupled every consumer
    to your choices. Warn level — sometimes intentional.

## Complexity Limits

Rule: Max function length (60-80 lines)
  What it prevents: God-functions that agents create routinely (200+ lines)
  How to check: AST-based function body line count.

Rule: Max module depth (3 levels)
  What it prevents: Over-engineered nesting (foo::bar::baz::qux::quux)
  How to check: Count directory nesting depth under src/.

Rule: Max flat file count (8) without subdirectories
  What it prevents: Flat dumping of 15+ files with no organization
  How to check: If src/ has >8 .rs files and no subdirectories, require
    organization into modules.

## Priority Assessment

| Priority          | Rule                              | False positive risk |
|-------------------|-----------------------------------|---------------------|
| Enforce always    | Cycle detection                   | Zero                |
| Enforce always    | No stdout/stderr                  | Near-zero           |
| Enforce always    | No process::exit/abort            | Zero in libraries   |
| Enforce always    | No todo!/unimplemented!           | Zero in shipped code|
| Enforce always    | No wildcard versions              | Zero                |
| Enforce always    | Debug derive on public types      | Near-zero           |
| Enforce threshold | Fan-out limit (5)                 | Low                 |
| Enforce threshold | Max function length (80)          | Low                 |
| Enforce threshold | Dependency count cap (10)         | Low                 |
| Warn              | #[non_exhaustive] on pub enums    | Medium              |
| Warn              | #[must_use] on Result fns         | Low                 |
| Warn              | No dep types in public API        | Medium              |

## API Shape Constraints (Round 2)

Rule: No wildcard re-exports (pub use foo::*)
  What it prevents: Unpredictable public API surface — module changes leak
  How to check: AST scan for `pub use ...::*` in lib.rs.
    Force explicit `pub use module::{TypeA, TypeB}`.

Rule: No public fields on structs
  What it prevents: Consumers constructing via field literals — can't add fields without breaking
  How to check: AST scan for `pub` fields on pub structs. Exception: structs with
    `#[non_exhaustive]`. Force constructor patterns (new(), builder()). Warn level.

Rule: Parameter count limit (5-6)
  What it prevents: Monster function signatures
  How to check: AST count of fn params. Use options struct above threshold.

Rule: Boolean parameter limit (1 bool param max)
  What it prevents: `do_thing(true, false, true)` — swapped argument bugs
  How to check: AST count of bool-typed params per function.

Rule: Generic type parameter limit (3-4)
  What it prevents: Over-abstraction (`fn foo<A, B, C, D, E>()`)
  How to check: AST count of generic type params per function/struct/trait.

Rule: Trait method count limit (8-10)
  What it prevents: God-traits that are nearly unimplementable
  How to check: AST count of methods per trait definition.

Rule: No unnecessary owned params in public functions
  What it prevents: Forcing callers to allocate (String vs &str, Vec vs &[T], PathBuf vs &Path)
  How to check: AST scan for pub fn params that take owned types where borrowed equivalents exist.

Rule: missing_docs enforcement for library profile
  What it prevents: Undocumented public API
  How to check: Verify missing_docs lint is enabled in workspace lints for library crates.

Rule: #[must_use] on custom Error type
  What it prevents: Callers silently dropping errors
  How to check: AST scan for pub enum/struct named *Error without #[must_use].

Rule: No string-based dispatch (3+ string literal match arms)
  What it prevents: Stringly-typed logic that should be an enum
  How to check: AST scan for match expressions with 3+ string literal patterns.

## Dependency Tree Constraints (Round 2)

Rule: No duplicate dependency versions
  What it prevents: Version conflicts in resolved dependency tree
  How to check: Parse Cargo.lock for same crate with different versions.

Rule: Transitive dependency depth limit (30-40)
  What it prevents: Library pulling in massive dependency subtrees
  How to check: Parse Cargo.lock, build dep tree, check total transitive count.

---

## Complete Coverage Analysis

| # | Check | Covered by | Lint/Tool | guardrail3 role | Notes |
|---|-------|------------|-----------|-----------------|-------|
| 1 | Facade-only lib.rs | **Custom** | — | Implement (AST) | No existing lint |
| 2 | Typed error module | **Custom** | — | Implement (structural) | No existing lint |
| 3 | pub(crate) discipline | **Custom** | — | Implement (AST) | No lint for "prefer pub(crate) in sub-modules" |
| 4 | No pub mod inline code | **Custom** | — | Implement (AST) | Proposed in clippy #15966, not merged |
| 5 | No wildcard re-exports | **Clippy** | `clippy::wildcard_imports` | Verify config | Requires `warn-on-all-wildcard-imports = true` in clippy.toml |
| 6 | No public fields | **Custom** | — | Implement (AST) | No existing lint |
| 7 | Cycle detection | **cargo-modules** | `cargo modules dependencies --acyclic` | Verify tool installed | External tool, not a lint |
| 8 | Fan-out limit (5) | **Custom** | — | Implement (AST) | No existing tool |
| 9 | Fan-in concentration | **Custom** | — | Implement (AST) | No existing tool |
| 10 | No stdout/stderr/dbg | **Clippy** | `print_stdout`, `print_stderr`, `dbg_macro` | Verify config | Restriction group, must deny explicitly |
| 11 | No process::exit/abort | **Partial** | `clippy::exit` | Verify config + custom for abort | exit covered, abort is not |
| 12 | No global mutable state | **Partial** | rustc `static_mut_refs` (2024 ed.) | Custom for LazyLock<Mutex> | static mut covered by edition; interior mutability not |
| 13 | No todo/unimplemented | **Clippy** | `clippy::todo`, `clippy::unimplemented` | Verify config | Restriction group |
| 14 | #[non_exhaustive] enums | **Clippy** | `clippy::exhaustive_enums` | Verify config | Restriction group |
| 15 | #[must_use] on Result fns | **Partial** | `clippy::must_use_candidate` | Verify config | Broader than needed; noisy but catches it |
| 16 | Debug on public types | **rustc** | `missing_debug_implementations` | Verify config | Allow-by-default; must enable |
| 17 | missing_docs | **rustc** | `missing_docs` | Verify config | Allow-by-default; must enable |
| 18 | #[must_use] on Error type | **Custom** | — | Implement (AST) | No lint for must_use on type defs |
| 19 | No wildcard versions | **cargo-deny** | `wildcards = "deny"` | Verify config | |
| 20 | Dependency count cap (10) | **Custom** | — | Implement (Cargo.toml) | No existing tool |
| 21 | No dep types in public API | **Custom** | — | Implement (AST) | No existing tool |
| 22 | No duplicate dep versions | **cargo-deny** | `multiple-versions = "deny"` | Verify config | |
| 23 | Transitive dep depth limit | **Custom** | — | Implement (Cargo.lock) | No existing tool |
| 24 | Max function length (80) | **Clippy** | `clippy::too_many_lines` | Verify config | Pedantic; set `too-many-lines-threshold` in clippy.toml |
| 25 | Max module depth (3) | **Partial** | `clippy::excessive_nesting` | Custom for filesystem depth | Lint checks code nesting, not directory depth |
| 26 | Max flat file count (8) | **Custom** | — | Implement (structural) | No existing tool |
| 27 | Parameter count (5) | **Clippy** | `clippy::too_many_arguments` | Verify config | Set `too-many-arguments-threshold` in clippy.toml |
| 28 | Bool param limit (1) | **Clippy** | `clippy::fn_params_excessive_bools` | Verify config | Set `max-fn-params-bools` in clippy.toml |
| 29 | Generic param limit (3-4) | **Custom** | — | Implement (AST) | `type_complexity` is different (score, not count) |
| 30 | Trait method count (8-10) | **Custom** | — | Implement (AST) | No existing lint |
| 31 | No unnecessary owned params | **Clippy** | `clippy::needless_pass_by_value` | Verify config | Pedantic |
| 32 | No string-based dispatch | **Custom** | — | Implement (AST) | No existing lint |

## Summary

**Fully covered by existing linters (10):** 5, 10, 13, 14, 17, 19, 22, 24, 27, 28
  → guardrail3 just verifies these lints are configured correctly

**Covered but need config verification (4):** 7, 16, 31, 15
  → guardrail3 verifies tool installed and/or lint enabled with right threshold

**Partially covered (4):** 11, 12, 25, 18
  → guardrail3 verifies existing coverage + implements custom check for gap

**Fully custom — guardrail3 must implement (14):**
  1, 2, 3, 4, 6, 8, 9, 20, 21, 23, 26, 29, 30, 32
    