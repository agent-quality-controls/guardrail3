# guardrail3 — Project Handoff

This file is the current source of truth for agent work in this repo.

`CLAUDE.md` is now historical context only. If it conflicts with this file, follow `AGENTS.md`.

## Current Direction

guardrail3 is currently focused on **Rust guardrails only**.

TypeScript and deployment work is not the active direction right now:
- do not expand TypeScript rule inventory
- do not spend implementation time on TS/hook/deploy families unless explicitly asked
- treat existing TS code and docs as legacy background, not the current roadmap

The active work is:
- clean Rust rule inventory
- new checker architecture based on `ProjectTree`
- family-by-family migration from old `rs/validate/*` code into new per-family checks

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
- source families: source, hexarch, garde, test, deps, release
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

The repo is no longer a single `src/...` tree. The real layout is under:
- [`apps/guardrail3/crates`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates)

Important roots:
- [`apps/guardrail3/crates/lib.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/lib.rs)
- [`apps/guardrail3/crates/app/core`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/core)
- [`apps/guardrail3/crates/app/rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs)
- [`apps/guardrail3/crates/domain/project_tree.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/domain/project_tree.rs)

## New Checks Layout

The new Rust checks live under:
- [`apps/guardrail3/crates/app/rs/checks`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/checks)

Pattern per family:
- `mod.rs` — orchestrator
- `facts.rs` — normalized family facts
- `inputs.rs` — minimal typed rule inputs
- one file per rule
- sidecar test file next to the family module

Example family:
- [`apps/guardrail3/crates/app/rs/checks/rs/fmt`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/checks/rs/fmt)

This `fmt` family is the current architecture specimen:
- it proves the new layout
- it proves family-local facts and inputs
- it proves orchestrator fan-out
- it proves the test pattern

Do not treat it as fully migrated production wiring yet. It is the reference shape for the next families.

## Test Pattern

Do not default to external integration tests for these family modules.

The preferred pattern for new family code is:
- sidecar test file next to the module
- wired with:

```rust
#[cfg(test)]
#[path = "fmt_tests.rs"]
mod tests;
```

Reason:
- keeps tests close to the family
- avoids widening visibility just for tests
- matches the repo’s existing sidecar pattern

Avoid:
- inline `mod tests { ... }` bodies in production files
- exposing internals only for integration-test access

## Implementation Order

Rust families should be built in this order:
1. `rs/fmt`
2. `rs/toolchain`
3. `rs/clippy`
4. `rs/deny`
5. `rs/cargo`
6. `rs/source`
7. `rs/hexarch`
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
- prefer sidecar test files
- use structured parsers, never regex/grep/`contains()` on config/source semantics

Don’t:
- expand TypeScript scope right now
- give rules oversized inputs “for convenience”
- let rules crawl the tree or filesystem
- use inline production-file test modules as the default
- update docs as if old `CLAUDE.md` architecture were still current

## Cold Start Reading List

If starting fresh, read in this order:
1. [AGENTS.md](/Users/tartakovsky/Projects/websmasher/guardrail3/AGENTS.md)
2. [`.plans/todo/checks/2026-03-21-153251-checker-architecture.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/2026-03-21-153251-checker-architecture.md)
3. [`.plans/todo/checks/rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs)
4. [`.plans/todo/checks/hooks/shared.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/hooks/shared.md)
5. [`.plans/todo/checks/hooks/rs.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/hooks/rs.md)
6. [`apps/guardrail3/crates/domain/project_tree.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/domain/project_tree.rs)
7. [`apps/guardrail3/crates/app/core/project_walker.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/core/project_walker.rs)
8. [`apps/guardrail3/crates/app/rs/checks/rs/fmt`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/checks/rs/fmt)
9. recent files in [`.worklogs`](/Users/tartakovsky/Projects/websmasher/guardrail3/.worklogs)

<!-- gitnexus:start -->
# GitNexus — Code Intelligence

This project is indexed by GitNexus as **guardrail3** (475 symbols, 1155 relationships, 34 execution flows). Use the GitNexus MCP tools to understand code, assess impact, and navigate safely.

> If any GitNexus tool warns the index is stale, run `npx gitnexus analyze` in terminal first.

## Always Do

- **MUST run impact analysis before editing any symbol.** Before modifying a function, class, or method, run `gitnexus_impact({target: "symbolName", direction: "upstream"})` and report the blast radius (direct callers, affected processes, risk level) to the user.
- **MUST run `gitnexus_detect_changes()` before committing** to verify your changes only affect expected symbols and execution flows.
- **MUST warn the user** if impact analysis returns HIGH or CRITICAL risk before proceeding with edits.
- When exploring unfamiliar code, use `gitnexus_query({query: "concept"})` to find execution flows instead of grepping. It returns process-grouped results ranked by relevance.
- When you need full context on a specific symbol — callers, callees, which execution flows it participates in — use `gitnexus_context({name: "symbolName"})`.

## When Debugging

1. `gitnexus_query({query: "<error or symptom>"})` — find execution flows related to the issue
2. `gitnexus_context({name: "<suspect function>"})` — see all callers, callees, and process participation
3. `READ gitnexus://repo/guardrail3/process/{processName}` — trace the full execution flow step by step
4. For regressions: `gitnexus_detect_changes({scope: "compare", base_ref: "main"})` — see what your branch changed

## When Refactoring

- **Renaming**: MUST use `gitnexus_rename({symbol_name: "old", new_name: "new", dry_run: true})` first. Review the preview — graph edits are safe, text_search edits need manual review. Then run with `dry_run: false`.
- **Extracting/Splitting**: MUST run `gitnexus_context({name: "target"})` to see all incoming/outgoing refs, then `gitnexus_impact({target: "target", direction: "upstream"})` to find all external callers before moving code.
- After any refactor: run `gitnexus_detect_changes({scope: "all"})` to verify only expected files changed.

## Never Do

- NEVER edit a function, class, or method without first running `gitnexus_impact` on it.
- NEVER ignore HIGH or CRITICAL risk warnings from impact analysis.
- NEVER rename symbols with find-and-replace — use `gitnexus_rename` which understands the call graph.
- NEVER commit changes without running `gitnexus_detect_changes()` to check affected scope.

## Tools Quick Reference

| Tool | When to use | Command |
|------|-------------|---------|
| `query` | Find code by concept | `gitnexus_query({query: "auth validation"})` |
| `context` | 360-degree view of one symbol | `gitnexus_context({name: "validateUser"})` |
| `impact` | Blast radius before editing | `gitnexus_impact({target: "X", direction: "upstream"})` |
| `detect_changes` | Pre-commit scope check | `gitnexus_detect_changes({scope: "staged"})` |
| `rename` | Safe multi-file rename | `gitnexus_rename({symbol_name: "old", new_name: "new", dry_run: true})` |
| `cypher` | Custom graph queries | `gitnexus_cypher({query: "MATCH ..."})` |

## Impact Risk Levels

| Depth | Meaning | Action |
|-------|---------|--------|
| d=1 | WILL BREAK — direct callers/importers | MUST update these |
| d=2 | LIKELY AFFECTED — indirect deps | Should test |
| d=3 | MAY NEED TESTING — transitive | Test if critical path |

## Resources

| Resource | Use for |
|----------|---------|
| `gitnexus://repo/guardrail3/context` | Codebase overview, check index freshness |
| `gitnexus://repo/guardrail3/clusters` | All functional areas |
| `gitnexus://repo/guardrail3/processes` | All execution flows |
| `gitnexus://repo/guardrail3/process/{name}` | Step-by-step execution trace |

## Self-Check Before Finishing

Before completing any code modification task, verify:
1. `gitnexus_impact` was run for all modified symbols
2. No HIGH/CRITICAL risk warnings were ignored
3. `gitnexus_detect_changes()` confirms changes match expected scope
4. All d=1 (WILL BREAK) dependents were updated

## CLI

- Re-index: `npx gitnexus analyze`
- Check freshness: `npx gitnexus status`
- Generate docs: `npx gitnexus wiki`

<!-- gitnexus:end -->
