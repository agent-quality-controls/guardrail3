# Clippy And Deny Coverage Matrix

This is the first-pass rule inventory and hardening ledger for the `rs/clippy` and `rs/deny` families.

It answers four questions for every rule:

1. what production file owns the rule
2. what current `*_tests.rs` file exists
3. what the current tests actually prove
4. what attack classes are still missing before the lane is done

## Family-wide structural findings

### Clippy

- all 22 production rules exist as one-rule-per-file
- all 22 rules are now migrated to rule-specific `*_tests/` directories
- direct generator-backed parity support now exists in clippy test support
- multi-root fixture-backed coverage now exists for `RS-CLIPPY-01`

### Deny

- all 30 production rules exist as one-rule-per-file
- migrated to rule-specific `*_tests/` directories so far:
  - `RS-DENY-01`
  - `RS-DENY-02`
  - `RS-DENY-03`
  - `RS-DENY-04`
  - `RS-DENY-05`
  - `RS-DENY-06`
  - `RS-DENY-07`
  - `RS-DENY-08`
  - `RS-DENY-10`
  - `RS-DENY-11`
  - `RS-DENY-12`
  - `RS-DENY-13`
  - `RS-DENY-15`
  - `RS-DENY-16`
  - `RS-DENY-17`
  - `RS-DENY-09`
  - `RS-DENY-14`
  - `RS-DENY-18`
  - `RS-DENY-19`
  - `RS-DENY-20`
  - `RS-DENY-23`
  - `RS-DENY-24`
  - `RS-DENY-28`
  - `RS-DENY-29`
  - `RS-DENY-30`
- the remaining deny rules still end in flat `*_tests.rs` files
- direct generator-backed parity support now exists in deny test support
- multi-root fixture-backed effective-root coverage now exists for `RS-DENY-01`

## Legacy corpus mapping

These older sources contain reusable attack vectors, but not in the new rule layout or rule-ID vocabulary.

| Source | Reusable vectors | Likely modern targets |
| --- | --- | --- |
| `apps/guardrail3/tests/adversarial_config_tests.rs` | missing `clippy.toml`, missing `deny.toml`, incomplete clippy bans, incomplete deny bans, missing licenses, missing sources, negative no-false-positive checks | `RS-CLIPPY-01`, `RS-CLIPPY-04`, `RS-CLIPPY-05`, `RS-DENY-01`, `RS-DENY-09`, `RS-DENY-14`, `RS-DENY-18` |
| `apps/guardrail3/tests/fixtures/adversarial-configs/incomplete-clippy` | partial method/type baseline fixture | `RS-CLIPPY-04`, `RS-CLIPPY-05`, `RS-CLIPPY-08`, `RS-CLIPPY-15`, `RS-CLIPPY-18`, `RS-CLIPPY-20` |
| `apps/guardrail3/tests/fixtures/adversarial-configs/incomplete-deny-bans` | partial deny ban baseline fixture and false-positive controls for licenses/sources presence | `RS-DENY-09`, `RS-DENY-14`, `RS-DENY-18` |
| `apps/guardrail3/tests/fixtures/adversarial-configs/missing-deny-licenses` | missing licenses fixture | `RS-DENY-14`, `RS-DENY-15`, `RS-DENY-16`, `RS-DENY-17` |
| `apps/guardrail3/tests/fixtures/adversarial-configs/missing-deny-sources` | missing sources fixture | `RS-DENY-18`, `RS-DENY-19`, `RS-DENY-20` |
| `apps/guardrail3/tests/adversarial_generate.rs` | generator drift, multiline formatting drift, comments-only overrides, whitespace normalization, duplicate override content, cross-section deduplication | clippy generator/checker parity infrastructure, especially `RS-CLIPPY-04`, `05`, `06`, `07`, `18`, `19`, `20` |
| `apps/guardrail3/tests/unit/deny_inventory_test.rs` | skip parsing formats and reason handling | `RS-DENY-23`, partly `RS-DENY-24`, `RS-DENY-27`, `RS-DENY-28`, `RS-DENY-29` |
| `apps/guardrail3/crates/app/rs/validate/clippy_coverage.rs` | old missing/extra ban split and reason-field logic | `RS-CLIPPY-04`, `05`, `06`, `07`, `08` |
| `apps/guardrail3/crates/app/rs/validate/deny_inventory.rs` | old skip/ignore inventory parsing | `RS-DENY-23`, `RS-DENY-24` |
| `apps/guardrail3/crates/app/rs/validate/deny_licenses.rs` | old sources/licenses policy branches | `RS-DENY-14`, `15`, `16`, `17`, `18`, `19`, `20` |

