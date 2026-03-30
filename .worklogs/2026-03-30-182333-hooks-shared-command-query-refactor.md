# Refactor Hook Command Query Into hooks-shared

**Date:** 2026-03-30 18:23
**Scope:** `apps/guardrail3/crates/app/rs/families/hooks-shared/src/hook_shell.rs`, `apps/guardrail3/crates/app/rs/families/hooks-shared/src/hook_shell/command_query.rs`, `apps/guardrail3/crates/app/rs/families/hooks-rs/src/*`, `apps/guardrail3/crates/app/rs/families/hooks-rs/assertions/src/*`, `apps/guardrail3/crates/app/rs/runtime/Cargo.toml`

## Summary
Moved the repeated hook-shell command resolution logic out of `hooks-rs` rule files into a new shared `hook_shell::command_query` module in `hooks-shared`, then iterated until the `hooks-rs` crate stopped pathological full-build behavior and its full test suite returned to green. The lean `guardrail3 --no-default-features --features family-hooks-rs` runtime path now builds quickly and runs successfully again.

## Context & Problem
`hooks-rs` was the lone remaining Rust family that compiled in isolation but hung or took pathologically long in the real lean binary path. Investigation showed the crate had accumulated multiple large, duplicated mini shell interpreters spread across rule files such as:

- `hook_rs_07_duplication_tool_is_cargo_dupes.rs`
- `hook_rs_08_guardrail_validate_staged_present.rs`
- `hook_rs_10_test_uses_workspace.rs`
- `hook_rs_11_gitleaks_step_present.rs`
- `hook_rs_12_cargo_dupes_step_present.rs`
- `hook_rs_13_cargo_dupes_excludes.rs`
- `hook_rs_09_clippy_denies_warnings.rs`

That duplicated parser/control-flow logic was the wrong architectural boundary and was also what triggered the bad compile behavior. The user explicitly asked to “put it in the hooks shared and fix all this shit,” so the goal was not just to make one command pass, but to move the hook-command understanding into the shared hook layer and then recover the rule semantics and tests on top of it.

## Decisions Made

### Centralize hook command resolution in `hooks-shared`
- **Chose:** add `hook_shell::command_query` as the single shared place for shell command resolution, wrapper handling, command-substitution extraction, function walking, and reachability handling.
- **Why:** the duplicated rule-local parser stacks were the root cause of both the compile pathology and the maintenance drift.
- **Alternatives considered:**
  - Keep patching each rule’s local parser separately — rejected because it would preserve the same bad structure and keep compile cost high.
  - Create a brand-new top-level crate immediately — rejected because `hooks-shared` is already the shared hook semantic layer and is the right home for this abstraction.

### Keep rule files small and semantic
- **Chose:** rewrite the affected `hooks-rs` rules to ask semantic questions of shared resolved-command helpers instead of embedding their own parser engines.
- **Why:** the rules should own “what counts as present/missing,” not “how shell syntax is walked.”
- **Alternatives considered:**
  - Leave some large rule-local parser helpers in place — rejected because that would leave the architecture half-fixed and keep the compile hot spot alive.

### Make wrapper semantics explicit instead of “mostly permissive”
- **Chose:** teach the shared resolver distinct behaviors for normal commands versus external-command wrappers (`env`, `command`, `exec`, shell `-c` payloads), including:
  - proper handling of `bash -lc` / clustered short shell flags
  - `env --split-string`
  - wrapper bypass of shadowed shell functions when appropriate
  - stricter rejection of unknown wrapper flags
  - selective relaxed handling where backgrounded/piped commands should still count for presence checks
- **Why:** the first shared-parser pass fixed the compile problem but surfaced real semantic differences across the hook rules. The right fix was to model those differences explicitly in the shared layer rather than reintroduce ad hoc parser forks.
- **Alternatives considered:**
  - Use one universal “any command mention counts” rule — rejected because `HOOK-RS-08` and `HOOK-RS-13` intentionally differ on whether detached/piped commands count.
  - Push all wrapper special-casing back into each rule — rejected because that would recreate the parser sprawl we were removing.

