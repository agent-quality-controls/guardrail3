# cargo test

## What it does
Runs Rust tests.

## Config file
None (profiles in Cargo.toml).

## How to invoke
```bash
cd <workspace-root> && cargo test --workspace
```
Per workspace. `--workspace` runs tests for all member crates.

## Guardrail3's role
- **Hook:** Run per discovered workspace
