# TS-FMT

Status: current family contract, partial legacy implementation only, much weaker than `RS-FMT`.

Implementation roots:

- formatter-related parts of `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
- formatter-related parts of `apps/guardrail3/crates/app/ts/validate/tool_config_checks.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/fmt.md` as the detailed family ledger until the cutover is complete

Current state:

- formatter enforcement exists only as mixed tool/package logic
- the old ledger owns a larger family contract than the current runtime actually implements
- compared with Rust, this family is still package-presence-led rather than a true formatting-policy family

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

Current implementation mapping:

- `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
  - `CORE_TOOLS` currently contains `("T-TOOL-04", "prettier")`, so package presence is enforced there.
- `apps/guardrail3/crates/app/ts/validate/tool_config_checks.rs`
  - currently does not implement prettier config existence, parseability, or format-script checks.

Implementation status:

- `T-TOOL-04` package presence: implemented
- formatter config existence/parseability: planned only
- formatting script presence: planned only
- formatter policy wiring: planned only

Current doc/code reconciliation notes:

- the old ledger describes the intended family, not the fully implemented current one
- live code currently covers only prettier package presence; the rest of the family is still planned
- this family is one of the clearer examples where the TS grouped validator has less enforcement than the planning surface implies
- compared with `RS-FMT`, TS still lacks:
  - an explicit formatter-config ownership model
  - fail-closed behavior for malformed required formatter config
  - a decision on whether formatting is root-only policy or multi-root policy
  - explicit shadowing/override semantics for nested formatter config surfaces

Historical/supplemental references:

- `.plans/todo/checks/ts/fmt.md`
- `.plans/by_family/rs/fmt.md`

Next planning focus:

- define canonical formatter config surfaces and root ownership
- map the old contract onto actual config filenames and script names before demoting the old TS ledger
- add the missing config/script/wiring checks before demoting the old TS fmt ledger
- decide explicitly whether this family should behave more like:
  - a root-level policy family, as in `RS-FMT`
  - or a local-config family with nearest-root ownership
