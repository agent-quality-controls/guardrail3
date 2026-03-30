# TS-SPELLING

Status: current family contract, partial legacy implementation only.

Implementation roots:

- spelling-related parts of `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
- spelling-related parts of `apps/guardrail3/crates/app/ts/validate/tool_config_checks.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/spelling.md` as the detailed family ledger until the cutover is complete

Current state:

- spelling enforcement exists only as mixed tool/package logic
- the current runtime implements package presence and config existence, but not the full old ledger yet

Rule inventory:

- `T-TOOL-01` — `cspell` package must be present. This rule exists to ensure the spelling tool is installed at all.
- `T-TOOL-07` — a cspell config file must exist. This rule exists to make spelling policy explicit instead of relying on package presence alone.
- planned spelling-script rule — a standard spelling script should exist where the product contract requires it. This rule exists to keep spelling checks runnable through a stable entrypoint.
- planned spelling-policy wiring rule — the chosen spelling config surface should be wired coherently into the local toolchain. This rule exists to prevent “tool installed but not actually used” drift.

Current implementation mapping:

- `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
  - `CORE_TOOLS` contains `("T-TOOL-01", "cspell")`
  - `check_additional_tools(...)` currently implements the package-presence part of the family
- `apps/guardrail3/crates/app/ts/validate/tool_config_checks.rs`
  - `check_cspell_config(...)` implements `T-TOOL-07`

Implementation status:

- `T-TOOL-01` package presence: implemented
- `T-TOOL-07` config existence: implemented
- spelling script presence: planned only
- spelling policy wiring: planned only

Known reconciliation notes:

- this family is partially real already, but still shares its implementation with generic tool buckets
- the old ledger promises more than the runtime currently enforces

Historical/supplemental references:

- `.plans/todo/checks/ts/spelling.md`
- `.plans/by_file/ts/cspell-json.md`

Next planning focus:

- define nearest-config ownership and script/package split
- map accepted cspell config filenames and script names into explicit family rules before demoting the old TS ledger
