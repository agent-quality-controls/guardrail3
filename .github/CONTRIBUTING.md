# Contributing to guardrail3

Thanks for your interest. guardrail3 enforces strict, deterministic guardrails on Rust and TypeScript codebases - banned APIs, architectural topology, input validation, centralized I/O. Contributions that make the rules sharper, the failure messages clearer, or the coverage wider are welcome.

## Dev setup

Requires the Rust toolchain pinned in `rust-toolchain.toml`. Rustup will install it automatically on first build.

```bash
git clone https://github.com/agent-quality-controls/guardrail3.git
cd guardrail3
cargo build --workspace
```

## Run

```bash
cargo run -- --help
```

## Checks

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

guardrail3 also enforces its rules on itself; the full self-validation script is in `scripts/`. PRs are expected to pass it.

## Adding a check

1. Identify the right module under `packages/` or `apps/` for the check's domain (lint, topology, validation, IO, release).
2. Define the check input and output types in the appropriate `types` crate.
3. Implement the check in the corresponding `logic` crate. Keep IO out of `logic`.
4. Wire the check into the runner with a stable check ID (e.g. `R-IO-08`).
5. Add fixtures and an approval suite (this repo uses fixture3 instead of unit tests for behavior-heavy paths).
6. Document the check in the rules guide.
7. Run the self-validation script.

## Design principles

- **Deterministic.** No LLMs, no nondeterminism, no environment-dependent results.
- **Banned by default.** What the tool permits is explicit. Everything else is a finding.
- **Total visibility.** Every suppression, every allow, every exception is reported. Nothing is hidden.
- **Self-validating.** guardrail3 enforces its rules on itself. If it cannot pass its own validation, it cannot ship.

## Pull request expectations

- One logical change per PR.
- Tests, fixtures, formatting, clippy, and self-validation passing.
- New checks have stable IDs and documentation.
- No new lint suppressions without a documented reason.
