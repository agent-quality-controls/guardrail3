# Rust Tool-Migration Audit Handoff

## Task

Review the current Rust guardrail families one by one and decide which rules should:
- stay in `guardrail3`
- move to an existing Rust tool/config surface
- be split between `guardrail3` and an external tool

Primary question:
- should any current `guardrail3` Rust rules be migrated into native tooling such as:
  - Clippy
  - rustfmt
  - cargo-deny
  - cargo metadata / Cargo features / workspace policy
  - other established Rust tooling

## Scope

Current Rust family scope from repo handoff:
- config families:
  - `clippy`
  - `deny`
  - `fmt`
  - `toolchain`
  - `cargo`
- topology/architecture families:
  - `topology`
  - `arch`
  - `apparch`
- code families:
  - `code`
  - `garde`
  - `test`
  - `deps`
  - `release`
- shared hooks:
  - shared hook architecture
  - Rust hook checks

Primary plan/docs to read first:
- [AGENTS.md](/Users/tartakovsky/Projects/websmasher/guardrail3/AGENTS.md)
- [.plans/todo/checks/2026-03-21-153251-checker-architecture.md](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/2026-03-21-153251-checker-architecture.md)
- [.plans/todo/checks/rs](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs)
- [.plans/todo/checks/hooks/shared.md](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/hooks/shared.md)
- [.plans/todo/checks/hooks/rs.md](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/hooks/rs.md)

## Audit question per family

For each family, answer:

1. Is the family enforcing something that a native Rust tool already owns better?
2. If yes, can that tool enforce it with:
   - stable config
   - stable built-in lints/rules
   - realistic custom-lint/plugin support
3. If yes, would migrating reduce custom `guardrail3` surface without losing important guarantees?
4. If not, why must it stay in `guardrail3`?

## Decision rubric

Prefer migration out of `guardrail3` when all are true:
- the rule matches the native tool's real semantic ownership
- the tool can enforce it directly and reliably
- the migration reduces custom maintenance
- the tool's reporting/fix flow is at least as good as `guardrail3`

Keep the rule in `guardrail3` when any are true:
- it is cross-file or cross-package architecture policy
- it depends on repo-specific structure or product policy
- it is about tool presence/wiring rather than the tool's own findings
- the native tool cannot express it cleanly
- custom-lint/plugin support is too heavy or too unstable relative to value

## Important distinction

Do not confuse:
- "this family checks a tool config file"
with:
- "the underlying rule belongs inside that tool"

Example:
- `guardrail3` checking that `clippy.toml` exists and has required bans is not the same as those bans being implementable as native Clippy lints.

## Expected output

Produce one section per family:

### Family: `<name>`

- Current purpose:
- Candidate external owner(s):
- Stay / Move / Split:
- Why:
- Exact migration target if moved:
- Exact residual `guardrail3` responsibility if split:
- Confidence:

Then finish with:

## Summary table

Columns:
- family
- decision
- target tool
- reason

## Must-follow method

- Do not guess about external tool capabilities.
- Verify current tool support from official docs or primary sources.
- If custom lint/plugin support is required, evaluate its real implementation burden.
- Be skeptical of migration to custom Clippy lints unless the payoff is clear.
- Distinguish:
  - built-in lint/config support
  - custom lint/plugin ecosystems
  - wrapper/CI policy enforcement

## Likely decision shape to pressure-test

Expected likely pattern:
- `fmt`, `clippy`, `deny`, parts of `deps`
  - strongest candidates for tool-native ownership or split ownership
- `topology`, `arch`, `apparch`, much of `release`, hooks
  - likely stay in `guardrail3`
- `cargo`, `code`, `garde`, `test`
  - likely mixed and worth careful review

Do not assume that pattern is correct. Prove or disprove it.
