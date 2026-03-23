# Complete Release Family

**Date:** 2026-03-23 13:08
**Scope:** `.plans/todo/checks/rs/release.md`, `apps/guardrail3/Cargo.toml`, `Cargo.lock`, `apps/guardrail3/crates/ports/outbound/traits/mod.rs`, `apps/guardrail3/crates/adapters/outbound/tool-runner/mod.rs`, `apps/guardrail3/crates/app/rs/checks/rs/mod.rs`, `apps/guardrail3/crates/app/rs/checks/rs/release/*`, `apps/guardrail3/crates/app/rs/checks/rs/deps/test_support.rs`, `apps/guardrail3/crates/app/rs/checks/rs/test/test_support.rs`

## Summary
Built the new `rs/release` family end-to-end in the current checker architecture and updated the release plan to match the implemented state. Also widened the shared `ToolChecker` API so release dry-run checks can use actual command success instead of the old stderr-text heuristic.

## Context & Problem
`rs/release` was the last major Rust family still living only in the old validator split. The old release code was useful as migration seed material, but it had several problems we explicitly did not want to preserve: walkdir-based crate discovery, silent parse/read skips, substring-only workflow checks, and a weak `cargo publish --dry-run` contract that inferred success from stderr text.

At the same time, the new breadth-first phase needed the family implemented to the real architecture standard:
- one rule file per rule
- one test file per rule
- family orchestrator + typed facts/inputs
- fail-closed input failures instead of silent skips

## Decisions Made

### Build `rs/release` Around Typed Repo/Crate/Edge Facts
- **Chose:** A family model with `RepoReleaseFacts`, `PublishableCrateFacts`, `ReleaseEdgeFacts`, and `ReleaseInputFailureFacts`.
- **Why:** The release rules naturally split across repo-level configuration, per-crate metadata, local dependency edges, and binary-only concerns. That keeps each rule input minimal and avoids old validator patterns where every rule had to rediscover state from the filesystem.
- **Alternatives considered:**
  - Reuse the old `release_checks.rs` discovery shape — rejected because it silently skipped unreadable/invalid manifests and bundled multiple concerns in one walkdir scan.
  - Push raw `toml::Value` and workflow YAML into each rule — rejected because it would leak parsing/discovery into rule files and recreate the legacy “rules crawl everything” problem.

### Upgrade `ToolChecker` to Return Command Outcome for Dry-Run Checks
- **Chose:** Add `CommandRunResult { success, stderr }` and a new `run_cargo_publish_dry_run_outcome` method, while keeping the old stderr-only helper as a compatibility wrapper.
- **Why:** `RS-PUB-09` should evaluate actual command success, not “stderr contains the word error.” The old contract was architecturally wrong for a release-family rule and would have forced a knowingly broken implementation.
- **Alternatives considered:**
  - Keep the old stderr-only method and accept heuristic success detection — rejected because it would bake the known audit flaw into the new family.
  - Change the trait in a breaking way and update all callers immediately — rejected because only release needs the richer contract right now; the compatibility wrapper lets other families keep compiling cleanly.

### Parse Workflows Structurally Instead of Using Substring Checks
- **Chose:** Add `serde_yaml` and flatten parsed workflow YAML into structural workflow facts: step `uses`, step `run` lines, env keys, and scalar strings.
- **Why:** The old repo/bin release checks were explicitly too weak because comments or unrelated prose could satisfy them. The new family needed to look at actual YAML step structure while still staying lightweight enough for breadth-first implementation.
- **Alternatives considered:**
  - Preserve raw `contains()` workflow matching — rejected because it was one of the known flaws the new family was supposed to correct.
  - Build a much heavier GitHub Actions semantic model — rejected for the breadth-first phase; the flatter parsed model is strict enough to avoid the old false positives without overbuilding.

### Use `semver` for Version Validation and Compatibility
- **Chose:** Replace the old hand-rolled version parsing logic with the `semver` crate for `RS-PUB-08` and `RS-PUB-11`.
- **Why:** The old helper had limited requirement support and was explicitly flagged during the release audit. This is exactly the sort of low-level correctness that should come from a real parser rather than local string logic.
- **Alternatives considered:**
  - Copy the old semver helper — rejected because it was one of the known weaknesses in the old release code.
  - Defer semver correctness to a later hardening pass — rejected because the parser dependency is cheap and the rule semantics depend on it directly.

## Architectural Notes
- `rs/release` follows the current Rust family pattern:
  - `mod.rs` orchestrator
  - `facts.rs` for discovery/normalization
  - `inputs.rs` for minimal rule inputs
  - one production file per rule
  - one test file per rule
