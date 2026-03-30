# TS-NPMRC

Status: current family contract, legacy-grouped implementation, already fairly cohesive.

Implementation roots:

- `apps/guardrail3/crates/app/ts/validate/npmrc_check.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/npmrc.md` as the detailed family ledger until the cutover is complete

Current state:

- package-manager root `.npmrc` policy already has a distinct validator file
- the current implementation already emits a compact, family-shaped rule surface
- compared with the other early TS families, this one is closest to the Rust style of a narrow config family

Rule inventory:

- `T11` root `.npmrc` exists
  - Should require a `.npmrc` file at the package-manager root.
  - It is for ensuring pnpm/npm policy is explicit instead of falling back to permissive defaults.
- `T-NPMRC-01` duplicate keys
  - Should error on duplicate `.npmrc` keys.
  - It is for preventing last-wins behavior from silently masking the effective setting.
- `T12` required baseline settings present
  - Should require every baseline `.npmrc` key to be present.
  - It is for enforcing the minimum package-manager hardening surface.
- `T13` required setting values are strong enough
  - Should error when a required key is present but weaker than the baseline value.
  - It is for preventing permissive pnpm settings from quietly eroding the policy.
- `T14` extra settings inventory
  - Should inventory non-baseline `.npmrc` keys.
  - It is for surfacing local package-manager deviations without hiding them.

Current implementation mapping:

- `check_npmrc(...)` orchestrates the family
- `parse_npmrc_settings(...)` extracts key/value settings
- `check_duplicate_keys(...)` implements `T-NPMRC-01`
- `check_expected_settings(...)` implements `T12` and `T13`
- `check_extra_settings(...)` implements `T14`

Known reconciliation notes:

- IDs are partly still generic (`T11`-`T14`) rather than fully family-scoped
- current code assumes the project root and package-manager root are the same surface
- current code has no separate rule for unreadable `.npmrc`; read failures simply short-circuit after existence is reported
- compared with `RS-CARGO` / `RS-DENY`, the main remaining gap is fail-closed input handling rather than rule-boundary confusion

Historical/supplemental references:

- `.plans/todo/checks/ts/npmrc.md`
- `.plans/by_file/ts/npmrc.md`
- `.plans/by_family/rs/cargo.md`
- `.plans/by_family/rs/deny.md`

Next planning focus:

- add a family-owned fail-closed rule for unreadable or malformed required `.npmrc`
- make package-manager-root ownership explicit once TS root discovery is reconciled
- consider renaming the generic `T11`-`T14` ids to fully family-scoped ids when the TS family cutover becomes implementation work
