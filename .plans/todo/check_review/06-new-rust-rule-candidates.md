# New Rust Rule Candidates

Source: `.plans/todo/NEW_CHECKS.md` (Rust-relevant items only)

## High-signal candidates not yet owned

- Typed error discipline for public APIs
  - extend beyond current `RS-CODE-25`
  - cover `anyhow`, `String`, and possibly require a crate-local typed error surface

- `pub(crate)` discipline / API surface minimization
  - detect bare `pub` in non-`lib.rs` modules unless re-exported intentionally

- No public fields on public structs
  - exception for patterns such as `#[non_exhaustive]`

- `Debug` on public types
  - verify or enforce `missing_debug_implementations` for relevant profiles, or add source-level inventory when config is absent

- `#[non_exhaustive]` on public enums
  - likely warn-level for library-oriented crates

- `#[must_use]` on public `Result` / `Option` functions

- `#[must_use]` on custom error types
  - e.g. public `*Error` types

- No dependency types leaked through public API

- No unnecessary owned params in public functions
  - e.g. `String` vs `&str`, `Vec<T>` vs `&[T]`, `PathBuf` vs `&Path`

- Internal module graph constraints
  - cycle detection
  - fan-out limit
  - fan-in concentration warning

- Dependency pressure rules for libraries
  - direct dependency count cap
  - transitive dependency depth pressure

- Structural organization rules
  - max module depth
  - max flat file count without subdirectories

- Generic parameter count cap

- String-based dispatch warning
  - match expressions with many string literal arms where an enum should likely exist

## Classification

### Implement next-wave

These are the candidates that are universal enough to define without project-structure knowledge.

- No public fields on public structs
  - **Family:** `rs/code`
  - **Shape:** source/API rule
  - **Likely severity:** `Warn`
  - **Universal contract:** a `pub struct` should not expose `pub` named fields directly
  - **Allowed exceptions to decide explicitly when planning the rule:**
    - tuple structs / newtypes
    - explicit opt-out comment/annotation path if we want one
  - **Why this qualifies:** purely syntactic, no repo-specific meaning required

- `#[must_use]` on public functions returning `Result` or `Option`
  - **Family:** `rs/code`
  - **Shape:** source/API rule
  - **Likely severity:** `Warn`
  - **Universal contract:** public functions returning `Result<_, _>` or `Option<_>` should have `#[must_use]`
  - **Why this qualifies:** purely syntactic and generally valid across Rust projects

- Narrow typed error discipline for public APIs
  - **Family:** `rs/code`
  - **Shape:** source/API rule
  - **Likely severity:** `Warn` first
  - **Universal contract:** public error-returning APIs should not expose obviously untyped error forms
  - **Start with explicit banned public error forms only:**
    - `String`
    - `&str`
    - `anyhow::Error`
    - `Box<dyn Error>`
  - **Notes:** extend `RS-CODE-25` rather than inventing a disconnected rule family
  - **Why this qualifies:** this narrow banned set is universal enough; broader “typed error discipline” is not

### Defer until architecture decisions are made

These are plausible, but need more product framing before implementation.

- No dependency types leaked through public API
  - **Family:** `rs/code`
  - **Shape:** source/API rule
  - **Likely severity:** `Warn` or `Error` for libraries
  - **Why deferred:** not universal enough without a project/profile-specific definition of which dependency types are acceptable in public APIs

- `pub(crate)` discipline / API surface minimization
  - **Family:** `rs/code`
  - **Shape:** source/API rule
  - **Likely severity:** `Warn`
  - **Why deferred:** too easy to overfire without a clear model for intended public API surfaces and re-export patterns

- `#[must_use]` on custom error types
  - **Family:** `rs/code`
  - **Shape:** source/API rule
  - **Likely severity:** `Warn`
  - **Why deferred:** lower confidence than function-level `#[must_use]`; utility depends on what exact error-type patterns we want

- No unnecessary owned params in public functions
  - **Family:** `rs/code`
  - **Shape:** source/API heuristic rule
  - **Likely severity:** `Warn`
  - **Why deferred:** high false-positive risk without type/context sophistication

- Generic parameter count cap
  - **Family:** `rs/code`
  - **Shape:** source structural rule
  - **Likely severity:** `Warn`
  - **Why deferred:** probably useful, but needs a sane threshold and careful exceptions for legit generic abstractions

- String-based dispatch warning
  - **Family:** `rs/code`
  - **Shape:** source heuristic rule
  - **Likely severity:** `Info` or `Warn`
  - **Why deferred:** useful smell detector, but highly heuristic and easy to annoy with

- Dependency pressure rules for libraries
  - **Family:** `rs/deps`
  - **Shape:** dependency graph / inventory + threshold rules
  - **Likely severity:** `Warn`
  - **Why deferred:** needs policy on thresholds and what counts as legitimate foundation crates

- Structural organization rules
  - **Family:** `rs/code` or `rs/hexarch`
  - **Shape:** structural metrics rules
  - **Likely severity:** `Warn`
  - **Why deferred:** needs a decision on whether these are universal Rust rules or project-style preferences

### Reject for now

These are either too policy-heavy, too heuristic for current value, or already better handled elsewhere.

- Internal module graph constraints
  - cycle detection
  - fan-out limit
  - fan-in concentration warning
  - **Reason:** this is attractive, but it wants a separate architectural model and probably a new family or a major `rs/code` expansion; not a next-wave add-on

## Ownership notes

- Most of the viable next-wave candidates belong to `rs/code`, not a new family.
- `rs/deps` is the right home only for dependency-count / pressure style rules.
- Prefer config/lint ownership over source ownership when a robust upstream lint can carry the policy.

## Recommended order

1. add no-public-fields-on-public-structs
2. add `#[must_use]` on public `Result` / `Option` functions
3. extend `RS-CODE-25` into narrow banned public error forms
4. discuss the deferred policy-heavy candidates separately

## Concrete next planning targets

These are the ones we should turn into real rule specs first:

- `RS-CODE-next`: no public fields on public structs
- `RS-CODE-next`: `#[must_use]` on public `Result` / `Option` functions
- `RS-CODE-25` extension or sibling rule: narrow banned public error forms

## Already substantially covered

- No wildcard versions
  - already substantially covered through deny/cargo policy surface
- No duplicate dependency versions
  - already substantially covered through `cargo-deny` baseline / policy surface
- Facade-only `lib.rs`
  - covered by `RS-CODE-27`
- No public inline module bodies in `lib.rs`
  - covered by `RS-CODE-28`
- No wildcard re-exports
  - covered by `RS-CODE-26`
- Max function length
  - already substantially covered through clippy/config threshold verification
- Parameter count / bool parameter pressure
  - already covered via clippy baseline / config contract
- Trait method count pressure
  - already covered by `RS-CODE-29`
- No stdout/stderr / no process control / no `todo!` / no global mutable state
  - already largely covered by `RS-CODE`, `RS-CLIPPY`, and canonical clippy bans

## Explicitly not part of the current contract

- `missing_docs` enforcement
  - intentionally not active in the current Rust contract
  - do not reintroduce without an explicit product decision
