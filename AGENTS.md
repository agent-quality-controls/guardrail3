# guardrail3 — Project Handoff

This file is the current source of truth for agent work in this repo.

## Current Direction

guardrail3 is currently focused on **Rust guardrails only**.

TypeScript and deployment work is not the active direction right now:
- do not expand TypeScript rule inventory
- do not spend implementation time on TS/hook/deploy families unless explicitly asked
- treat existing TS code and docs as legacy background, not the current roadmap

The active work is:
- clean Rust rule inventory
- new checker architecture based on `ProjectTree`
- family-by-family migration from old `rs/legacy/validate/*` code into new per-family checks

## What guardrail3 is

guardrail3 is a **configuration and architecture enforcer**, not a replacement for clippy/rustc.

It exists to ensure:
- the right Rust tools and lints are configured
- architectural boundaries are enforced
- escape hatches are documented
- dependency boundaries are explicit
- guardrails are difficult for agents to bypass silently

## Rust-only scope

The Rust work currently includes:
- config families: clippy, deny, fmt, toolchain, cargo
- topology/architecture families: topology, arch, apparch
- code families: code, garde, test, deps, release
- shared hook architecture and Rust hook checks

The Rust rule inventory lives under:
- [`.plans/todo/checks/rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs)
- [`.plans/todo/checks/hooks/shared.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/hooks/shared.md)
- [`.plans/todo/checks/hooks/rs.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/hooks/rs.md)

## Architecture

The active architecture plan is:
- [`.plans/todo/checks/2026-03-21-153251-checker-architecture.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/2026-03-21-153251-checker-architecture.md)

The pipeline is:

```text
project walker
  -> ProjectTree
  -> family orchestrator
  -> typed rule inputs
  -> pure rule functions
```

### Rules

Rules must be:
- one rule per file
- pure functions
- given the smallest typed input that represents one local assertion

Rules must not:
- receive `&ProjectTree`
- receive `&dyn FileSystem`
- perform discovery
- parse unrelated files
- loop over unrelated entities

### Orchestrators

Family orchestrators own:
- discovery from `ProjectTree`
- parse-once work
- normalization into family facts
- fan-out into minimal rule inputs

If a rule needs multiple related objects, the orchestrator should pre-bind them.

Example:
- `workspace + member` pair is a valid atomic input
- `workspace + all members` is usually too large for a rule input

### ProjectTree

`ProjectTree` is the shared repository snapshot:
- structure
- cached config-file content
- no rule semantics

Source file content is still streamed by orchestrators on demand.

## Code Layout

The active Rust CLI lives under:
- [`apps/guardrail3-rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3-rs)

The old multi-language app is archived under:
- [`legacy/apps/guardrail3-current`](/Users/tartakovsky/Projects/websmasher/guardrail3/legacy/apps/guardrail3-current)

Important active roots:
- [`apps/guardrail3-rs/Cargo.toml`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3-rs/Cargo.toml)
- [`apps/guardrail3-rs/crates/io/inbound/cli/src/main.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3-rs/crates/io/inbound/cli/src/main.rs)
- [`apps/guardrail3-rs/crates/logic/validate-command/src/lib.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3-rs/crates/logic/validate-command/src/lib.rs)
- [`apps/guardrail3-rs/crates/io/outbound/packages/src/lib.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3-rs/crates/io/outbound/packages/src/lib.rs)
- [`packages/rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs)

## Package Checks Layout

The active Rust checks live under:
- [`packages/rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs)

Pattern per family:
- `g3rs-<family>-types` owns the public family contract
- `g3rs-<family>-ingestion` owns discovery, parsing, and normalization
- `g3rs-<family>-config-checks`, `-source-checks`, and `-filetree-checks` own pure rules for the implemented lanes

Do not treat `legacy/apps/guardrail3-current` as an active implementation target.

## Test Pattern

Do not default to external integration tests for these family modules.

The required pattern for new family code is:
- each rule file has its own rule-specific sidecar test module directory
- wired with:

```rust
#[cfg(test)]
#[path = "rs_fmt_01_exists_tests/mod.rs"]
mod tests;
```

Reason:
- keeps tests close to the family
- avoids widening visibility just for tests
- preserves exact one-rule/one-test traceability
- allows test files to split by attack vector instead of collapsing into one file

Avoid:
- inline `mod tests { ... }` bodies in production files
- exposing internals only for integration-test access
- family-wide grouped sidecar files such as `fmt_tests.rs`, `cargo_tests.rs`, `clippy_tests.rs`, `deny_tests.rs`
- one-off `*_tests.rs` sidecars as the long-term target
- grouped production files such as `rs_clippy_thresholds.rs` or `rs_deny_bans.rs`

## Implementation Order

Rust families should be built in this order:
1. `rs/fmt`
2. `rs/toolchain`
3. `rs/clippy`
4. `rs/deny`
5. `rs/cargo`
6. `rs/code`
7. `rs/apparch`
8. `rs/deps`
9. `rs/garde`
10. `rs/test`
11. `rs/release`

Current status:
- `rs/fmt` family skeleton exists in the new architecture
- `rs/cargo` is the next important family because it proves parent/child and set-fanout inputs

## Current Design Rules

When designing family inputs:
- one input instance should represent one opportunity for the rule to fire
- prefer pair inputs over bag inputs
- prefer set-diff inputs only for rules that are inherently about sets

Good:
- one config file
- one source file
- one workspace/member pair
- one dependency edge
- one workspace membership-set comparison

Bad:
- whole repo plus all related files shoved into one rule

## Hooks

Hook planning has been updated for Rust/shared only:
- [`hooks/shared.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/hooks/shared.md)
- [`hooks/rs.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/hooks/rs.md)

The durable hook requirements now include:
- structural safety
- executable-line matching rather than raw substring matching
- fail-open prevention
- Rust guardrail step presence
- config-change triggering

But hooks are not the current first implementation target. Rust check families come first.

## Do / Don’t

Do:
- use `ProjectTree` as the only shared discovery object
- keep rules pure and tiny
- keep extraction/parsing in orchestrators
- build one family at a time
- require one rule file and one rule-specific sidecar test module directory
- use structured parsers, never regex/grep/`contains()` on config/source semantics

Don’t:
- expand TypeScript scope right now
- give rules oversized inputs “for convenience”
- let rules crawl the tree or filesystem
- use inline production-file test modules as the default
- bundle multiple rule IDs into one production file
- bundle multiple rules into one family-wide test file
- update docs as if any removed legacy handoff file were still current

## Cold Start Reading List

If starting fresh, read in this order:
1. [AGENTS.md](/Users/tartakovsky/Projects/websmasher/guardrail3/AGENTS.md)
2. [`.plans/todo/checks/2026-03-21-153251-checker-architecture.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/2026-03-21-153251-checker-architecture.md)
3. [`.plans/todo/checks/rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs)
4. [`.plans/todo/checks/hooks/shared.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/hooks/shared.md)
5. [`.plans/todo/checks/hooks/rs.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/hooks/rs.md)
6. [`apps/guardrail3-rs/crates/logic/validate-command/src/lib.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3-rs/crates/logic/validate-command/src/lib.rs)
7. [`apps/guardrail3-rs/crates/io/outbound/packages/src/lib.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3-rs/crates/io/outbound/packages/src/lib.rs)
8. one representative finished family under [`packages/rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs)
9. recent files in [`.worklogs`](/Users/tartakovsky/Projects/websmasher/guardrail3/.worklogs)
