# TS-FMT

Status: current family contract, partial legacy implementation only.

Implementation roots:

- formatter-related parts of `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
- formatter-related parts of `apps/guardrail3/crates/app/ts/validate/tool_config_checks.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/fmt.md` as the detailed family ledger until the cutover is complete

Current state:

- formatter enforcement exists only as mixed tool/package logic

Rule inventory:

- `TS-FMT-01` — formatter package is installed.
  - What it should do: require `prettier` in the expected dev dependency surface.
  - What it is for: formatting policy is meaningless if the formatter package is not actually present.
  - Current code mapping: `T-TOOL-04` in `apps/guardrail3/crates/app/ts/validate/package_deps.rs`.
- `TS-FMT-02` — formatter config exists.
  - What it should do: require the canonical Prettier config surface at the owning TS root.
  - What it is for: formatting policy must live in a real config file, not only in convention or package presence.
  - Current code mapping: not implemented as a dedicated rule yet.
- `TS-FMT-03` — formatter config parses.
  - What it should do: reject malformed formatter config.
  - What it is for: a broken config silently drops formatting policy.
  - Current code mapping: not implemented as a dedicated rule yet.
- `TS-FMT-04` — formatting script exists where required.
  - What it should do: require a stable `format` or equivalent package script when that is part of the project contract.
  - What it is for: CI and local workflows need a canonical formatting entrypoint.
  - Current code mapping: not implemented as a dedicated rule yet.
- `TS-FMT-05` — formatter wiring/profile gating is coherent.
  - What it should do: ensure formatting policy is wired only where the root/product contract requires it and is not confused with CSS/tooling families.
  - What it is for: this keeps formatting separate from package policy and CSS/stylelint policy.
  - Current code mapping: only partially represented today by mixed package/tool checks.

Current code mapping:

- `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
  - currently implements formatter package presence via `T-TOOL-04`
- `apps/guardrail3/crates/app/ts/validate/tool_config_checks.rs`
  - does not yet provide a distinct Prettier config/script rule surface

Current doc/code reconciliation notes:

- the old ledger describes the intended family correctly, but the live code only implements a small subset of it
- this family is still mostly a contract split problem, not a parser problem
- formatter policy should stay separate from `ts/package` and `ts/css`; the current mixed runtime does not yet make that separation clean
- the old ledger owns a larger family contract than the current runtime actually implements

Rule inventory:

- `T-TOOL-04` — `prettier` package must be present. This rule exists to ensure the formatter tool is installed at all.
- planned formatter-config existence rule — the active formatter config surface should exist and be parseable. This rule is for making formatting policy explicit instead of relying only on package presence.
- planned formatting-script rule — a standard formatting script should exist where the product contract requires one. This rule is for keeping formatting runnable through a stable entrypoint.
- planned formatter-policy wiring rule — local toolchain wiring should point at the chosen formatter surface correctly. This rule is for preventing “package installed but not actually used” drift.

Current implementation mapping:

- `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
  - `CORE_TOOLS` contains `("T-TOOL-04", "prettier")`
  - `check_additional_tools(...)` currently implements the package-presence part of the family
- no dedicated formatter-config check exists yet in `tool_config_checks.rs`

Implementation status:

- `T-TOOL-04` package presence: implemented
- formatter config existence/parseability: planned only
- formatting script presence: planned only
- formatter policy wiring: planned only

Known reconciliation notes:

- `ts/fmt` is only partially implemented today despite looking like a current family in the old ledger
- unlike `npmrc` and `tsconfig`, this family still needs a real split out of generic tool/package checks before the by-family plan can become authoritative
- the current implementation is much narrower than the intended family contract

Historical/supplemental references:

- `.plans/todo/checks/ts/fmt.md`

Rule inventory:

- `T-TOOL-04` — prettier package presence.
  What it should do: require `prettier` in the root/package-manager `devDependencies`.
  What it is for: make formatter tooling explicit and installable in CI and local development.
- planned: prettier config existence and parseability.
  What it should do: require the canonical prettier config surface and validate that it parses.
  What it is for: prevent “formatter installed but effectively unconfigured” drift.
- planned: formatting script presence where required.
  What it should do: require a canonical formatting script at roots that are expected to expose one.
  What it is for: standardize formatter invocation in CI and local workflows.
- planned: formatter policy wiring.
  What it should do: ensure the repo actually wires Prettier into the expected TS toolchain rather than leaving it as an unused dependency.
  What it is for: make formatting an enforced tool surface rather than a nominal dependency.

Current code mapping:

- `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
  - `CORE_TOOLS` currently contains `("T-TOOL-04", "prettier")`, so package presence is enforced there.
- `apps/guardrail3/crates/app/ts/validate/tool_config_checks.rs`
  - currently does not implement prettier config existence, parseability, or format-script checks.

Current doc/code reconciliation notes:

- the old ledger describes the intended family, not the fully implemented current one
- live code currently covers only prettier package presence; the rest of the family is still planned
- this family is one of the clearer examples where the TS grouped validator has less enforcement than the planning surface implies

Next planning focus:

- define canonical formatter config surfaces and root ownership
- decide the exact script contract (`format`, `format:check`, or profile-specific variants) before splitting code
- map the old contract onto actual config filenames and script names before demoting the old TS ledger
- add the missing config/script/wiring checks before demoting the old TS fmt ledger
