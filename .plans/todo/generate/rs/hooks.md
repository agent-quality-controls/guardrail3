# Rust Generator — `hooks`

## Generated artifacts

At the validation root:
- `.githooks/pre-commit`
- `.githooks/pre-commit.d/10-merge-conflict-markers.sh`
- `.githooks/pre-commit.d/20-gitleaks.sh`
- `.githooks/pre-commit.d/30-file-size.sh`
- `.githooks/pre-commit.d/40-lockfile-integrity.sh`
- `.githooks/pre-commit.d/50-rust.sh`

Generator-owned operational setup:
- `git config core.hooksPath .githooks`

## Ownership mode

- exact-owned for generated hook files
- generator-owned setup side effect for hook-path configuration

## Root selection

`hooks` is a validation-root family.

The generator owns hook artifacts only at the validation root.

It must never generate:
- `hooks/pre-commit`
- sibling or nested fallback hook files
- language-specific hook layouts outside `.githooks/`

The generator standardizes on the preferred `.githooks/` layout.
Checker-side compatibility for legacy `hooks/pre-commit` does not make that legacy path generator-owned.

## Required generator contract

- the generated hook layout is modular, not monolithic
- `.githooks/pre-commit` is a dispatcher over `.githooks/pre-commit.d/*.sh`
- generated hook files satisfy the shared hook structure contract and the Rust hook semantic contract
- `core.hooksPath` points at `.githooks`
- `10-merge-conflict-markers.sh`, `20-gitleaks.sh`, and `30-file-size.sh` are always generated
- `40-lockfile-integrity.sh` is generated only when the validation root owns a lockfile-integrity surface enforced by `HOOK-SHARED`
- the generated Rust hook step covers:
  - `cargo fmt --check`
  - `cargo clippy` with deny-warnings behavior
  - `cargo deny`
  - `cargo machete`
  - `cargo test`
  - `cargo dupes --exclude-tests`
  - `guardrail3 rs validate --staged` or `guardrail3 validate --staged`
- generator-owned hooks are fail-closed and executable

The generator does not own:
- `.guardrail3/overrides/pre-commit.d/*`
- non-Rust hook families

## Checker target

- `.plans/todo/checks/hooks/shared.md`
- `.plans/todo/checks/hooks/rs.md`

The generated result must satisfy:
- `HOOK-SHARED`
- `HOOK-RS`

## Parity contract

1. `generate -> validate`
- generate the full `.githooks/` layout and hook-path setup
- `HOOK-SHARED` and `HOOK-RS` pass

2. `generate twice`
- second generation is byte-identical for all generated hook files

3. negative mutation
- mutating one generated dispatcher, shell-safety, or Rust-step surface produces the exact `HOOK-SHARED-*` or `HOOK-RS-*` finding

4. scope exactness
- generator creates only the preferred `.githooks/` layout and does not create compatibility fallback hook files
