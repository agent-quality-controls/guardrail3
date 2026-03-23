# Check Review Backlog

This folder groups the remaining **Rust-side** review backlog into semantic buckets.

Primary source inputs:
- `.plans/todo/checks/2026-03-23-rust-hardening-followups.md`
- `.plans/todo/NEW_CHECKS.md` (Rust-relevant items only)

Out of scope here:
- frontend
- TypeScript
- deploy/TS planning

## Files

- `01-hooks-and-cli.md`
  - hook-family migration gaps
  - mutation-hook rigor
  - CLI/domain-routing mismatches
  - prerequisite tool diagnostics
  - hook generation divergence
- `02-fail-closed-and-input-integrity.md`
  - family fail-open behavior
  - config/source parse failures
  - discovery integrity
- `03-generator-checker-parity.md`
  - generator/checker drift
  - canonical baseline consistency
  - module registry completeness
- `04-plan-hygiene-and-migration-closure.md`
  - stale statuses
  - stale `Current code` pointers
  - lingering legacy helper dependencies
  - docs that should be archived or relabeled
- `05-release-and-policy-decisions.md`
  - release-family semantic gaps
  - root-vs-workspace scope decisions
  - explicit policy forks that need decisions
- `06-new-rust-rule-candidates.md`
  - still-unowned Rust rule ideas extracted from `NEW_CHECKS.md`
  - explicitly separates already-covered vs still-candidate ideas