### Keep existing assertion contracts where they still matter
- **Chose:** align several rule titles/assertion expectations back to the existing hook test contract for `HOOK-RS-03`, `HOOK-RS-05`, `HOOK-RS-10`, `HOOK-RS-12`, and `HOOK-RS-13`.
- **Why:** a chunk of the failures after the parser refactor were not logic bugs; they were contract drift in the human-facing result titles. The tests are the best local source of truth for those strings right now.
- **Alternatives considered:**
  - Rewrite all affected tests to match the new titles — rejected because the title drift was incidental, not an intentional product change.

## Architectural Notes
- `hooks-shared` now owns the reusable hook-shell command query layer.
- `hooks-rs` rule files are thinner and no longer each ship their own parser/wrapper walker.
- The new shared resolver handles:
  - command segmentation and reachability
  - command substitutions, including quoted substitutions while ignoring escaped literals
  - shell wrappers and `-c` payloads
  - `env` split-string handling
  - function resolution, including helper chains and same-line helper definitions
  - function-shadowing vs wrapper bypass distinctions
- `family-hooks-rs` now depends on `family-hooks-shared` transitively in the runtime feature model, which matches the existing family-selection rule that `HooksRs` implies `HooksShared`.

## Information Sources
- Live build/test behavior from repeated runs of:
  - `cargo build --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-hooks-rs`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-hooks-rs --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-hooks-shared --lib`
  - `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 --no-default-features --features family-hooks-rs -- rs validate apps/guardrail3 --family hooks-rs --format json`
  - `cargo check --manifest-path apps/guardrail3/Cargo.toml --quiet`
- Prior context:
  - `.worklogs/2026-03-30-135511-verify-rs-family-split-matrix.md`
  - `.worklogs/2026-03-30-172629-workspace-compile-green-checkpoint.md`
- Current hook-family code and tests under:
  - `apps/guardrail3/crates/app/rs/families/hooks-shared/src/`
  - `apps/guardrail3/crates/app/rs/families/hooks-rs/src/`

## Open Questions / Future Considerations
- The shared hook command-query layer is now broad enough that it may deserve dedicated direct tests of its own beyond the existing `hook_shell` tests. Right now the hook-family rule suites are exercising it indirectly.
- `HOOK-RS-09` still keeps more custom stateful logic than the other migrated rules because it tracks `RUSTFLAGS` and warning-level semantics, but its iterator-heavy compile hot path has been reduced substantially.
- If other hook families appear later, they should consume `hook_shell::command_query` rather than reimplement command walking.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/hooks-shared/src/hook_shell.rs` — shared hook-shell parsing entrypoint and module wiring
- `apps/guardrail3/crates/app/rs/families/hooks-shared/src/hook_shell/command_query.rs` — new shared command-resolution layer
- `apps/guardrail3/crates/app/rs/families/hooks-rs/src/hook_rs_07_duplication_tool_is_cargo_dupes.rs` — representative thin rule using the shared resolver
- `apps/guardrail3/crates/app/rs/families/hooks-rs/src/hook_rs_08_guardrail_validate_staged_present.rs` — representative rule with stricter “presence” semantics
- `apps/guardrail3/crates/app/rs/families/hooks-rs/src/hook_rs_13_cargo_dupes_excludes.rs` — representative rule with relaxed detached-command semantics and stricter flag parsing
- `apps/guardrail3/crates/app/rs/families/hooks-rs/src/hook_rs_09_clippy_denies_warnings.rs` — remaining stateful rule that still needed local support cleanup
- `apps/guardrail3/crates/app/rs/runtime/Cargo.toml` — runtime feature wiring making `family-hooks-rs` include `family-hooks-shared`
- `.worklogs/2026-03-30-135511-verify-rs-family-split-matrix.md` — prior proof that `hooks-rs` was the lone remaining lean-run problem

## Next Steps / Continuation Plan
1. If the user wants this pattern hardened further, add direct unit tests for `hook_shell::command_query` itself instead of relying only on `hooks-rs`/`hooks-shared` rule suites.
2. Re-run the full Rust family lean run matrix if you want to refresh the repo-wide “all families run separately” proof with the now-fixed `hooks-rs` path.
3. If title/message contracts change intentionally later, update both the hook rule files and the corresponding assertion helpers together so this does not regress into string drift again.
