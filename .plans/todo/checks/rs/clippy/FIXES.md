# RS-CLIPPY Fixes

Companion repair list for [`../clippy.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/clippy.md).

This file is the concrete follow-up list from the RS-CLIPPY attack audit. It is intentionally more operational than `clippy.md`:

- `clippy.md` is the family contract
- `FIXES.md` is the concrete bug, hardening, and cleanup backlog

Primary implementation roots:

- [`README.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/README.md)
- [`facts.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs)
- [`clippy_support.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/clippy_support.rs)
- [`lib.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/lib.rs)
- [`domain/modules/clippy`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/domain/modules/clippy)

## Status Labels

- `FIX NOW` = unambiguous improvement: stricter guardrails, cleaner architecture, lower drift
- `DECIDE` = real issue, but the repair shape needs an ownership or policy decision first
- `TEST GAP` = existing behavior is not pinned tightly enough
- `DOC DRIFT` = written contract and live code disagree

## FIX NOW

### 1. Fail closed on wrong-shape `.cargo/config*` data for `RS-CLIPPY-24`

- **Problem:** `RS-CLIPPY-24` only errors on TOML syntax failure or missing content. Parseable but invalid shape such as `env = []` is treated as clean.
- **Why this is clearly better:** the rule exists to prevent `CLIPPY_CONF_DIR` discovery bypasses. Treating malformed override surfaces as clean is a direct fail-open.
- **Current code:** [`facts.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs#L401)
- **Acceptance bar:**
  - malformed `env` shape produces an `RS-CLIPPY-24` error
  - missing-content path remains an error
  - sidecars cover syntax error, shape error, and missing-content error

### 2. Stop silently dropping malformed ban entries

- **Problem:** `parse_ban_entries()` ignores non-string entries, tables without string `path`, and other malformed shapes. Ban-driven rules then emit clean inventory as if nothing was wrong.
- **Why this is clearly better:** silent normalization is a guardrail bypass. Structural invalidity should surface as an error or at minimum a warning owned by the family.
- **Current code:** [`clippy_support.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/clippy_support.rs#L145)
- **Affected rules:** `RS-CLIPPY-04/05/06/07/08/15/18/20`
- **Acceptance bar:**
  - malformed ban entries are surfaced explicitly
  - clean inventory is impossible when a managed ban section has malformed entries
  - sidecars cover non-array section, bad array element type, missing `path`, non-string `path`

### 3. Replace `BTreeSet`-based assertion helpers with exact-result assertions

- **Problem:** several assertion helpers collapse findings into sets and do not prove multiplicity or exact counts.
- **Why this is clearly better:** duplicate emissions, over-reporting, and accidental extra findings can currently slip through green tests.
- **Examples:**
  - [`rs_clippy_08_reason_quality.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/assertions/src/rs_clippy_08_reason_quality.rs#L21)
  - [`rs_clippy_15_trivial_reason.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/assertions/src/rs_clippy_15_trivial_reason.rs#L21)
  - [`rs_clippy_18_duplicate_bans.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/assertions/src/rs_clippy_18_duplicate_bans.rs#L21)
- **Acceptance bar:**
  - helpers assert exact count where the scenario implies exact count
  - helpers assert no unexpected findings
  - repeated identical diagnostics fail tests unless explicitly expected

### 4. Import canonical domain-module exports in parity tests

- **Problem:** parity tests hardcode copied method/type/macro inventories instead of importing canonical exports from `domain/modules/clippy`.
- **Why this is clearly better:** one source of truth is cleaner architecture and removes easy drift.
- **Examples:**
  - [`rs_clippy_04_missing_method_ban_tests/parity.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_04_missing_method_ban_tests/parity.rs#L5)
  - [`rs_clippy_05_missing_type_ban_tests/parity.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_05_missing_type_ban_tests/parity.rs#L18)
  - [`rs_clippy_20_macro_bans_tests/parity.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_20_macro_bans_tests/parity.rs#L5)
- **Acceptance bar:**
  - parity tests derive expected inventories from canonical exports
  - no copied full inventory tables remain in rule sidecars

### 5. Distinguish missing values from wrong-type values for managed scalar keys

- **Problem:** wrong-type values degrade into weaker or misleading diagnostics.
- **Cases:**
  - thresholds: non-integer becomes “missing”
  - `RS-CLIPPY-16`: non-bool `avoid-breaking-exported-api` becomes “not set”
  - `RS-CLIPPY-17`: absent/non-bool keys can produce messages claiming ``= true``
- **Current code:**
  - [`clippy_support.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/clippy_support.rs#L183)
  - [`rs_clippy_16_avoid_breaking_exported_api.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_16_avoid_breaking_exported_api.rs#L17)
  - [`rs_clippy_17_test_relaxations.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_17_test_relaxations.rs#L16)
- **Why this is clearly better:** explicit malformed config should not be hidden behind weaker diagnostics.
- **Acceptance bar:**
  - wrong-type inputs produce distinct diagnostics
  - missing inputs produce missing diagnostics
  - sidecars cover both branches for all managed scalar keys

### 6. Tighten `RS-CLIPPY-04/05` end-to-end output proofs

- **Problem:** the “golden” tests do not prove that the runtime emits the full expected inventory; they only sample a couple of messages.
- **Examples:**
  - [`rs_clippy_04_missing_method_ban_tests/golden.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_04_missing_method_ban_tests/golden.rs#L6)
  - [`rs_clippy_05_missing_type_ban_tests/golden.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_05_missing_type_ban_tests/golden.rs#L6)
- **Why this is clearly better:** these are baseline completeness rules. Sample-based tests are too weak.
- **Acceptance bar:**
  - rule output count matches expected canonical count
  - emitted paths match the expected canonical set exactly

### 7. Add missing fail-closed and branch sidecars where behavior is already intended

- **Needed cases:**
  - `RS-CLIPPY-24` missing-content branch
  - `RS-CLIPPY-01` standalone-package coverage branch
  - `RS-CLIPPY-12` same-root precedence at workspace roots and standalone package roots
  - `RS-CLIPPY-16` negative published-library classification cases
  - `RS-CLIPPY-15` empty/whitespace, `fixme`, `fix later`, `tbd`, `...`
  - clean-path user-added bans with real reasons
  - plain-string ban entries that are present for completeness but still bad for reason quality
- **Why this is clearly better:** these are straightforward test-hardening additions with no downside.

## DECIDE

### 8. Fix fail-open handling for malformed routed `Cargo.toml`

- **Problem:** malformed routed manifests can stop being treated as workspace/package roots, suppress `RS-CLIPPY-01`, and distort `RS-CLIPPY-12` and `RS-CLIPPY-16`.
- **Current code:** [`facts.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs#L254), [`facts.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/facts.rs#L830)
- **Decision needed:** should fail-closed classification of routed Cargo roots live inside RS-CLIPPY, or should placement/family-mapper produce stronger root facts so clippy never reparses ownership-critical semantics itself?
- **Desired end state:** malformed routed manifests do not silently erase policy roots.

### 9. Reconcile pure-layer service baseline vs library-only runtime expectations

- **Problem:** canonical clippy generation still adds global-state bans for service pure-layer roots, but runtime expects those bans only for `profile == "library"`.
- **Current code:**
  - generator: [`render.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/domain/modules/clippy/render.rs#L17)
  - runtime: [`clippy_support.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/clippy_support.rs#L121)
- **Decision needed:** should pure-layer service roots really require those global-state bans, or should canonical generation stop emitting them?
- **Desired end state:** generator, runtime expectations, and plan text agree.

### 10. Collapse duplicate threshold parse errors into one coherent malformed-config result

- **Problem:** one malformed `clippy.toml` produces seven separate threshold parse errors.
- **Current code:** [`lib.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/lib.rs#L87)
- **Decision needed:** should malformed `clippy.toml` be owned by one family-level parseability rule, one per-config error, or remain per-rule by design?
- **Desired end state:** malformed config is reported once in a way that is strict without being noisy.

### 11. Decide whether `RS-CLIPPY-13` should suppress itself when `RS-CLIPPY-23` owns policy-context failure

- **Problem:** `RS-CLIPPY-13` returns early on `policy_context_parse_error`.
- **Current code:** [`rs_clippy_13_local_policy_root_baseline.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_13_local_policy_root_baseline.rs#L15)
- **Decision needed:** is this proper single-owner fail-closed behavior via `RS-CLIPPY-23`, or should local policy roots also emit an error about incomplete/unresolvable baseline?
- **Desired end state:** explicit single-owner behavior with sidecars proving it.

### 12. Decide the clean-path inventory contract for `RS-CLIPPY-06/07`

- **Problem:** `RS-CLIPPY-06` and `RS-CLIPPY-07` emit nothing when clean, which appears to conflict with the broader family inventory contract.
- **Current code:**
  - [`rs_clippy_06_extra_method_ban.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_06_extra_method_ban.rs#L12)
  - [`rs_clippy_07_extra_type_ban.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_07_extra_type_ban.rs#L12)
- **Decision needed:** should inventory-only extra-ban rules produce positive clean inventory or intentionally stay silent?
- **Desired end state:** one consistent rule-family inventory contract.

### 13. Revisit the typo heuristic for `RS-CLIPPY-19`

- **Problem:** normalized Levenshtein `<= 2` may over-warn on borderline unknown keys.
- **Current code:** [`rs_clippy_19_unknown_keys.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_19_unknown_keys.rs#L27)
- **Decision needed:** keep the heuristic and just pin more boundary cases, or narrow the detection rule.
- **Desired end state:** typo detection catches realistic managed-key mistakes without noisy false positives.

## TEST GAP

### 14. Add malformed policy-context short-circuit coverage for profile-sensitive rules

- **Missing coverage:** `RS-CLIPPY-04/05/06/07/14` currently short-circuit when `RS-CLIPPY-23` owns malformed `guardrail3.toml`, but sidecars do not prove that contract.
- **Current code examples:**
  - [`rs_clippy_04_missing_method_ban.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_04_missing_method_ban.rs#L13)
  - [`rs_clippy_14_library_global_state.rs`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_14_library_global_state.rs#L13)

### 15. Add malformed ban-shape coverage to ban-quality rules

- **Missing coverage:** `RS-CLIPPY-08/15/18` only test valid-array-but-bad-content scenarios.
- **Needed cases:** non-array section, mixed arrays, bad table shape, non-string `path`.

### 16. Add exact boundary tests for published-library classification

- **Missing coverage:** only positive published-library cases are pinned.
- **Needed cases:** `publish = false`, malformed `publish`, non-publishable packages/workspaces.

### 17. Add wrong-type managed-key sidecars across the threshold and policy-bool cluster

- **Missing coverage:** wrong-type threshold values, non-bool `avoid-breaking-exported-api`, non-bool test-relaxation keys.

### 18. Add cross-rule tests for “present but low-quality” string-form ban entries

- **Missing coverage:** prove that a plain string ban entry still counts for completeness/extra-inventory while `RS-CLIPPY-08` warns about missing `reason`.

## DOC DRIFT

### 19. Update the plan text to match live rule reality

- `RS-CLIPPY-05` still describes “10 base types” while the live baseline is larger.
- `RS-CLIPPY-14` still says the `_profile` parameter is ignored, but the rule explicitly gates on `profile_name == "library"`.
- **File:** [`clippy.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/clippy.md#L213)

### 20. Document the chosen single-owner malformed-input model

- If malformed `guardrail3.toml`, malformed `clippy.toml`, malformed `Cargo.toml`, and malformed `.cargo/config*` are intentionally owned by different rule IDs, the family README and plan should say that explicitly.
- **Files:**
  - [`README.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/clippy/README.md)
  - [`clippy.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/clippy.md)

## Recommended Execution Order

1. Fix wrong-shape `.cargo/config*` fail-closed behavior for `RS-CLIPPY-24`.
2. Fix malformed ban-entry handling at the shared parser layer.
3. Harden assertion helpers and `RS-CLIPPY-04/05` exact-output tests.
4. Replace copied parity inventories with canonical exports.
5. Fix wrong-type scalar diagnostics for thresholds, `RS-CLIPPY-16`, and `RS-CLIPPY-17`.
6. Decide and repair routed `Cargo.toml` fail-open ownership.
7. Decide and reconcile pure-layer service baseline semantics.
8. Decide malformed-config aggregation behavior for threshold rules.
9. Decide `RS-CLIPPY-13` ownership when `RS-CLIPPY-23` fires.
10. Decide whether `RS-CLIPPY-06/07` should inventory positively on clean paths.
11. Update `clippy.md` and `README.md` so the written contract matches the chosen behavior.
