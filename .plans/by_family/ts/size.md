# TS-SIZE

Status: current family contract, partial legacy implementation only, still bucket-shaped.

Implementation roots:

- size-related parts of `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
- size-related parts of `apps/guardrail3/crates/app/ts/validate/tool_config_checks.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/size.md` as the detailed family ledger until the cutover is complete

Current state:

- bundle/size-budget enforcement exists only as mixed tool/package logic
- the current runtime implements package presence and one content-profile config check, but not the full size-budget family contract yet
- compared with Rust family standards, this family is still too close to a bag of “size-related things” rather than one clean policy surface

Rule inventory:

- `T-TOOL-05` — `size-limit` package must be present where size budgets are enabled. This rule exists to ensure the size-budget tool is installed.
- `T-TOOL-06` — size-limit preset package must be present where that preset is part of the baseline. This rule exists to keep the expected size-limit profile complete.
- `T-TOOL-11` — size-limit config must exist when the content profile requires it. This rule exists to make bundle/artifact size policy explicit instead of relying only on package presence.
- planned size-budget script rule — a standard size-budget script should exist where the family is enabled. This rule exists to keep size checks runnable through a stable entrypoint.
- planned enforced budget policy rule — configured budgets should meet the family baseline. This rule exists to make the family about size-policy enforcement, not just tool presence.
- planned profile-gating rule — roots that do not require size budgets should stay out of scope. This rule exists to avoid noisy false positives on projects that do not have a size-budget contract.

Current implementation mapping:

- `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
  - `CONTENT_TOOLS` contains `("T-TOOL-05", "size-limit")` and `("T-TOOL-06", "@size-limit/preset-app")`
  - `check_additional_tools(...)` currently implements the package-presence part of the family
- `apps/guardrail3/crates/app/ts/validate/tool_config_checks.rs`
  - `check_tool_configs(...)` emits `T-TOOL-11` when `content_enabled` is true

Implementation status:

- `T-TOOL-05` package presence: implemented
- `T-TOOL-06` preset package presence: implemented
- `T-TOOL-11` content-profile config existence: implemented
- size-budget script presence: planned only
- enforced budget thresholds: planned only
- profile gating beyond the current `content_enabled` switch: planned only

Known reconciliation notes:

- this family is currently content-profile-biased in code, while the old ledger frames it more generally as a size-budget family
- the runtime currently checks only for package/config presence, not concrete budgets
- the main unresolved design question is whether this is:
  - a content/public-web capability family
  - or a general app/package budget family
- until that is decided, root ownership and applicability will remain too fuzzy for a stable implementation split

Historical/supplemental references:

- `.plans/todo/checks/ts/size.md`
- `.plans/by_family/rs/code.md`
- `.plans/by_family/rs/arch.md`

Next planning focus:

- define the exact size-budget config surface and profile gating
- decide whether `ts/size` should stay content-profile-only or grow a more general app/package budget model
- if it stays content-profile-only, make that capability-family dependency explicit in the plan
