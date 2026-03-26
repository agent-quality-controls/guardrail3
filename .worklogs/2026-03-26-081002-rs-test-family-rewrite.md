# Rewrite RS-TEST Family To 15-Rule Sidecar Shape

**Date:** 2026-03-26 08:10
**Scope:** `.plans/todo/checks/rs/test.md`, `apps/guardrail3/crates/app/core/project_walker.rs`, `apps/guardrail3/crates/app/rs/families/test/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/test/src/**`

## Summary
Rewrote the Rust `RS-TEST` family from the old 19-rule layout into the accepted 15-rule family contract, with one production file per rule and one rule-specific sidecar test directory per rule. Hardened discovery, activation, fail-closed behavior, mutation-hook matching, and rule attack coverage until the family test crate passed cleanly and the repo-level `rs validate --family test` run produced the expected live-family findings on `apps/guardrail3`.

## Context & Problem
The existing `RS-TEST` family had drifted away from the current plan and README contract. The code and tests still reflected an older 19-rule inventory, flat `*_tests.rs` sidecars, and outdated semantics around activation and mutation adoption. The user requirement was not a partial cleanup: the live family had to be brought to exact parity with the accepted README/plan shape without editing the README itself, and the internal test corpus had to be adversarially hardened rather than merely made to compile.

## Decisions Made

### Replace the old 19-rule corpus with the accepted 15-rule inventory
- **Chose:** Delete the old `rs_test_01..19` rule files and replace them with the accepted `RS-TEST-01` through `RS-TEST-15` production files and matching sidecar test directories.
- **Why:** The plan and family README are the accepted contract now. Keeping the older inventory on disk would preserve dead semantics and make the family impossible to reason about.
- **Alternatives considered:**
  - Keep the old files and add a translation layer — rejected because it would preserve stale ownership and duplicate rule meaning.
  - Leave the old test files in place as seeds — rejected because the user explicitly wanted parity and clean organization, not archaeological leftovers.

### Move all rule semantics behind a single family orchestrator with support-only helpers
- **Chose:** Keep discovery/parsing/root normalization in `discover.rs`, `facts.rs`, `inputs.rs`, `parse.rs`, and `lib.rs`, and keep each `rs_test_*` file focused on one rule.
- **Why:** This matches the repo’s current family architecture contract and prevents rule files from crawling the tree or parsing unrelated inputs directly.
- **Alternatives considered:**
  - Let rule files perform local ad hoc discovery — rejected because it breaks the new checker architecture and makes tests harder to isolate.
  - Keep logic embedded in `lib.rs` — rejected because it hides rule ownership and weakens the one-rule/one-file contract.

### Treat async and mutation surfaces as activation-gated and fail closed only when active
- **Chose:** Make `RS-TEST-09` and mutation rules activate only when their documented markers exist, while still surfacing unreadable or malformed required inputs through `RS-TEST-10`.
- **Why:** The README explicitly distinguishes inactive surfaces from active required inputs. Failing closed on inactive config files would produce noise; not failing closed on active inputs would let the family silently skip work.
- **Alternatives considered:**
  - Always require `.config/nextest.toml` and `.cargo/mutants.toml` — rejected because that violates the activation model.
  - Ignore unreadable optional files even when the owning surface is active — rejected because the family is supposed to fail closed once the rule activates.

### Tighten mutation-hook matching to executable command shape rather than substring presence
- **Chose:** Replace raw substring acceptance with shell-token command matching for `cargo mutants` / `cargo-mutants`, rejecting comments, `echo` mentions, and help/version probes.
- **Why:** The family plan explicitly requires executable-line matching for mutation hooks. A parsed executable line is not enough if the rule still accepts any string containing `cargo mutants`.
- **Alternatives considered:**
  - Keep simple `contains()` matching after shell parsing — rejected because it still accepts false positives like `echo cargo mutants`.
  - Require only the exact literal `cargo mutants` command — rejected because real hooks use env wrappers, path-qualified binaries, and cargo toolchain prefixes.

