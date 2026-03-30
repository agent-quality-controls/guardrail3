# TS-CSS

Status: current family contract, legacy-grouped implementation.

Implementation roots:

- `apps/guardrail3/crates/app/ts/validate/stylelint_check.rs`
- CSS package/tool portions of `apps/guardrail3/crates/app/ts/validate/package_deps.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/css.md` as the detailed family ledger until the cutover is complete

Current state:

- stylelint/CSS policy exists, but is still mixed with other TS tool checks

Rule inventory:

- `T-STYL-01` — stylelint config exists.
  - What it should do: require a recognized stylelint config file at the owning root.
  - What it is for: CSS policy needs an explicit config surface.
- `T-STYL-02` — `stylelint-config-standard` is extended.
  - What it should do: require the standard stylelint baseline in config.
  - What it is for: this provides the foundational CSS quality rules.
- `T-STYL-03` — `stylelint-config-tailwindcss` is extended.
  - What it should do: require the Tailwind-aware stylelint baseline when the CSS contract uses it.
  - What it is for: Tailwind-aware linting avoids false positives and keeps the CSS stack coherent.
- `T-STYL-04` — `@double-great/stylelint-a11y` plugin is configured.
  - What it should do: require the CSS accessibility plugin in stylelint config.
  - What it is for: accessibility linting is a first-class part of the CSS contract here.
- `T-STYL-05` — required a11y rules are enabled.
  - What it should do: require the configured stylelint a11y rule set.
  - What it is for: plugin presence alone is not enough if the high-value rules are not on.
- `T-STYL-06` — architecture exceptions are explicit.
  - What it should do: require the known intentional disabled-rule exceptions to be present.
  - What it is for: this distinguishes accepted CSS architecture exceptions from accidental drift.

Current mixed package-presence surfaces:

- `T-PLUG-05` — `stylelint` package present
- `T-PLUG-06` — `@double-great/stylelint-a11y` package present
- `T-PLUG-07` — `stylelint-config-standard` package present
- `T-PLUG-08` — `stylelint-config-tailwindcss` package present

Those currently live in `package_deps.rs` and are conceptually CSS-owned, but they are still implemented as generic package-presence checks.

Current code mapping:

- `apps/guardrail3/crates/app/ts/validate/stylelint_check.rs`
  - implements `T-STYL-01`..`T-STYL-06`
- `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
  - currently implements the CSS package-presence spillover

Current doc/code reconciliation notes:

- the old ledger is close to the current runtime for the config/rule half of the family
- package presence and config semantics are still split across two files and should be reconciled under one family contract later

Historical/supplemental references:

- `.plans/todo/checks/ts/css.md`
- `.plans/by_file/ts/stylelintrc-mjs.md`

Next planning focus:

- define exact CSS root ownership and package/config split
- decide whether `tailwind-ban` belongs in `ts/css` or stays with `ts/eslint` as a lint-side CSS/design-token bridge rule
