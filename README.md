# guardrail3

[![license](https://img.shields.io/github/license/agent-quality-controls/guardrail3)](LICENSE)
[![rust](https://img.shields.io/badge/rust-stable-orange)](rust-toolchain.toml)
[![issues](https://img.shields.io/github/issues/agent-quality-controls/guardrail3)](https://github.com/agent-quality-controls/guardrail3/issues)


Composable code guardrails for Rust and TypeScript projects.

For design rationale (why banned-by-default, why self-validating, why Rust), see [Philosophy](https://github.com/agent-quality-controls/guardrail3/wiki/Philosophy). For how guardrail3 compares to ArchUnit, clippy, cargo-deny, dependency-cruiser, and ts-prune, see [Comparison](https://github.com/agent-quality-controls/guardrail3/wiki/Comparison).

## Quick Start

```bash
cargo install guardrail3

# Rust service
guardrail3 rs init --profile service .
guardrail3 rs generate
guardrail3 rs validate .

# TypeScript app
guardrail3 ts init .
guardrail3 ts generate
guardrail3 ts validate .

# Generate comprehensive guide
guardrail3 dump-guide
```

Run `guardrail3 --help` for full documentation including all commands, profiles, topology conventions, config reference, and the complete check inventory.

## What It Checks

**Rust:** Config completeness (clippy, deny, rustfmt, toolchain, workspace lints), AST-based source scan via syn (allows without reason, unsafe, unwrap, todo, std::fs, file length), dependency allowlists, apparch enforcement, release readiness, test quality, garde validation at input boundaries.

All source scanning is 100% AST-based (syn for Rust, tree-sitter for TypeScript). Zero grep. No false positives from strings, comments, or macros.

## Active Layout

The active Rust CLI is:

```
apps/guardrail3-rs/
packages/rs/
packages/parsers/
packages/shared/
```

The old multi-language app is archived under:

```
legacy/apps/guardrail3-current/
legacy/guardrail3.toml
```

## License

MIT