## Clippy matrix

Legend:
- current coverage is what the present `*_tests.rs` file proves now
- missing coverage is what still must be added during hardening

| Rule | Production | Current test file | Current coverage | Missing coverage |
| --- | --- | --- | --- | --- |
| `RS-CLIPPY-01` | `rs_clippy_01_coverage.rs` | `rs_clippy_01_coverage_tests/` | real multi-root fixture coverage, nearer-local-config ownership, and exact uncovered-root ownership | nested-root coverage; stronger false-positive controls for sibling non-Rust roots; parity against generated baseline beyond current fixture-backed ownership tests |
| `RS-CLIPPY-02` | `rs_clippy_02_max_struct_bools.rs` | `rs_clippy_02_max_struct_bools_tests/` | generated baseline inventory, wrong/missing value errors, and rule-local parse-error coverage | broad mutation across all relevant policy roots; mixed-profile or local-root baseline attacks; false-positive controls near neighboring thresholds |
| `RS-CLIPPY-03` | `rs_clippy_03_max_fn_params_bools.rs` | `rs_clippy_03_max_fn_params_bools_tests/` | generated baseline inventory, wrong/missing value errors, and rule-local parse-error coverage | broad mutation across all relevant policy roots; mixed-profile or local-root baseline attacks; false-positive controls near neighboring thresholds |
| `RS-CLIPPY-04` | `rs_clippy_04_missing_method_ban.rs` | `rs_clippy_04_missing_method_ban_tests/` | generated service baseline inventory, `garde = false` non-requirement branch, and multi-missing-ban hard errors | broad mutation across all policy roots; false-positive controls for extra project bans; stronger severity exactness across mixed inventory/error outcomes; formatting drift attacks from legacy generator corpus |
| `RS-CLIPPY-05` | `rs_clippy_05_missing_type_ban.rs` | `rs_clippy_05_missing_type_ban_tests/` | generated service baseline inventory, `garde = false` non-requirement branch, library-profile expansion, and multi-missing-ban hard errors | broad mutation across all policy roots; stronger library-profile interaction coverage with `RS-CLIPPY-14`; false-positive controls for extra project bans; formatting drift attacks from legacy generator corpus |
| `RS-CLIPPY-06` | `rs_clippy_06_extra_method_ban.rs` | `rs_clippy_06_extra_method_ban_tests/` | generated service baseline non-hit path, project-specific extra-ban inventory, and `garde = false` conversion of garde-owned method bans into extras | broader mutation across multiple policy roots; false-positive controls when extras match generated baseline after normalization; duplicate-path with different reasons attack |
| `RS-CLIPPY-07` | `rs_clippy_07_extra_type_ban.rs` | `rs_clippy_07_extra_type_ban_tests/` | generated service baseline non-hit path, project-specific extra-ban inventory, `garde = false` conversion of extractor bans into extras, and library-profile non-extra controls | broader mutation across multiple policy roots; false-positive controls when section-aware matching matters; cross-section confusion attack from legacy generator corpus |
| `RS-CLIPPY-08` | `rs_clippy_08_reason_quality.rs` | `rs_clippy_08_reason_quality_tests/` | generated reasoned-entry non-hit path plus missing-reason warnings across methods, types, and macros | false-positive controls for valid long reasons are now partly covered by the canonical baseline but still need broader edge coverage; parity framing against generated baseline can be made more explicit |
| `RS-CLIPPY-09` | `rs_clippy_09_too_many_lines_threshold.rs` | `rs_clippy_09_too_many_lines_threshold_tests/` | generated baseline inventory, wrong/missing value errors, and rule-local parse-error coverage | broad mutation across relevant roots; mixed-profile or local-root attacks; false-positive controls for neighboring threshold keys |
| `RS-CLIPPY-10` | `rs_clippy_10_too_many_arguments_threshold.rs` | `rs_clippy_10_too_many_arguments_threshold_tests/` | generated baseline inventory, wrong/missing value errors, and rule-local parse-error coverage | broad mutation across relevant roots; mixed-profile or local-root attacks; false-positive controls for neighboring threshold keys |
| `RS-CLIPPY-11` | `rs_clippy_11_excessive_nesting_threshold.rs` | `rs_clippy_11_excessive_nesting_threshold_tests/` | generated baseline inventory, wrong/missing value errors, and rule-local parse-error coverage | broad mutation across relevant roots; mixed-profile or local-root attacks; false-positive controls for neighboring threshold keys |
| `RS-CLIPPY-12` | `rs_clippy_12_allowed_placement.rs` | `rs_clippy_12_allowed_placement_tests/` | forbidden nested filename variants and same-root precedence conflict | broad attack that mutates all forbidden placements at once; allowed-root non-hit controls for validation root, workspace root, and standalone package root; nested-root plus same-root precedence in one suite; severity exactness under multi-hit outcomes |
| `RS-CLIPPY-13` | `rs_clippy_13_local_policy_root_baseline.rs` | `rs_clippy_13_local_policy_root_baseline_tests/` | exact missing managed sections, parse failure, and exact-baseline local policy replacement path | broad mutation across all local policy roots; mixed profile or layer cases; owned non-hit assertions for inherited ancestor roots |
| `RS-CLIPPY-14` | `rs_clippy_14_library_global_state.rs` | `rs_clippy_14_library_global_state_tests/` | non-library non-hit path, generated library baseline non-hit path, and exact missing library global-state bans | broad mutation across all library roots; disagreement coverage for current known generator/checker drift on global-state bans |
| `RS-CLIPPY-15` | `rs_clippy_15_trivial_reason.rs` | `rs_clippy_15_trivial_reason_tests/` | generated substantive-reason non-hit path plus placeholder/trivial reason warnings across methods, types, and macros | false-positive controls for borderline but valid reasons; broader edge coverage on placeholder heuristics |
| `RS-CLIPPY-16` | `rs_clippy_16_avoid_breaking_exported_api.rs` | `rs_clippy_16_avoid_breaking_exported_api_tests/` | explicit `false` inventory path, non-published `true` warning, published-library `true` inventory path, and missing-value warning | broader mutation across relevant roots; parity framing against generated baseline can be strengthened |
| `RS-CLIPPY-17` | `rs_clippy_17_test_relaxations.rs` | `rs_clippy_17_test_relaxations_tests/` | generated baseline non-hit path and one warning per enabled relaxation key | broader mutation across all relevant policy roots; exact owned hit and non-hit sets under multi-root coverage; false-positive controls for unrelated booleans |
| `RS-CLIPPY-18` | `rs_clippy_18_duplicate_bans.rs` | `rs_clippy_18_duplicate_bans_tests/` | generated baseline non-hit path and one warning per duplicated path per section across methods/types/macros | same-path-with-different-reasons edge cases are partly covered but broader dedup semantics and distinct-section false-positive controls can still be expanded |
| `RS-CLIPPY-19` | `rs_clippy_19_unknown_keys.rs` | `rs_clippy_19_unknown_keys_tests/` | typo-like managed-key drift warns while unrelated custom keys stay quiet | honest temporary-heuristic matrix; broad mutation across multiple typo variants; parity or explicit doc for generator/checker disagreement |
| `RS-CLIPPY-20` | `rs_clippy_20_macro_bans.rs` | `rs_clippy_20_macro_bans_tests/` | generated macro-baseline inventory path and exact ownership for multiple missing required macro bans | broader mutation across all macro entries; false-positive controls for extra project macros; reason-quality interplay with `RS-CLIPPY-08` and `RS-CLIPPY-15` |
| `RS-CLIPPY-21` | `rs_clippy_21_cognitive_complexity_threshold.rs` | `rs_clippy_21_cognitive_complexity_threshold_tests.rs` | local correct, wrong, and missing threshold value | rule-specific `*_tests/` layout; parity against generated baseline; broad mutation across relevant roots; false-positive controls for neighboring threshold keys |
| `RS-CLIPPY-22` | `rs_clippy_22_type_complexity_threshold.rs` | `rs_clippy_22_type_complexity_threshold_tests.rs` | local correct, wrong, and missing threshold value | rule-specific `*_tests/` layout; parity against generated baseline; broad mutation across relevant roots; false-positive controls for neighboring threshold keys |

