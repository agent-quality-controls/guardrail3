# TS-NPMRC

Status: current family contract, legacy-grouped implementation.

Implementation roots:

- `apps/guardrail3/crates/app/ts/validate/npmrc_check.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/npmrc.md` as the detailed family ledger until the cutover is complete

Current state:

- package-manager root `.npmrc` policy already has a distinct validator file
- this is one of the cleanest existing TS families: the current runtime already looks family-shaped even though it still lives inside the grouped TS validator

Rule inventory:

- `T11` — `.npmrc` exists at the package-manager root.
  What it should do: require a root `.npmrc` and inventory success when it exists.
  What it is for: make pnpm policy explicit instead of falling back to permissive defaults.
- `T-NPMRC-01` — duplicate `.npmrc` keys are forbidden.
  What it should do: detect duplicate keys in the same `.npmrc`.
  What it is for: prevent pnpm last-wins behavior from silently masking stricter earlier settings.
- `T12` — every required baseline setting is present.
  What it should do: error when a required baseline key is missing.
  What it is for: ensure the repo gets the full package-manager guardrail baseline rather than a partial config.
- `T13` — required settings are not weaker than baseline.
  What it should do: error when a required key is present with the wrong value.
  What it is for: prevent quiet weakening of dependency strictness, supply-chain, and install-policy controls.
- `T14` — extra `.npmrc` settings are inventoried.
  What it should do: emit inventory/info for non-baseline keys.
  What it is for: make local policy drift visible without pretending every non-baseline setting is automatically wrong.

Current implementation mapping:

- `apps/guardrail3/crates/app/ts/validate/npmrc_check.rs`
  - `check_npmrc(...)` owns the family orchestration.
  - `parse_npmrc_settings(...)` parses key/value lines.
  - `check_duplicate_keys(...)` implements `T-NPMRC-01`.
  - `check_expected_settings(...)` implements `T12` and `T13`.
  - `check_extra_settings(...)` implements `T14`.

Implementation status:

- `T11`: implemented
- `T-NPMRC-01`: implemented
- `T12`: implemented
- `T13`: implemented
- `T14`: implemented

Current doc/code reconciliation notes:

- the old ledger in `.plans/todo/checks/ts/npmrc.md` is substantially aligned with code
- there is no evidence yet of extra hidden rules beyond the five listed above
- the main remaining reconciliation problem is root ownership, not rule semantics

Historical/supplemental references:

- `.plans/todo/checks/ts/npmrc.md`
- `.plans/by_file/ts/npmrc.md`

Next planning focus:

- reconcile package-manager-root ownership with TS package/root discovery
- decide whether parse failures should stay folded into `T11` or become a separate explicit parseability rule in a later cleanup
- once the TS family cutover is further along, this should be one of the first old TS ledgers to demote
