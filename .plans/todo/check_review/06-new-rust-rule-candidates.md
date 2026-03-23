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
