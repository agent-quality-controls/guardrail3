# Rust Project Shape

## Adopted Rust Unit

An adopted Rust unit is one Rust workspace or package root with:

- `Cargo.toml`
- `guardrail3-rs.toml`

`g3rs validate workspace --path <path>` validates exactly one adopted Rust unit.

## Repo Hook Surface

The repo hook surface is:

- Git config `core.hooksPath=.githooks`
- `.githooks/pre-commit`
- `.githooks/pre-commit.d/g3rs`

Git runs `.githooks/pre-commit`. That hook must run `.githooks/pre-commit.d/g3rs`.

## Allowed Workspace Root Shapes

This section defines allowed workspace root shapes.

- A root Cargo workspace with `Cargo.toml` and `guardrail3-rs.toml`.
- A nested Cargo workspace with `Cargo.toml` and `guardrail3-rs.toml`.
- A single package root with `Cargo.toml` and `guardrail3-rs.toml`.

## Forbidden Root Shapes

This section defines forbidden root shapes.

- A Rust source directory without root `Cargo.toml`.
- A Rust unit without root `guardrail3-rs.toml`.
- A repo root pretending to own every nested Rust unit.
- A nested Rust unit moved upward only to satisfy a root validator.

## Apparch Layer

This section defines the apparch layer contract.

Apparch layer checks own Rust application layer placement.

## Ownership

Topology owns workspace membership, marker pairs, and repo shape. In short: topology owns project structure.

Apparch owns application architecture layers. In short: apparch owns layer placement.

Cargo does not own project shape. Cargo checks package metadata and dependency policy, not where adopted Rust units may live. In short: cargo does not own project shape.
