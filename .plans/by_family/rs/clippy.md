# RS-CLIPPY

Status: current, implemented, heavily hardened, self-hosted.

Implementation roots:

- `apps/guardrail3/crates/app/rs/families/clippy/`
- `apps/guardrail3/crates/domain/modules/clippy/`

Current source of truth:

- this file for family planning/status
- `apps/guardrail3/crates/app/rs/families/clippy/README.md` for family-local behavior
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/` for live implementation

Current state:

- self-hosted with `crates/runtime`, `crates/assertions`, and `test_support`
- detailed live contract still lives in the family rule ledger and README
- family-local status/closure record lives in:
  - `.plans/todo/checks/rs/clippy/FIXES.md`
- by-file clippy config research is no longer primary authority

Historical/supplemental references:

- `.plans/todo/checks/rs/clippy.md`
- `.plans/by_file/rs/clippy-toml.md`
- `.plans/by_file/tools/edge-cases/clippy.md`
- old clippy/deny hardening briefs under `.plans/todo/check_review/test_hardening/14-*`

Next planning focus:

- keep generator/runtime/domain-module parity explicit
- add a family README note if new policy baselines are added outside the current 25-rule contract
