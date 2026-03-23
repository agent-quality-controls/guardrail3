# Legacy Plan Archive

This folder stores reviewed top-level plan notes that are no longer the active source of truth.

These files are kept for history and backstory, not as primary implementation targets.

## Archived here

- `release_setup_validator.md`
  - fully superseded by `.plans/todo/checks/rs/release.md` and the implemented `rs/release` family
- `2026-03-15-183125-guardrail3-domains.md`
  - historical domain-flag / TS-era planning note
  - superseded by the Rust-only family architecture and active `checks/rs/*.md` plans
- `migrate_to_ast_parsing.md`
  - partially superseded
  - TypeScript content is obsolete for current direction
  - remaining Rust-only items were carried into:
    - `.plans/todo/checks/rs/code.md`
    - `.plans/todo/checks/rs/test.md`
    - `.plans/todo/checks/rs/release.md`
- `GARDE_GUARDRAILS.md`
  - not fully superseded
  - live Rust-only unmet requirements were carried into:
    - `.plans/todo/checks/rs/garde.md`
    - `.plans/todo/checks/rs/clippy.md`
- `remaining-fixes.md`
  - mostly obsolete TS/CLI backlog
  - remaining Rust items were either already implemented (`RS-TEST-07`) or already covered by the clippy baseline (`std::env::var` centralization)
- `semver_releases.md`
  - mostly superseded by `.plans/todo/checks/rs/release.md` and the implemented `rs/release` family
  - residual canonical `release-plz.toml` / `cliff.toml` semantic expectations were carried into `.plans/todo/checks/rs/release.md`
- `tests_guardrails.md`
  - top-level mutation-hook and mixed Rust/TS test note
  - surviving Rust-only follow-up items were moved into `.plans/todo/checks/2026-03-23-rust-hardening-followups.md`
- `audit/`
  - large adversarial audit backlog from the old mixed-stack validator era
  - surviving Rust/shared carry-forward items were consolidated into `.plans/todo/checks/2026-03-23-rust-hardening-followups.md`
- `checks/deploy/ts.md`
  - was archived during the Rust-only phase
  - later moved back into active planning under `.plans/todo/typescript/`
- `checks/hooks/ts.md`
  - was archived during the Rust-only phase
  - later moved back into active planning under `.plans/todo/typescript/`
- `checks/hooks_deploy_audit.md`
  - was archived during the Rust-only phase
  - later moved back into active planning under `.plans/todo/typescript/`

If a future session needs active Rust requirements, read the `checks/rs/*.md` files first, not the archived copies here.
