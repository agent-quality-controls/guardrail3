# Rust Generator Planning

This directory is the target-state generator contract for Rust.

Each family file defines:
- the exact artifact surface the generator owns
- the root taxonomy that decides where those artifacts belong
- whether the family is exact-owned, semantic-patch, or scaffold
- the checker family the generated result must satisfy
- the parity tests that prove generation and checking actually match

Current implementation mapping and gaps do not live in these family specs.
They live in:
- `rs/gap-analysis.md`

## Ownership modes

### Exact-owned
- the generator owns the full bytes of a file
- repeated generation is byte-identical

### Semantic-patch
- the generator owns only specific sections of a user-owned file
- repeated generation preserves non-owned content and stabilizes the owned sections

### Scaffold
- the generator owns creation of directories, starter files, and any required generator-owned wiring that makes the scaffold valid
- repeated generation must not drift or duplicate scaffold state

## Required generator code structure

Rust generator code is split by family the same way Rust checks are split by family.

```text
apps/guardrail3/crates/app/rs/generate/
├── mod.rs
└── rs/
    ├── mod.rs
    ├── fmt/
    ├── toolchain/
    ├── clippy/
    ├── deny/
    ├── cargo/
    ├── release/
    ├── hooks/
    ├── hexarch/
    └── arch-package/
```

Within each family:
- one file per exact-owned artifact
- one file per semantic patch surface
- one file per scaffold artifact or scaffold operation

Examples:
- `fmt/rustfmt_toml.rs`
- `toolchain/rust_toolchain_toml.rs`
- `clippy/clippy_toml.rs`
- `deny/deny_toml.rs`
- `release/release_plz_toml.rs`
- `release/cliff_toml.rs`
- `release/release_workflow.rs`
- `hooks/pre_commit_dispatcher.rs`
- `hooks/rust_checks.rs`

## Parity doctrine

Every generator family must support the same parity proof shape:

1. `generate -> validate`
- generated output passes the target checker family exactly

2. `generate twice`
- a second run is stable
- exact-owned files are byte-identical
- semantic-patch families preserve non-owned content and normalize owned content deterministically

3. negative mutation
- mutating one generated surface produces the exact checker finding that owns that surface

4. scope exactness
- generation happens only at the roots the family owns
- no sibling, nested, or ancestor escape-hatch files are created accidentally
