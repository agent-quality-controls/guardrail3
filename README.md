# guardrail3

[![license](https://img.shields.io/github/license/agent-quality-controls/guardrail3)](LICENSE)
[![rust](https://img.shields.io/badge/rust-stable-orange)](rust-toolchain.toml)
[![issues](https://img.shields.io/github/issues/agent-quality-controls/guardrail3)](https://github.com/agent-quality-controls/guardrail3/issues)


Composable code guardrails for Rust and TypeScript projects.

## Why This Exists

The future of software is agent-managed codebases. AI agents already write code better than most humans, and they write it fast — producing in days what took years before. But speed creates problems. Agents produce a lot of code, and with that volume comes a lot of bugs. In practice, it takes about 5 iterations of agentic review and fixes to stabilize a large batch of agent-written code. That's 5x more time debugging than writing.

That's not acceptable.

**guardrail3 creates environments where agents can produce large amounts of code that stays stable.** Instead of fixing bugs after the fact, we prevent them from being introduced. The approach is simple: make it as hard as possible to write broken code.

## Philosophy

This is the same idea that large companies have applied to human engineering for decades — strict linting, enforced topology, mandatory code review gates, standardized project structure. The difference is that humans tend to resist rigid systems and work around them. Agents don't. If the system is harsh enough that it genuinely doesn't allow mistakes, agents will comply and write good code reliably.

**guardrail3 enforces:**

- **Least privilege.** Everything is banned by default. Methods, types, crates — if it's not on the allow-list, it's banned. Every exception requires a documented reason.
- **Rust apparch.** Rust workspaces are checked for typed architectural boundaries: `types`, `logic`, `io/inbound`, and `io/outbound`, plus dependency direction, purity, release readiness, and documented escape hatches.
- **Input validation at every boundary.** Every struct that receives external data (`Deserialize`, `Parser`, `Args`, `FromRow`) must also derive `Validate`. No raw input passes into business logic unchecked.
- **Centralized I/O.** All filesystem operations go through one module. All other files are banned from calling `std::fs` directly. This is enforced by both clippy bans AND AST-based source scan.
- **Total visibility.** Every lint suppression, every `#[allow]`, every `#[garde(skip)]`, every config exception is reported. Nothing is hidden. You decide what to act on — the tool hides nothing.
- **Self-validating.** guardrail3 enforces the same rules on itself. If it can't pass its own validation, it has no business validating others.

## Why Rust

Rust was chosen deliberately. Its compiler and type system enforce correctness by design — ownership, borrowing, no null, no data races. Almost everything that frameworks like ArchUnit enforce in Java is already handled by Rust's compiler. guardrail3 adds the remaining layers: topology enforcement, dependency control, code quality gates, and release readiness checks.

The inspiration comes from Java's Spring Boot and ArchUnit ecosystem — which demonstrated how many bugs can be caught at build time and lint time rather than runtime. guardrail3 brings that same rigor to Rust, where the language itself already provides a stronger foundation.

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