### Harden the test corpus by attack vector, not only by golden-path examples
- **Chose:** Expand rule-specific test directories with adversarial cases for sidecar ownership, runtime/assertions boundaries, proof-site detection, async activation, input failures, mixed-root mutation activation, and stacked fake mutation settings.
- **Why:** The user explicitly asked for repeated “test attack” passes until the family converged. Passing golden tests alone would not prove parity or robustness.
- **Alternatives considered:**
  - Keep only one golden and one failure case per rule — rejected because too many real regressions sit in edge conditions like mixed roots, inactive surfaces, and shell-shape parsing.
  - Push this hardening to later follow-up work — rejected because the ask was to finish the family rewrite, not leave obvious attack gaps behind.

## Architectural Notes
- The live `RS-TEST` family now follows the repo’s required shape: one production rule file and one `*_tests/` sidecar directory per rule.
- `project_walker` now caches `.config/nextest.toml`, which keeps the family aligned with the “cached config-file content” part of the current `ProjectTree` architecture.
- Root facts now own mutation-hook discovery per owned root instead of using a global hook surface list, which prevents cross-root contamination in mixed-root repositories.
- `test_support.rs` stays as a flat helper file because the family README explicitly allows shared generic support there.
- The repo-level `guardrail3 rs validate . --family test --inventory --format json` run still reports large legacy debt outside the family rewrite itself, including `.claude/worktrees/**` and older root/unit test suites. That is expected fallout from validating the whole repo root and not evidence of a regression in the rewritten family crate.

## Information Sources
- `AGENTS.md`
- `apps/guardrail3/crates/app/rs/families/test/README.md`
- `.plans/todo/checks/rs/test.md`
- `apps/guardrail3/crates/app/core/project_walker.rs`
- `apps/guardrail3/crates/app/rs/families/test/src/**`
- `.worklogs/2026-03-25-125847-remove-repo-root-cargo-workspace.md`
- `.worklogs/2026-03-25-125037-kill-root-package-promote-bin-crate.md`
- `.worklogs/2026-03-25-123323-thin-root-facade-and-direct-owner-imports.md`
- `CARGO_TARGET_DIR=/tmp/guardrail3-test-family-round5 cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-test --lib`
- `CARGO_TARGET_DIR=/tmp/guardrail3-app-rs-runtime-round5 cargo check --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-runtime`
- `apps/guardrail3/target/debug/guardrail3 rs validate . --family test --inventory --format json`

## Open Questions / Future Considerations
- The repo-level `rs/test` validation currently scans large legacy and scratch trees such as `.claude/worktrees/**`; if that is not desired long-term, the project-walk/discovery layer needs an explicit exclusion policy rather than per-family ad hoc filtering.
- The rewritten family now enforces `RS-TEST` on older repo test suites that were never structured to satisfy this contract. That repo debt is real and should be migrated or quarantined deliberately.
- There are unrelated dirty changes in `apps/guardrail3/crates/app/hooks/**`, `apps/guardrail3/crates/app/rs/runtime.rs`, and other files outside this commit slice. They are intentionally not part of this worklog’s change set.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/src/lib.rs` — family orchestrator and rule fan-out
- `apps/guardrail3/crates/app/rs/families/test/src/facts.rs` — root discovery, activation facts, mutation-hook matching, fail-closed input collection
- `apps/guardrail3/crates/app/rs/families/test/src/parse.rs` — Rust AST-derived per-file/per-function facts used by rules
- `apps/guardrail3/crates/app/rs/families/test/src/test_support.rs` — family-local fixture helpers for rule tests
- `apps/guardrail3/crates/app/core/project_walker.rs` — config caching needed by `RS-TEST-09`
- `.plans/todo/checks/rs/test.md` — current mirrored plan for the accepted 15-rule contract
- `.worklogs/2026-03-25-125847-remove-repo-root-cargo-workspace.md` — current workspace-root architecture context
- `.worklogs/2026-03-25-125037-kill-root-package-promote-bin-crate.md` — current CLI/package-root context

## Next Steps / Continuation Plan
1. If repo-level `rs/test` validation is meant to be actionable at the repo root, migrate or quarantine the older legacy/unit/adversarial suites that now fail `RS-TEST-01` and `RS-TEST-07`.
2. Decide whether `.claude/worktrees/**` and similar scratch trees should be excluded by the shared project walker rather than left visible to family checks.
3. Keep the family README as the rule contract and avoid reintroducing flat `*_tests.rs` sidecars or multi-rule production files in future edits to this family.
