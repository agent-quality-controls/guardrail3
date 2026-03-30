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
- the old ledger owns a larger family contract than the current runtime actually implements

Rule inventory:

- `T-TOOL-04` formatter package presence
  - Should require `prettier` at the owning package root.
  - It is for ensuring the formatter tool is installed at all.
- `TS-FMT-02` formatter config exists
  - Should require a formatter config surface at the nearest owning config root.
  - It is for making formatting policy explicit instead of relying only on package presence.
- `TS-FMT-03` formatter config parseability
  - Should parse and validate the formatter config surface.
  - It is for preventing dead config files and broken formatter setup.
- `TS-FMT-04` format script presence
  - Should require a stable formatting script entrypoint where the contract expects one.
  - It is for CI and developer ergonomics.
- `TS-FMT-05` formatter policy wiring
  - Should verify that the formatter config, script, and package surface line up coherently.
  - It is for preventing half-installed or half-configured formatting setups.

Current implementation mapping:

- `package_deps.rs`
  - `CORE_TOOLS` contains `("T-TOOL-04", "prettier")`
  - `check_additional_tools(...)` currently implements the package-presence part of the family
- `tool_config_checks.rs`
  - currently does not implement prettier config existence, parseability, or format-script checks

Known reconciliation notes:

- the old family contract is already more specific than the current code: only package presence is implemented today
- the placeholder's "nearest config root" ownership is not reflected in current code, which still behaves like a root-level grouped validator
- formatter config file names and precedence are not yet codified in the current TS family surface

Historical/supplemental references:

- `.plans/todo/checks/ts/fmt.md`

Next planning focus:

- define canonical prettier config surfaces and precedence
- add dedicated config/script rules so this family becomes more than a package-presence check
