# Code AST Profile Resolution

**Status:** planned

## Goal

Teach `g3rs-code-ast-ingestion` to classify each Rust source file with the
minimum crate/profile context needed by the remaining single-file `code` AST
rules.

This is not general repo legality. It is only the context needed to decide
whether a one-file code rule applies.

## Why this is needed

The remaining unmigrated `code` AST rules are library-sensitive:

- `RS-CODE-25` weak public `Result` placeholder
- `RS-CODE-26` `pub use foo::*` in `lib.rs`
- `RS-CODE-27` facade-only `lib.rs`
- `RS-CODE-29` large public trait
- `RS-CODE-31` public struct with public named fields
- `RS-CODE-33` bad public error forms like `Result<_, String>`

Those rules cannot decide correctly from AST alone. They need to know whether
the current file belongs to a library target and whether it is the library
root.

## Ownership split

### Ingestion owns

- mapping source file -> owning crate target
- deciding whether the owning target is a library target
- deciding whether the file is that target's `lib.rs`
- attaching this context to `G3RsCodeAstChecksInput`

### AST checks runtime owns

- parsing source content once
- deriving small AST facts
- applying rule logic using the already-attached profile context

### Rules do not own

- cargo/workspace discovery
- target resolution
- path-to-crate ownership logic

## Required output shape

Keep `G3RsSourceFile` as the carrier and make `profile_name` real.

For the `code` family, `profile_name` should mean target profile, not vague app
policy.

First concrete values:

- `Some("library")`
- `Some("binary")`
- `None` when the file is not owned by a recognized Rust target

Add one more explicit flag:

```rust
pub struct G3RsSourceFile {
    pub rel_path: String,
    pub content: String,
    pub is_test: bool,
    pub profile_name: Option<String>,
    pub is_library_root: bool,
}
```

Why both:

- `profile_name == Some("library")` tells rules they are in a library target
- `is_library_root` tells `lib.rs` rules they are on the actual library entry
  file

Do not make rules rediscover `src/lib.rs` by string matching alone.

## Classification algorithm

For each selected Rust source file:

1. Parse workspace/package `Cargo.toml` files that describe Rust targets
2. Build owned target entries with:
   - crate/package root
   - target kind
   - target root source file path
3. Match each selected Rust file to the nearest owning target
4. Emit:
   - `profile_name = Some("library")` for `[lib]` target files
   - `profile_name = Some("binary")` for `[[bin]]` target files
   - `is_library_root = true` only for the exact root source file of the
     library target

For v1, only library vs binary is required.

Examples:

- `src/lib.rs` of a package with a lib target
  - `profile_name = Some("library")`
  - `is_library_root = true`

- `src/foo.rs` included under that same lib target
  - `profile_name = Some("library")`
  - `is_library_root = false`

- `src/main.rs` or `src/bin/*.rs`
  - `profile_name = Some("binary")`
  - `is_library_root = false`

## Data source

Use Rust package metadata already available from the workspace crawl and Cargo
manifests.

Do not reintroduce app-family routing or legacy mapper logic.

The ingestion package should read what it needs from:

- workspace crawl output
- workspace/package `Cargo.toml`

It should not depend on legacy app code.

## Failure policy

Profile resolution should not silently guess.

- if source file selection succeeds but target ownership is unknown:
  - keep the file
  - emit `profile_name = None`
  - emit `is_library_root = false`

- if a needed `Cargo.toml` cannot be parsed well enough to classify owned code:
  - ingestion should fail

Reason:

- unknown ownership is acceptable for rules that do not need profile context
- broken manifest parsing for owned Rust code would make profile-sensitive rules
  fail open

## Implementation order

1. Add `is_library_root` to `G3RsSourceFile`
2. Replace the current stub `resolve_profile_name(...)`
3. Add target ownership helpers in `g3rs-code-ast-ingestion`
4. Add ingestion tests for:
   - library root
   - library non-root module
   - binary root
   - package with both lib + bin
   - unowned file stays `None`
5. Only after that, migrate:
   - `RS-CODE-26`
   - `RS-CODE-27`
   - `RS-CODE-29`
   - `RS-CODE-31`
   - `RS-CODE-33`

## Non-goals for this step

- no multi-file AST
- no repo-global legality
- no public API graph analysis
- no file-tree structural checks
- no attempt to classify every exotic Cargo target kind on day one

## Success criteria

- `g3rs-code-ast-ingestion` emits stable library/binary context
- `lib.rs`-specific rules no longer rely on raw path heuristics
- remaining library-sensitive single-file `code` AST rules can migrate without
  adding workspace discovery to the checks runtime
