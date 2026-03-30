# TS-TYPECOV

Status: current family contract, partial legacy implementation only.

Implementation roots:

- type-coverage-related parts of `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
- type-coverage-related parts of `apps/guardrail3/crates/app/ts/validate/tool_config_checks.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/typecov.md` as the detailed family ledger until the cutover is complete

Current state:

- type-coverage enforcement exists only as mixed tool/package logic
- the current runtime implements package presence and script presence, but not the full threshold/config contract promised by the old ledger

Rule inventory:

- `T-TOOL-02` — `type-coverage` package must be present. This rule exists to ensure the explicit type-coverage tool is installed.
- `T-TOOL-08` — `type-coverage` script must exist. This rule exists to keep the type-coverage check runnable through a stable entrypoint.
- planned type-coverage config rule — a dedicated config surface should exist and parse when the project uses local type-coverage configuration. This rule exists to make threshold policy explicit.
- planned enforced-threshold rule — the type-coverage threshold should meet the project baseline. This rule exists to make the family about coverage policy, not just tool presence.

Current implementation mapping:

- `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
  - `CORE_TOOLS` contains `("T-TOOL-02", "type-coverage")`
  - `check_additional_tools(...)` currently implements the package-presence part of the family
- `apps/guardrail3/crates/app/ts/validate/tool_config_checks.rs`
  - `check_script(...)` is used to enforce the `type-coverage` script as `T-TOOL-08`

Implementation status:

- `T-TOOL-02` package presence: implemented
- `T-TOOL-08` script presence: implemented
- type-coverage config existence/parseability: planned only
- enforced threshold policy: planned only

Known reconciliation notes:

- the old ledger draws a cleaner family boundary than the current runtime does
- threshold ownership still needs to be made explicit against `ts/tsconfig`

Historical/supplemental references:

- `.plans/todo/checks/ts/typecov.md`

Next planning focus:

- define the exact threshold/config ownership split against `ts/tsconfig`
- decide whether type-coverage config should be nearest-local-config-owned or package-root-owned in the reconciled family plan
