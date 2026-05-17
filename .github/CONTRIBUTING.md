# Contributing to guardrail3

Thanks for your interest. The fastest way to get a change in is usually to open a detailed issue, not a PR.

## How to contribute

### 1. Open a detailed issue (preferred)

When you spot a bug, want a new check, or want to change behavior, open an issue with as much detail as you can:

- What the check should detect or prevent
- Concrete code examples (accepted vs rejected)
- The bug class, security risk, architectural drift, or maintenance pain it prevents
- Suggested check ID and domain (lint, topology, validation, io, release)
- Suggested severity (error or warning)

Detailed issues are easy to one-shot with an agent. The clearer the spec, the faster the turnaround. Most issues get picked up and implemented without needing a PR from you.

The issue forms in `.github/ISSUE_TEMPLATE/` collect these fields by default.

### 2. Send a pull request (also fine)

If you want to implement the change yourself, go ahead. All contributed code must pass the pre-commit hooks before it can be merged.

## Pre-commit hooks

This repo uses **G3RS** (the Rust variant of guardrail3, running on its own source) as a pre-commit gate. G3RS enforces deterministic code quality on every commit: rustfmt formatting, clippy with deny-warnings, dependency allowlists, banned APIs, AST-based source scan (no unwrap, no todo, no raw std::fs, no allow-without-reason), input-validation enforcement, architectural topology, release readiness, and total suppression visibility.

Hooks run automatically on `git commit`. A failing commit is rejected with a list of findings. Fix them and recommit, or open an issue if a hook fires on code that should be allowed.

If a PR cannot get past the hooks, CI will fail too. The pre-commit gate is non-negotiable for merging.

guardrail3 is self-validating: G3RS runs the same rules on guardrail3's own source that it enforces elsewhere. If a rule cannot pass its own checks, it cannot ship.

## Dev setup

Requires the Rust toolchain pinned in `rust-toolchain.toml` (rustup installs it automatically).

```bash
git clone https://github.com/agent-quality-controls/guardrail3.git
cd guardrail3
cargo build --workspace
```

Run the full validation pipeline:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

## Design principles

- **Deterministic.** No LLMs, no nondeterminism, no environment-dependent results.
- **Banned by default.** What the tool permits is explicit. Everything else is a finding.
- **Total visibility.** Every suppression, every allow, every exception is reported.
- **Self-validating.** If guardrail3 cannot pass its own checks, it cannot ship.