## Deny matrix

| Rule | Production | Current test file | Current coverage | Missing coverage |
| --- | --- | --- | --- | --- |
| `RS-DENY-01` | `rs_deny_01_coverage.rs` | `rs_deny_01_coverage_tests/` | real multi-root effective-root coverage, nearer-local-config ownership, uncovered-root ownership, and malformed-allowed-config parse errors | nested-root coverage; stronger precedence/nearest-config attacks beyond current fixture-backed ownership tests; parity against generated deny baseline |
| `RS-DENY-02` | `rs_deny_02_allowed_locations.rs` | `rs_deny_02_allowed_locations_tests/` | all accepted deny filename variants rejected at forbidden nested roots | allowed-root non-hit controls; nested-root broad mutation; same-root interplay with `RS-DENY-03` |
| `RS-DENY-03` | `rs_deny_03_shadowing.rs` | `rs_deny_03_shadowing_tests/` | all accepted filenames shadowing below allowed roots and same-root precedence conflicts | broad mutation across multiple shadow roots; nearest-config precedence attacks in mixed root trees; false-positive controls for legitimately deeper allowed roots |
| `RS-DENY-04` | `rs_deny_04_deprecated_advisories.rs` | `rs_deny_04_deprecated_advisories_tests/` | generated advisories baseline non-hit path and one warning per deprecated advisory key (`vulnerability`, `notice`, `unsound`) | broader mutation across relevant roots; mixed deprecated and canonical advisory keys in the same config |
| `RS-DENY-05` | `rs_deny_05_advisories_baseline.rs` | `rs_deny_05_advisories_baseline_tests/` | generated advisories baseline non-hit path, missing `[advisories]`, independent missing baseline keys, and independent wrong baseline values | broader mutation across relevant roots; mixed missing-and-wrong combinations in one config; stronger parity framing against canonical baseline |
| `RS-DENY-06` | `rs_deny_06_stricter_advisories_inventory.rs` | `rs_deny_06_stricter_advisories_inventory_tests/` | generated advisories baseline non-hit path and independent inventory for `unmaintained = "deny"` / `yanked = "deny"` | broader mutation across relevant roots; mixed stricter and baseline values across multiple effective roots |
| `RS-DENY-07` | `rs_deny_07_graph_all_features.rs` | `rs_deny_07_graph_all_features_tests/` | generated graph baseline non-hit path, missing `[graph]`, and equivalent errors for missing or false `all-features` | broader mutation across relevant roots; interplay with `RS-DENY-08` and more mixed graph-shape cases |
| `RS-DENY-08` | `rs_deny_08_graph_no_default_features.rs` | `rs_deny_08_graph_no_default_features_tests/` | generated graph baseline non-hit path, missing `[graph]`, and equivalent errors for missing or true `no-default-features` | broader mutation across relevant roots; interplay with `RS-DENY-07` and more mixed graph-shape cases |
| `RS-DENY-09` | `rs_deny_09_ban_baseline_complete.rs` | `rs_deny_09_ban_baseline_complete_tests/` | generated service/library baseline non-hit paths, missing canonical bans, fail-closed section checks, and managed-wrapper integrity errors | broad mutation across all deny roots; stronger false-positive controls for project-specific extras; explicit parity framing for the audited `lazy_static` delta |
| `RS-DENY-10` | `rs_deny_10_multiple_versions_floor.rs` | `rs_deny_10_multiple_versions_floor_tests/` | generated multiple-versions baseline non-hit path, missing `[bans]`, missing `multiple-versions`, and weakened `multiple-versions` warnings | broader mutation across relevant roots; stronger parity framing against canonical baseline; mixed baseline and weaker keys in one config |
| `RS-DENY-11` | `rs_deny_11_highlight_inventory.rs` | `rs_deny_11_highlight_inventory_tests/` | generated highlight baseline non-hit path plus inventory for missing or project-specific `highlight` values | broader mutation across root set; parity against canonical baseline; false-positive controls for exact baseline value across multiple roots |
| `RS-DENY-12` | `rs_deny_12_allow_wildcard_paths.rs` | `rs_deny_12_allow_wildcard_paths_tests/` | generated allow-wildcard-paths baseline non-hit path, missing `[bans]`, and equivalent errors for missing or false `allow-wildcard-paths` | broader mutation across all deny roots; interplay with `RS-DENY-13`; false-positive controls for correct user config |
| `RS-DENY-13` | `rs_deny_13_wildcards_inventory.rs` | `rs_deny_13_wildcards_inventory_tests/` | generated wildcards baseline non-hit path plus warnings for missing or project-specific `wildcards` values | broader mutation across all deny roots; exact owned hit and non-hit sets under multi-root coverage; parity against canonical baseline |
| `RS-DENY-14` | `rs_deny_14_license_allow_baseline.rs` | `rs_deny_14_license_allow_baseline_tests/` | generated license baseline non-hit path, missing-license errors, missing `[licenses]`, and exact `private.ignore` enforcement | broad mutation across all license baseline knobs; false-positive controls for supersets, exact matches, and unrelated sections |
| `RS-DENY-15` | `rs_deny_15_confidence_threshold.rs` | `rs_deny_15_confidence_threshold_tests/` | generated confidence-threshold baseline non-hit path, weaker-value warnings, stricter-value inventory, and missing/invalid-value warnings | broader mutation across relevant roots; parity against canonical baseline; integer-value boundary coverage |
| `RS-DENY-16` | `rs_deny_16_copyleft_allowlist.rs` | `rs_deny_16_copyleft_allowlist_tests/` | generated allow-list non-hit path and one warning per added copyleft license | broader mutation across more copyleft families; false-positive controls for approved licenses; parity against canonical baseline |
| `RS-DENY-17` | `rs_deny_17_license_exceptions_inventory.rs` | `rs_deny_17_license_exceptions_inventory_tests/` | generated no-exception non-hit path and one inventory item per named or crate-keyed exception entry | broader mutation across malformed or unnamed entries; unknown-key interplay with `RS-DENY-28`; parity against canonical baseline |
| `RS-DENY-18` | `rs_deny_18_unknown_sources_policy.rs` | `rs_deny_18_unknown_sources_policy_tests/` | generated unknown-source baseline non-hit path, missing `[sources]`, and independent errors for weakened `unknown-registry` / `unknown-git` | broader interplay with `RS-DENY-19` and `RS-DENY-20`; source-policy mutation across more mixed cases |
| `RS-DENY-19` | `rs_deny_19_allow_registry_baseline.rs` | `rs_deny_19_allow_registry_baseline_tests/` | both accepted crates.io forms, missing `[sources]`, missing crates.io, and unexpected extra registries | mixed registry-list order cases; broader root/profile coverage; explicit parity framing against canonical baseline |
| `RS-DENY-20` | `rs_deny_20_allow_git_inventory.rs` | `rs_deny_20_allow_git_inventory_tests/` | empty `allow-git` non-hit path, one warning per non-empty config, and one inventory item per git source | broader mutation across mixed git-entry shapes; interplay with source baseline and unknown-git policy |
| `RS-DENY-21` | `rs_deny_21_tokio_full_ban.rs` | `rs_deny_21_tokio_full_ban_tests.rs` | local missing `full` ban and changed allow-list cases | rule-specific `*_tests/` layout; direct parity against canonical tokio feature-ban baseline; broad mutation across all tokio feature policy axes; false-positive controls for exact canonical allow list |
| `RS-DENY-22` | `rs_deny_22_extra_feature_bans_inventory.rs` | `rs_deny_22_extra_feature_bans_inventory_tests.rs` | local extra feature-ban inventory branch | rule-specific `*_tests/` layout; broad mutation across multiple non-tokio feature bans; parity against canonical baseline; false-positive controls for canonical tokio-only state |
| `RS-DENY-23` | `rs_deny_23_skip_hygiene.rs` | `rs_deny_23_skip_hygiene_tests/` | malformed entry warnings, missing/non-string reason warnings, and valid inventory for modern and legacy shapes | broad mutation across multiple skip entry shapes in one suite; duplicate-entry interplay with `RS-DENY-27`; unknown-key interplay with `RS-DENY-28`; fail-closed handling for malformed container type |
| `RS-DENY-24` | `rs_deny_24_ignore_hygiene.rs` | `rs_deny_24_ignore_hygiene_tests/` | malformed entry warnings, missing/non-string reason warnings, and valid inventory for plain-string and table shapes | broad mutation across multiple ignore entry shapes in one suite; duplicate-entry interplay with `RS-DENY-27`; unknown-key interplay with `RS-DENY-28`; fail-closed handling for malformed container type |
| `RS-DENY-25` | `rs_deny_25_allow_override_channel.rs` | `rs_deny_25_allow_override_channel_tests.rs` | local non-empty allow-list and overlap behavior | rule-specific `*_tests/` layout; broad mutation across empty, non-overlapping, and overlapping allow entries; parity against canonical deny baseline; false-positive controls for absent allow section |
| `RS-DENY-26` | `rs_deny_26_ban_reason_inventory.rs` | `rs_deny_26_ban_reason_inventory_tests.rs` | local inventory when deny ban entries lack reasons | rule-specific `*_tests/` layout; broad mutation across many ban entries in one suite; interplay with canonical baseline completeness; false-positive controls for fully reasoned entries; parity against generated baseline |
| `RS-DENY-27` | `rs_deny_27_duplicate_entries.rs` | `rs_deny_27_duplicate_entries_tests.rs` | local duplicates for deny, ignore, and feature-ban entries | rule-specific `*_tests/` layout; broaden to skip duplicates too; broad mutation across all duplicate-capable sections in one suite; same-id different-shape attacks; false-positive controls for distinct entries that look similar |
| `RS-DENY-28` | `rs_deny_28_unknown_keys.rs` | `rs_deny_28_unknown_keys_tests/` | top-level/core-section unknown keys plus nested skip/ignore/exception/feature-entry unknown keys | broader critical-section sweep including sources and bans sections; false-positive controls for all supported schema keys; fail-closed behavior for unsupported schema nesting |
| `RS-DENY-29` | `rs_deny_29_ignore_accumulation.rs` | `rs_deny_29_ignore_accumulation_tests/` | threshold boundary at 5 versus 6 ignore entries, including mixed valid entry shapes | broader mutation with malformed entries coexisting; false-positive controls below threshold across multiple valid shapes |
| `RS-DENY-30` | `rs_deny_30_wrappers.rs` | `rs_deny_30_wrappers_tests/` | managed non-empty wrapper drift errors and project-specific wrapper additions inventory | broader mutation across multiple wrapper-bearing bans; false-positive controls for exact canonical wrapper set |

## Immediate hardening implications

1. The next concrete code step is now to keep this matrix current while finishing the remaining unmigrated rules.

2. The highest remaining structural gaps are the still-flat deny rules:
   deny `21`, `22`, `25`, `26`, `27`

3. The highest-value semantic gaps after the recent migrations are:
   - broader parity framing for already-migrated rules
   - multi-root rework for any remaining root/profile-sensitive clippy checks that still rely only on helper-built trees
   - remaining deny policy and inventory rules that still have only flat local tests

4. No old corpus item should be ported mechanically.
   Each legacy test or fixture should still be re-expressed as one modern attack vector with exact owned hits, exact owned non-hits, and exact severity.
