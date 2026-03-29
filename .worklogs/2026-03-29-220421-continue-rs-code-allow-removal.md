# Continue RS-CODE Allow Removal

**Date:** 2026-03-29 22:04
**Scope:** `apps/guardrail3/crates/bin/guardrail3/src/main.rs`, `apps/guardrail3/crates/adapters/inbound/cli/modules_cmd.rs`, `apps/guardrail3/crates/app/core/discover.rs`, `apps/guardrail3/crates/app/ts/validate/{tsconfig_check.rs,package_check.rs,package_deps.rs}`

## Summary
Continued the `RS-CODE-04` reduction by removing more avoidable helper-level `#[allow(...)]` sites in the CLI entry layer, Rust discovery, and TypeScript validation helpers. This slice dropped repo-root `RS-CODE-04` from `173` to `153` without weakening checker behavior.

## Context & Problem
After the earlier allow-removal commits, the remaining `RS-CODE-04` surface was no longer dominated by dead test scaffolding. The next removable group was:
- helper functions that still printed/exited internally instead of returning structured output
- short parser helpers carrying stale broad suppressions
- TS validation functions with function-wide `too_many_lines` / `disallowed_methods` suppressions even though the real boundary was a small JSON parse step

The user direction remained the same: keep removing allows where possible instead of replacing them with better reason text.

## Decisions Made

### Push more CLI behavior back to the real boundary
- **Chose:** Make `modules_cmd` return strings/results instead of printing/exiting, and make `DumpGuide` / `DumpTree` in `main.rs` return values that the CLI boundary emits.
- **Why:** These functions were still carrying helper-level output/exit allowances even though their behavior can be expressed as pure return values.
- **Alternatives considered:**
  - Keep the existing helper-local `print_*` / `process::exit` allows — rejected because the behavior was structurally removable.
  - Do a full `run() -> Effect` refactor of the whole CLI entrypoint in this slice — rejected because it was broader than needed for the current bucket reduction.

### Remove stale `detect_rust` allowances directly
- **Chose:** Rewrite the Cargo.toml parse branch in `discover.rs` to use `let Ok(...) = ... else { ... }` and drop the `manual_let_else`, `string_slice`, and `needless_collect` suppressions.
- **Why:** The function no longer needed any of those broad excuses; they were leftover from an older shape.
- **Alternatives considered:**
  - Replace the removed allowances with new reason comments — rejected because the compiler and code shape showed they were unnecessary.
  - Leave `detect_rust` untouched and move on to bigger buckets — rejected because this was a cheap, safe removal.

### Localize TS JSON parsing and split section checks
- **Chose:** Add small parser/section helpers in `tsconfig_check.rs`, `package_check.rs`, and `package_deps.rs`.
- **Why:** The previous function-wide `too_many_lines` and `disallowed_methods` allowances were covering orchestration code rather than a real exceptional boundary. Splitting the checks by concern made the allow sites removable.
- **Alternatives considered:**
  - Keep the large functions and just add stronger reason comments — rejected because it would preserve avoidable exception surface.
  - Defer TS validation cleanup because TypeScript is not the active roadmap — rejected because `RS-CODE` audits these files in the live repo, so the allow debt is still real.

## Architectural Notes
This slice keeps the same code-cleanup rule as the earlier passes:
- helper functions should return data, not emit user-facing IO when that behavior can be pushed outward
- parsing allowances should live at the smallest actual parse boundary, not on whole orchestration functions
- section-heavy validation files should be decomposed by concern so line-count suppressions are not normalized

The TS validators are still legacy-scope code, but this refactor makes them easier to audit and less dependent on broad clippy escapes.

## Information Sources
- Prior worklogs:
  - `.worklogs/2026-03-29-213614-continue-rs-code-allow-cleanup.md`
  - `.worklogs/2026-03-29-215038-continue-rs-code-allow-removal.md`
- Repo-root validation:
  - `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family code --format json`
- Targeted package verification:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-core --lib`
  - `cargo build --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-ts`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-adapters-inbound-cli --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 --bin guardrail3`
- Subagent audits on removable non-boundary allows in `discover.rs`, `main.rs`, and the TS validation files.

## Open Questions / Future Considerations
- `guardrail3-app-ts` still has a pre-existing unrelated unit test import failure in `eslint_plugin_checks_tests.rs`; the crate build is green, but the library test target is not yet clean from this slice alone.
- The largest remaining live `RS-CODE` buckets are now `RS-CODE-32` (`.expect(...)` message quality) and `RS-CODE-24` (`#[path = ...]` usage), with `RS-CODE-04` no longer dominant.
- `main.rs`, `generate.rs`, `init.rs`, and `map.rs` still hold real CLI boundary allows. Further reduction there will require a larger “single emitter / structured effect” pass.
- There is substantial unrelated dirty work in the repo (`Cargo.lock`, `hooks-rs`, code family tests, `project-tree`, AST assertions, release/deps areas). This commit intentionally excludes it.

## Key Files for Context
- `apps/guardrail3/crates/bin/guardrail3/src/main.rs` — CLI boundary gradually pushed toward pure-return helpers instead of helper-local exit/output.
- `apps/guardrail3/crates/adapters/inbound/cli/modules_cmd.rs` — module-list/show commands now return values instead of printing/exiting.
- `apps/guardrail3/crates/app/core/discover.rs` — `detect_rust` no longer carries stale broad clippy suppressions.
- `apps/guardrail3/crates/app/ts/validate/tsconfig_check.rs` — extracted parse and section helpers that replaced function-wide suppressions.
- `apps/guardrail3/crates/app/ts/validate/package_check.rs` — root package validation split into smaller section emitters.
- `apps/guardrail3/crates/app/ts/validate/package_deps.rs` — table-driven dev-dependency checks plus shared root-package loader.
- `.worklogs/2026-03-29-215038-continue-rs-code-allow-removal.md` — the immediate prior allow-removal slice and bucket state before this one.

## Next Steps / Continuation Plan
1. Commit only the six touched source files plus this worklog; do not stage unrelated dirty files.
2. Re-run repo-root `RS-CODE` and confirm the new top buckets remain `RS-CODE-32` and `RS-CODE-24`.
3. Start the next slice on `RS-CODE-24`: canonicalize `#[path = ...]` usage and add same-line `// reason:` only where the attribute is genuinely required.
4. After the `#[path]` sweep, tackle `RS-CODE-32` by rewriting weak `.expect(...)` messages in descending-file-count order.
