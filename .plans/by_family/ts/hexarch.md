# TS-HEXARCH

Status: current family contract, legacy-grouped implementation, under-specified relative to `RS-HEXARCH`.

Implementation roots:

- `apps/guardrail3/crates/app/ts/validate/ts_arch_checks.rs`
- architecture portions of `apps/guardrail3/crates/app/ts/validate/eslint_audit.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/hexarch.md` as the detailed family ledger until the cutover is complete

Current state:

- architecture enforcement exists, but still leans on grouped ESLint/boundary logic
- compared with Rust, this family still mixes true app-internal architecture semantics with ESLint config prerequisite checks

Rule inventory:

- `T-ARCH-01` — service/extension hex structure exists.
  - What it should do: require the expected TS app structure under `src/modules`, including canonical container/layer layout.
  - What it is for: this is the structural half of the service/extension architecture contract.
- `T-ARCH-02` — import boundaries are respected.
  - What it should do: reject imports that cross the allowed TS service-layer boundaries.
  - What it is for: structural directories are meaningless if imports can ignore them.
- `T36` — boundary zone definitions exist in ESLint config.
  - What it should do: require ESLint boundaries-zone definitions.
  - What it is for: this is the config-side prerequisite for actual import-zone enforcement.
- `T37` — import-direction rules exist in ESLint config.
  - What it should do: require `boundaries/element-types` or equivalent import-direction configuration.
  - What it is for: this makes the lint config actually enforce the architectural direction rules.
- `T38` — entry-point barrel enforcement exists.
  - What it should do: require `boundaries/entry-point` or equivalent barrel-only import enforcement.
  - What it is for: this prevents deep imports into a module’s internal implementation.
- `T39` — external dependency per-zone bans exist.
  - What it should do: require `boundaries/external` or equivalent per-zone external package restrictions.
  - What it is for: architecture boundaries are incomplete if any zone can import any external library.
- planned route-entry rule — canonical route-wrapper usage is enforced.
  - What it should do: require service route handlers to use the approved wrapper pattern.
  - What it is for: the family contract explicitly wants route entrypoints to be part of the service architecture.
  - Current mixed code note: a similar concern is currently checked as `T50` under the ESLint family, which is a boundary ambiguity still to resolve.

Current code mapping:

- `apps/guardrail3/crates/app/ts/validate/ts_arch_checks.rs`
  - implements `T-ARCH-01` structure checks
  - implements `T-ARCH-02` source-import boundary checks
- `apps/guardrail3/crates/app/ts/validate/eslint_audit.rs`
  - implements `T36`, `T37`, `T38`, `T39`
- current route-entry enforcement is still mixed with generic ESLint policy in `T50`, not cleanly owned here yet

Current doc/code reconciliation notes:

- the old ledger is directionally correct but the live implementation is narrower than the intended family contract
- the current runtime still splits architecture across source scanning and ESLint-config auditing
- `ts/hexarch` depends on a future `ts/arch` family for root ownership, but already owns app-internal service/extension architecture
- compared with `RS-HEXARCH`, TS still lacks:
  - a clear fail-closed contract for malformed required architecture inputs
  - an explicit routed-input boundary from a shared TS placement layer
  - a clean split between config prerequisites and semantic architecture enforcement
- `T36` through `T39` are probably legitimate family rules, but they should be treated as config prerequisites subordinate to the actual architecture semantics rather than the whole family
- route-wrapper enforcement is still mixed with `TS-ESLINT`; that is the clearest current boundary ambiguity
- the older TS ledger also implies evasion-resistant checks such as alias, barrel, and dynamic-import escape handling; those should become explicit rule buckets instead of staying as implicit hardening hopes

Historical/supplemental references:

- `.plans/todo/checks/ts/hexarch.md`
- `.plans/by_family/rs/hexarch.md`
- `.plans/by_family/rs/arch.md`

Next planning focus:

- reconcile TypeScript app-shape detection and boundary ownership with the future TS arch family
- split config prerequisite rules from semantic architecture rules in the family description
- decide whether `T50` route-wrapper enforcement moves into `ts/hexarch` or remains a lint-side bridge rule in `ts/eslint`
- add explicit fail-closed expectations for required owned inputs once TS root routing exists
- name the main evasion categories explicitly once the family runtime is designed:
  - alias normalization
  - barrel laundering
  - dynamic-import escapes
