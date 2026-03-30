# TS-JSCPD

Status: current family contract, legacy-grouped implementation, mostly coherent once content spillover is removed.

Implementation roots:

- `apps/guardrail3/crates/app/ts/validate/jscpd_check.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/jscpd.md` as the detailed family ledger until the cutover is complete

Current state:

- duplication-policy logic exists, but older notes say it still mixes some content-site concerns
- compared with several other TS families, the pure duplication core here is already fairly well formed

Rule inventory:

- `T19` — `.jscpd.json` exists and parses.
  - What it should do: require a jscpd config file and reject invalid JSON.
  - What it is for: duplication policy should be explicit and machine-readable.
- `T20` — duplication threshold is zero.
  - What it should do: require `threshold = 0`.
  - What it is for: the current contract is zero tolerance for duplication rather than “some duplication is fine.”
- `T21` — non-default `minTokens` is inventoried.
  - What it should do: inventory non-default `minTokens` values.
  - What it is for: duplicate-block size tuning is a policy deviation that should stay visible.
- `T22` — ignore patterns are inventoried.
  - What it should do: inventory configured ignore patterns.
  - What it is for: every excluded subtree weakens duplicate detection and should be reviewable.
- `T-JSCPD-01` — `minTokens` is explicitly set.
  - What it should do: warn when `minTokens` is omitted.
  - What it is for: relying on tool defaults makes the duplicate-detection surface implicit.
- `T-JSCPD-02` — `absolute: true` is set.
  - What it should do: warn when `absolute` is missing or not true.
  - What it is for: monorepo results are harder to interpret without absolute paths.
- `T-JSCPD-03` — required ignore patterns exist.
  - What it should do: require the baseline ignore patterns for generated/vendor/build trees.
  - What it is for: this prevents noisy false positives from uninteresting trees.
- `T-JSCPD-04` — `format` is explicitly set.
  - What it should do: warn when the config does not specify scanned languages.
  - What it is for: the language scope should be explicit rather than tool-default-driven.

Current mixed spillover:

- `T60` — content import restriction
- `T61` — velite config exists

Those two are currently implemented in `jscpd_check.rs`, but they are not really duplication-policy rules. They belong to the future `ts/content` family and should move there during the TS family split.

Current code mapping:

- `apps/guardrail3/crates/app/ts/validate/jscpd_check.rs`
  - `check_jscpd(...)` implements `T19`, `T20`, `T21`, `T22`, `T-JSCPD-01`, `T-JSCPD-02`, `T-JSCPD-03`, `T-JSCPD-04`
  - `check_content_import_restriction(...)` currently carries mixed content rule `T60`
  - `check_velite_config(...)` currently carries mixed content rule `T61`

Current doc/code reconciliation notes:

- the old ledger already warned that this file mixes `ts/jscpd` and content-site logic; that warning is still correct
- the actual duplication family is narrower and cleaner than the mixed runtime file suggests
- the main remaining design work is not the duplication core itself; it is removing content-family spillover and deciding how strict inventory-only rules like `T21`/`T22` should remain
- this family should eventually grow explicit fail-closed behavior for malformed required jscpd config, because right now parseability is described but broader input-integrity expectations are still implicit

Historical/supplemental references:

- `.plans/todo/checks/ts/jscpd.md`
- `.plans/by_file/ts/jscpd-json.md`
- `.plans/by_family/rs/code.md`

Next planning focus:

- split pure duplication policy from content-specific site checks
- decide whether `T21` and `T22` remain inventory-only in the final family contract
- keep the family narrow; do not let generic content/code policy grow back into the duplication contract