- The family exports:
  - repo rules `RS-RELEASE-01..12`
  - crate rules `RS-PUB-01..14`
  - binary rules `RS-BIN-01..03`
- Workflow checks are intentionally structural but still breadth-first in complexity:
  - actual parsed YAML
  - flattened step/env facts
  - no substring-only rule logic
- `RS-RELEASE-12` is the fail-closed spine for the family. It carries:
  - manifest parse failures
  - release-plz/cliff parse failures
  - workflow YAML parse failures
  - README read failures
- The `ToolChecker` API change is shared infrastructure. Existing families still compile via the compatibility wrapper, while release uses the richer outcome.

## Information Sources
- Planning and architecture:
  - `AGENTS.md`
  - `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
  - `.plans/todo/checks/rs/release.md`
- Old release validator and tests used as migration seeds:
  - `apps/guardrail3/crates/app/rs/validate/release_checks.rs`
  - `apps/guardrail3/crates/app/rs/validate/release_repo_checks.rs`
  - `apps/guardrail3/crates/app/rs/validate/release_crate_checks.rs`
  - `apps/guardrail3/crates/app/rs/validate/release_crate_deps.rs`
  - `apps/guardrail3/crates/app/rs/validate/release_bin_checks.rs`
  - `apps/guardrail3/crates/app/rs/validate/workspace_metadata.rs`
  - `apps/guardrail3/tests/unit/test_release_repo_checks.rs`
  - `apps/guardrail3/tests/unit/test_release_crate_checks.rs`
  - `apps/guardrail3/tests/unit/test_release_crate_deps.rs`
  - `apps/guardrail3/tests/unit/test_release_bin_checks.rs`
  - `apps/guardrail3/tests/unit/test_release_checks.rs`
- Live repo workflow/config examples:
  - `.github/workflows/release.yml`
  - `.github/workflows/binary-release.yml`
  - `.github/workflows/ci.yml`
  - `apps/guardrail3/Cargo.toml`
- Prior worklogs that set the current family standard:
  - `.worklogs/2026-03-22-192520-normalize-check-family-structure.md`
  - `.worklogs/2026-03-22-203352-finish-rust-check-test-hardening.md`
  - `.worklogs/2026-03-23-124049-complete-test-family.md`

## Open Questions / Future Considerations
- The release family is breadth-first complete, not exhaustively hardened. The biggest future audit surface is workflow semantics:
  - `RS-RELEASE-05..07`
  - `RS-BIN-01..02`
  Those are structurally better than the old substring checks, but still a good target for adversarial deepening later.
- `RS-PUB-09` now uses actual command success, but it still depends on the current `ToolChecker` abstraction rather than richer command reporting (stdout, exit code details, etc.). If other families need tool execution outcomes, this port may need a more general command-result shape later.
- The release family tests are intentionally rule-targeted for breadth-first completion. A later hardening pass should add heavier real-tree fixtures for:
  - release-plz coverage gaps
  - target-specific dependency edges
  - workflow false positives / negatives
  - README read failure surfacing

## Key Files for Context
- `AGENTS.md` — current repo rules and architecture constraints
- `.plans/todo/checks/rs/release.md` — release rule inventory and the reconciled contract
- `apps/guardrail3/crates/ports/outbound/traits/mod.rs` — shared `ToolChecker` API, now with `CommandRunResult`
- `apps/guardrail3/crates/adapters/outbound/tool-runner/mod.rs` — production tool-runner implementation
- `apps/guardrail3/crates/app/rs/checks/rs/release/mod.rs` — release family orchestrator
- `apps/guardrail3/crates/app/rs/checks/rs/release/facts.rs` — release-family discovery and normalized fact model
- `apps/guardrail3/crates/app/rs/checks/rs/release/release_support.rs` — shared workflow/dependency helpers
- `apps/guardrail3/crates/app/rs/checks/rs/release/test_support.rs` — rule-level test builders
- `.worklogs/2026-03-22-192520-normalize-check-family-structure.md` — prior normalization of one-rule/one-test structure
- `.worklogs/2026-03-23-124049-complete-test-family.md` — immediately prior breadth-first family completion context

## Next Steps / Continuation Plan
1. Run an adversarial audit specifically against `rs/release` and the updated release plan, focusing on the structurally new workflow rules and edge handling.
2. If the release family audit is clean enough for breadth-first standards, switch from family expansion to the systematic hardening phase across implemented families.
3. Start that hardening phase with the riskiest already-implemented families:
   - `hexarch`
   - `code`
   - `release`
4. For the hardening phase, expand large-rule tests into rule-specific test modules/directories where the scenario count becomes too large for a single sidecar file.
