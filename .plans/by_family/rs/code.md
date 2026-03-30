# RS-CODE

Status: current, implemented, self-hosted, inventory-complete, still the main source-rule policy lane.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/code/`

Current source of truth:

- this file for family planning/status
- `apps/guardrail3/crates/app/rs/families/code/README.md` for family-local behavior

Current state:

- self-hosted with `crates/runtime`, `crates/assertions`, and `test_support`
- current rule inventory is live through `RS-CODE-36`
- detailed semantics were historically tracked in `.plans/todo/checks/rs/code.md`
- two companion docs remain current supplements, not primary contract:
  - `apps/guardrail3/crates/app/rs/families/code/FIXES.md`
  - `apps/guardrail3/crates/app/rs/families/code/EXPANSION.md`
- `code-family-stabilization.md` is tactical history, not current authority

Historical/supplemental references:

- `.plans/todo/checks/rs/code.md`
- `.plans/todo/checks/rs/code-family-stabilization.md`

Next planning focus:

- move any still-live rule inventory details from the old ledger into the README over time
- keep `FIXES.md` for correctness backlog and `EXPANSION.md` for policy ideas; do not let either become an unowned shadow spec
