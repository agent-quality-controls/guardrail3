# Restore Sidecar Rule Tests

**Date:** 2026-04-05 19:34
**Scope:** extracted content-check runtime crates under `packages/g3-*-content-checks/crates/runtime/src`

## Summary
Restored rule-local sidecar test directories for the extracted content-check packages after the previous cleanup moved tests into generic nested `tests/` modules. The corrected shape is `rule.rs` plus `rule_tests/`, with test modules declared from `rule.rs`.

## Context & Problem
The previous cleanup commit targeted bad parent-relative test paths and missing feature gating in older extracted packages. In doing that, it changed the test shape from sidecar directories to generic nested `tests/` directories under each rule module. That contradicted the desired pattern: tests should stay sidecar, close to the production file, but not inline.

## Decisions Made

### Restore Sidecar Tests Next to `rule.rs`
- **Chose:** Rename every nested `tests/` directory to `rule_tests/` and wire it from `rule.rs` with `#[path = "rule_tests/mod.rs"]`.
- **Why:** This preserves the “tests stay as sidecars” rule while keeping them file-local to the actual production file that owns the rule logic.
- **Alternatives considered:**
  - Keep nested `tests/` directories under the rule module — rejected because it lost the sidecar shape the project is supposed to use.
  - Inline `mod tests` bodies — rejected because the requirement is explicitly sidecar, not inline.
  - Keep the old sibling `../..._tests` directories — rejected because that was the path shape being cleaned up in the first place.

### Keep Feature-Gating Changes Intact
- **Chose:** Leave the runtime/assertions feature-gating fixes from the previous cleanup in place.
- **Why:** Those were still valid; only the test layout interpretation was wrong.
- **Alternatives considered:**
  - Revert the whole previous cleanup commit — rejected because it would reintroduce the gating problems that were actually fixed correctly.

## Architectural Notes
This change moves test ownership from module `mod.rs` files to the `rule.rs` files that actually implement the checks. That keeps the runtime module directory thin (`mod.rs` only exports `rule::check`) and makes the sidecar relationship explicit at the production-file level.

The repo’s current `RS-ARCH-09` still flags `#[path]` usage broadly, including parser sidecars. This work follows the explicit local design decision for sidecar tests rather than trying to satisfy that validator rule by inlining tests.

## Information Sources
- `AGENTS.md` — project-level architecture and testing expectations
- `.worklogs/2026-04-05-192600-older-package-test-path-and-feature-gating-cleanup.md` — prior cleanup that fixed gating but chose the wrong test shape
- existing parser runtime pattern such as `packages/*-parser/crates/parser/runtime/src/parser.rs` + `parser_tests/`

## Open Questions / Future Considerations
- The repo-wide `RS-ARCH-09` rule still disagrees with the chosen sidecar `#[path]` pattern. That mismatch should be resolved deliberately rather than papered over package by package.

## Key Files for Context
- `packages/g3-toolchain-content-checks/crates/runtime/src/rs_toolchain_02_channel_and_components/rule.rs` — representative restored rule-side sidecar wiring
- `packages/g3-fmt-content-checks/crates/runtime/src/rs_fmt_02_settings/rule.rs` — representative restored sidecar wiring in a different package
- `packages/g3-deny-content-checks/crates/runtime/src/advisories/rs_deny_04_deprecated_advisories/rule.rs` — representative deep-module variant
- `.worklogs/2026-04-05-192600-older-package-test-path-and-feature-gating-cleanup.md` — immediate backstory for the correction

## Next Steps / Continuation Plan
1. If more extracted packages are added, follow the same `rule.rs` + `rule_tests/` pattern from the start instead of nested `tests/`.
2. Reconcile the sidecar test design with `RS-ARCH-09` at the rule-definition level so the validator stops fighting the intended pattern.
