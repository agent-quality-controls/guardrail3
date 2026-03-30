# Clippy And Deny Coverage Matrix

> Historical hardening ledger. Rule counts and file paths in this matrix can drift after family migration.
> Use:
> - [`.plans/todo/checks/rs/clippy.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/clippy.md)
> - [`.plans/todo/checks/rs/deny.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/deny.md)
> for current live inventory.

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
- verified architecture note: in the shared golden fixture, `packages/shared-types` is a workspace member, not a standalone policy root; the real mixed-profile bug was root package-profile resolution in `clippy/facts.rs`

### Deny

- all 30 production rules exist as one-rule-per-file
- all 30 rules are now migrated to rule-specific `*_tests/` directories
- direct generator-backed parity support now exists in deny test support
- multi-root fixture-backed effective-root coverage now exists for `RS-DENY-01`
- verified architecture note: the real mixed-profile bug was not standalone-root classification in the shared fixture; it was root package-profile resolution in `deny/facts.rs` and per-app profile selection in `generate_helpers.rs`

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
| `RS-CLIPPY-01` | `rs_clippy_01_coverage.rs` | `rs_clippy_01_coverage_tests/` | real multi-root fixture coverage, nearer-local-config ownership, exact uncovered-root ownership, and explicit non-Rust-root non-hit control in the shared fixture | nested-root coverage; parity against generated baseline beyond current fixture-backed ownership tests |
| `RS-CLIPPY-02` | `rs_clippy_02_max_struct_bools.rs` | `rs_clippy_02_max_struct_bools_tests/` | generated baseline inventory, wrong/missing value errors, rule-local parse-error coverage, and local-policy-root baseline coverage | broad mutation across all relevant policy roots; mixed-profile attacks; false-positive controls near neighboring thresholds |
| `RS-CLIPPY-03` | `rs_clippy_03_max_fn_params_bools.rs` | `rs_clippy_03_max_fn_params_bools_tests/` | generated baseline inventory, wrong/missing value errors, rule-local parse-error coverage, and local-policy-root baseline coverage | broad mutation across all relevant policy roots; mixed-profile attacks; false-positive controls near neighboring thresholds |
| `RS-CLIPPY-04` | `rs_clippy_04_missing_method_ban.rs` | `rs_clippy_04_missing_method_ban_tests/` | generated service baseline inventory, `garde = false` non-requirement branch, multi-missing-ban hard errors, and exact generator-vs-checker parity for managed method bans | broad mutation across all policy roots; false-positive controls for extra project bans; stronger severity exactness across mixed inventory/error outcomes; formatting drift attacks from legacy generator corpus |
| `RS-CLIPPY-05` | `rs_clippy_05_missing_type_ban.rs` | `rs_clippy_05_missing_type_ban_tests/` | generated service baseline inventory, `garde = false` non-requirement branch, library-profile expansion, multi-missing-ban hard errors, and exact generator-vs-checker parity for managed type bans | broad mutation across all policy roots; stronger library-profile interaction coverage with `RS-CLIPPY-14`; false-positive controls for extra project bans; formatting drift attacks from legacy generator corpus |
| `RS-CLIPPY-06` | `rs_clippy_06_extra_method_ban.rs` | `rs_clippy_06_extra_method_ban_tests/` | generated service baseline non-hit path, project-specific extra-ban inventory, `garde = false` conversion of garde-owned method bans into extras, and exact parity for the managed method baseline | broader mutation across multiple policy roots; false-positive controls when extras match generated baseline after normalization; duplicate-path with different reasons attack |
| `RS-CLIPPY-07` | `rs_clippy_07_extra_type_ban.rs` | `rs_clippy_07_extra_type_ban_tests/` | generated service baseline non-hit path, project-specific extra-ban inventory, `garde = false` conversion of extractor bans into extras, library-profile non-extra controls, and exact parity for the managed type baseline | broader mutation across multiple policy roots; false-positive controls when section-aware matching matters; cross-section confusion attack from legacy generator corpus |
| `RS-CLIPPY-08` | `rs_clippy_08_reason_quality.rs` | `rs_clippy_08_reason_quality_tests/` | generated reasoned-entry non-hit path, missing-reason warnings across methods/types/macros, and exact parity that canonical entries use table format with reasons | false-positive controls for valid long reasons still need broader edge coverage |
| `RS-CLIPPY-09` | `rs_clippy_09_too_many_lines_threshold.rs` | `rs_clippy_09_too_many_lines_threshold_tests/` | generated baseline inventory, wrong/missing value errors, and rule-local parse-error coverage | broad mutation across relevant roots; mixed-profile or local-root attacks; false-positive controls for neighboring threshold keys |
| `RS-CLIPPY-10` | `rs_clippy_10_too_many_arguments_threshold.rs` | `rs_clippy_10_too_many_arguments_threshold_tests/` | generated baseline inventory, wrong/missing value errors, and rule-local parse-error coverage | broad mutation across relevant roots; mixed-profile or local-root attacks; false-positive controls for neighboring threshold keys |
| `RS-CLIPPY-11` | `rs_clippy_11_excessive_nesting_threshold.rs` | `rs_clippy_11_excessive_nesting_threshold_tests/` | generated baseline inventory, wrong/missing value errors, and rule-local parse-error coverage | broad mutation across relevant roots; mixed-profile or local-root attacks; false-positive controls for neighboring threshold keys |
| `RS-CLIPPY-12` | `rs_clippy_12_allowed_placement.rs` | `rs_clippy_12_allowed_placement_tests/` | forbidden nested filename variants, same-root precedence conflict, and allowed-root non-hit controls for validation/workspace/standalone-package roots | broad attack that mutates all forbidden placements at once; nested-root plus same-root precedence in one suite; severity exactness under multi-hit outcomes |
| `RS-CLIPPY-13` | `rs_clippy_13_local_policy_root_baseline.rs` | `rs_clippy_13_local_policy_root_baseline_tests/` | exact missing managed sections, parse failure, exact-baseline local policy replacement path, and explicit non-hit control for the validation-root config | broad mutation across all local policy roots; mixed profile or layer cases; owned non-hit assertions for inherited ancestor roots |
| `RS-CLIPPY-14` | `rs_clippy_14_library_global_state.rs` | `rs_clippy_14_library_global_state_tests/` | non-library non-hit path, generated library baseline non-hit path, exact missing library global-state bans, and exact generator-vs-checker parity for library global-state bans | broad mutation across all library roots |
| `RS-CLIPPY-15` | `rs_clippy_15_trivial_reason.rs` | `rs_clippy_15_trivial_reason_tests/` | generated substantive-reason non-hit path, placeholder/trivial reason warnings across methods/types/macros, and exact parity that canonical reasons are non-placeholder | false-positive controls for borderline but valid reasons; broader edge coverage on placeholder heuristics |
| `RS-CLIPPY-16` | `rs_clippy_16_avoid_breaking_exported_api.rs` | `rs_clippy_16_avoid_breaking_exported_api_tests/` | explicit `false` inventory path, non-published `true` warning, published-library `true` inventory path, missing-value warning, and exact parity for the generated managed boolean | broader mutation across relevant roots; mixed publishability/profile cases |
| `RS-CLIPPY-17` | `rs_clippy_17_test_relaxations.rs` | `rs_clippy_17_test_relaxations_tests/` | generated baseline non-hit path, one warning per enabled relaxation key, and exact parity that canonical relaxations stay disabled | broader mutation across all relevant policy roots; exact owned hit and non-hit sets under multi-root coverage; false-positive controls for unrelated booleans |
| `RS-CLIPPY-18` | `rs_clippy_18_duplicate_bans.rs` | `rs_clippy_18_duplicate_bans_tests/` | generated baseline non-hit path, one warning per duplicated path per section across methods/types/macros, and exact parity that canonical ban sections stay duplicate-free | same-path-with-different-reasons edge cases are partly covered but broader dedup semantics and distinct-section false-positive controls can still be expanded |
| `RS-CLIPPY-19` | `rs_clippy_19_unknown_keys.rs` | `rs_clippy_19_unknown_keys_tests/` | typo-like managed-key drift warns while unrelated custom keys stay quiet, and exact parity that generated top-level keys are all managed keys | honest temporary-heuristic matrix; broad mutation across multiple typo variants |
| `RS-CLIPPY-20` | `rs_clippy_20_macro_bans.rs` | `rs_clippy_20_macro_bans_tests/` | generated macro-baseline inventory path, exact ownership for multiple missing required macro bans, and exact generator-vs-checker parity for managed macro bans | broader mutation across all macro entries; false-positive controls for extra project macros; reason-quality interplay with `RS-CLIPPY-08` and `RS-CLIPPY-15` |
| `RS-CLIPPY-21` | `rs_clippy_21_cognitive_complexity_threshold.rs` | `rs_clippy_21_cognitive_complexity_threshold_tests.rs` | local correct, wrong, and missing threshold value | rule-specific `*_tests/` layout; parity against generated baseline; broad mutation across relevant roots; false-positive controls for neighboring threshold keys |
| `RS-CLIPPY-22` | `rs_clippy_22_type_complexity_threshold.rs` | `rs_clippy_22_type_complexity_threshold_tests.rs` | local correct, wrong, and missing threshold value | rule-specific `*_tests/` layout; parity against generated baseline; broad mutation across relevant roots; false-positive controls for neighboring threshold keys |

## Deny matrix

| Rule | Production | Current test file | Current coverage | Missing coverage |
| --- | --- | --- | --- | --- |
| `RS-DENY-01` | `rs_deny_01_coverage.rs` | `rs_deny_01_coverage_tests/` | real multi-root effective-root coverage, nearer-local-config ownership, local `.cargo/deny.toml` ownership, same-root precedence coverage, uncovered-root ownership, malformed-allowed-config parse errors, and malformed `guardrail3.toml` policy-context ownership | nested-root coverage; parity against generated deny baseline |
| `RS-DENY-02` | `rs_deny_02_allowed_locations.rs` | `rs_deny_02_allowed_locations_tests/` | all accepted deny filename variants rejected at forbidden nested roots, plus allowed validation/workspace-root non-hit controls | nested-root broad mutation; same-root interplay with `RS-DENY-03` |
| `RS-DENY-03` | `rs_deny_03_shadowing.rs` | `rs_deny_03_shadowing_tests/` | all accepted filenames shadowing below allowed roots, same-root precedence conflicts, and non-hit coverage for allowed local policy roots | broad mutation across multiple shadow roots; nearest-config precedence attacks in mixed root trees |
| `RS-DENY-04` | `rs_deny_04_deprecated_advisories.rs` | `rs_deny_04_deprecated_advisories_tests/` | generated advisories baseline non-hit path, one warning per deprecated advisory key (`vulnerability`, `notice`, `unsound`), and local-root ownership replacement for deprecated advisory drift | mixed deprecated and canonical advisory keys in the same config |
| `RS-DENY-05` | `rs_deny_05_advisories_baseline.rs` | `rs_deny_05_advisories_baseline_tests/` | generated advisories baseline non-hit path, missing `[advisories]`, independent missing baseline keys, independent wrong baseline values, and local-root ownership replacement for weakened advisory baseline | mixed missing-and-wrong combinations in one config; stronger parity framing against canonical baseline |
| `RS-DENY-06` | `rs_deny_06_stricter_advisories_inventory.rs` | `rs_deny_06_stricter_advisories_inventory_tests/` | generated advisories baseline non-hit path, independent inventory for `unmaintained = "deny"` / `yanked = "deny"`, and local-root ownership replacement for stricter advisory policy | mixed stricter and baseline values across multiple effective roots |
| `RS-DENY-07` | `rs_deny_07_graph_all_features.rs` | `rs_deny_07_graph_all_features_tests/` | generated graph baseline non-hit path, missing `[graph]`, equivalent errors for missing or false `all-features`, and local-root ownership replacement for graph drift | interplay with `RS-DENY-08` and more mixed graph-shape cases |
| `RS-DENY-08` | `rs_deny_08_graph_no_default_features.rs` | `rs_deny_08_graph_no_default_features_tests/` | generated graph baseline non-hit path, missing `[graph]`, equivalent errors for missing or true `no-default-features`, and local-root ownership replacement for graph drift | interplay with `RS-DENY-07` and more mixed graph-shape cases |
| `RS-DENY-09` | `rs_deny_09_ban_baseline_complete.rs` | `rs_deny_09_ban_baseline_complete_tests/` | generated service/library baseline tests, missing canonical bans, fail-closed section checks, managed-wrapper integrity errors, exact generator-vs-checker parity for both service and library ban sets, and standalone-app routed library-profile coverage | tighten false-positive controls for project-specific extras; broaden mixed-root/profile attacks where local deny roots replace ancestor policy |
| `RS-DENY-10` | `rs_deny_10_multiple_versions_floor.rs` | `rs_deny_10_multiple_versions_floor_tests/` | generated multiple-versions baseline non-hit path, missing `[bans]`, missing `multiple-versions`, weakened `multiple-versions` warnings, and local-root ownership replacement for multiple-versions drift | stronger parity framing against canonical baseline; mixed baseline and weaker keys in one config |
| `RS-DENY-11` | `rs_deny_11_highlight_inventory.rs` | `rs_deny_11_highlight_inventory_tests/` | generated highlight baseline non-hit path, inventory for missing or project-specific `highlight` values, and local-root ownership replacement for highlight drift | parity against canonical baseline; false-positive controls for exact baseline value across multiple roots |
| `RS-DENY-12` | `rs_deny_12_allow_wildcard_paths.rs` | `rs_deny_12_allow_wildcard_paths_tests/` | generated allow-wildcard-paths baseline non-hit path, missing `[bans]`, equivalent errors for missing or false `allow-wildcard-paths`, and local-root ownership replacement for wildcard-path drift | interplay with `RS-DENY-13`; false-positive controls for correct user config |
| `RS-DENY-13` | `rs_deny_13_wildcards_inventory.rs` | `rs_deny_13_wildcards_inventory_tests/` | generated wildcards baseline non-hit path, warnings for missing or project-specific `wildcards` values, and local-root ownership replacement for wildcards drift | parity against canonical baseline |
| `RS-DENY-14` | `rs_deny_14_license_allow_baseline.rs` | `rs_deny_14_license_allow_baseline_tests/` | generated license baseline non-hit path, missing-license errors, missing `[licenses]`, exact `private.ignore` enforcement, and exact allow-list parity | broad mutation across all license baseline knobs; false-positive controls for supersets, exact matches, and unrelated sections |
| `RS-DENY-15` | `rs_deny_15_confidence_threshold.rs` | `rs_deny_15_confidence_threshold_tests/` | generated confidence-threshold baseline non-hit path, weaker-value warnings, stricter-value inventory, missing/invalid-value warnings, and local-root ownership replacement for threshold drift | parity against canonical baseline; integer-value boundary coverage |
| `RS-DENY-16` | `rs_deny_16_copyleft_allowlist.rs` | `rs_deny_16_copyleft_allowlist_tests/` | generated allow-list non-hit path, one warning per added copyleft license, and local-root ownership replacement for copyleft drift | false-positive controls for approved licenses; parity against canonical baseline |
| `RS-DENY-17` | `rs_deny_17_license_exceptions_inventory.rs` | `rs_deny_17_license_exceptions_inventory_tests/` | generated no-exception non-hit path, one inventory item per named or crate-keyed exception entry, and local-root ownership replacement for license exceptions | broader mutation across malformed or unnamed entries; unknown-key interplay with `RS-DENY-28`; parity against canonical baseline |
| `RS-DENY-18` | `rs_deny_18_unknown_sources_policy.rs` | `rs_deny_18_unknown_sources_policy_tests/` | generated unknown-source baseline non-hit path, missing `[sources]`, independent errors for weakened `unknown-registry` / `unknown-git`, and exact source-policy parity | broader interplay with `RS-DENY-19` and `RS-DENY-20`; source-policy mutation across more mixed cases |
| `RS-DENY-19` | `rs_deny_19_allow_registry_baseline.rs` | `rs_deny_19_allow_registry_baseline_tests/` | both accepted crates.io forms, missing `[sources]`, missing crates.io, unexpected extra registries, local-root ownership replacement for registry drift, and exact generated registry-list parity | mixed registry-list order cases; broader profile coverage beyond the current local library-root case |
| `RS-DENY-20` | `rs_deny_20_allow_git_inventory.rs` | `rs_deny_20_allow_git_inventory_tests/` | empty `allow-git` non-hit path, one warning per non-empty config, one inventory item per git source, and exact empty-baseline parity | broader mutation across mixed git-entry shapes; interplay with source baseline and unknown-git policy |
| `RS-DENY-21` | `rs_deny_21_tokio_full_ban.rs` | `rs_deny_21_tokio_full_ban_tests/` | generated tokio feature-policy non-hit path, missing `full` denial warning, allow-list drift warning, local-root ownership replacement for tokio-policy drift, and exact generated tokio allow/deny parity | broader mutation across more tokio feature-policy axes |
| `RS-DENY-22` | `rs_deny_22_extra_feature_bans_inventory.rs` | `rs_deny_22_extra_feature_bans_inventory_tests/` | generated tokio-only feature state non-hit path, one inventory item per non-tokio feature-ban entry, local-root ownership replacement for extra feature bans, and exact parity for the single canonical tokio feature-ban entry | broader mutation across multiple non-tokio feature bans and mixed entry shapes |
| `RS-DENY-23` | `rs_deny_23_skip_hygiene.rs` | `rs_deny_23_skip_hygiene_tests/` | malformed entry warnings, missing/non-string reason warnings, valid inventory for plain-string/modern/legacy shapes, fail-closed warning for malformed `[bans].skip` container type, and local-root ownership replacement for skip inventory | broad mutation across multiple skip entry shapes in one suite; duplicate-entry interplay with `RS-DENY-27`; unknown-key interplay with `RS-DENY-28` |
| `RS-DENY-24` | `rs_deny_24_ignore_hygiene.rs` | `rs_deny_24_ignore_hygiene_tests/` | malformed entry warnings, missing/non-string reason warnings, valid inventory for plain-string and table shapes, fail-closed warning for malformed `[advisories].ignore` container type, and local-root ownership replacement for advisory-ignore inventory | broad mutation across multiple ignore entry shapes in one suite; duplicate-entry interplay with `RS-DENY-27`; unknown-key interplay with `RS-DENY-28` |
| `RS-DENY-25` | `rs_deny_25_allow_override_channel.rs` | `rs_deny_25_allow_override_channel_tests/` | canonical generated no-allow-list non-hit path, non-empty allow-list error, one override error per allowed banned crate, explicit non-overlapping allow-list coverage, local-root ownership replacement for allow-list drift, and standalone-app routed library-profile coverage | broader mixed-profile attacks beyond the current local service-root case |
| `RS-DENY-26` | `rs_deny_26_ban_reason_inventory.rs` | `rs_deny_26_ban_reason_inventory_tests/` | canonical generated fully-reasoned non-hit path, inventory for missing and whitespace-only reasons, and local-root ownership replacement for missing reasons | broader mutation across mixed named/plain entries and broader profile coverage |
| `RS-DENY-27` | `rs_deny_27_duplicate_entries.rs` | `rs_deny_27_duplicate_entries_tests/` | canonical non-duplicated non-hit path, duplicate warnings for deny/skip/advisory-ignore/feature-ban sections, distinct-near-duplicate non-hit coverage, and same-identity-different-shape duplicate coverage | broader mixed-root/profile duplicate attacks |
| `RS-DENY-28` | `rs_deny_28_unknown_keys.rs` | `rs_deny_28_unknown_keys_tests/` | top-level/core-section unknown keys, explicit `[bans]` and `[sources]` unknown-key coverage, nested skip/ignore/exception/feature-entry unknown keys, and local-root ownership replacement for schema drift | false-positive controls for all supported schema keys; fail-closed behavior for unsupported schema nesting |
| `RS-DENY-29` | `rs_deny_29_ignore_accumulation.rs` | `rs_deny_29_ignore_accumulation_tests/` | threshold boundary at 5 versus 6 ignore entries, including mixed valid entry shapes and malformed entries at threshold breach, plus local-root ownership replacement for oversized ignore lists | false-positive controls below threshold across multiple valid shapes |
| `RS-DENY-30` | `rs_deny_30_wrappers.rs` | `rs_deny_30_wrappers_tests/` | managed non-empty wrapper drift errors, project-specific wrapper additions inventory, local workspace-root ownership replacement for managed wrapper drift, exact canonical wrapper-map parity, and local canonical-baseline non-hit coverage | broader mutation across multiple wrapper-bearing bans |

## Immediate hardening implications

1. The next concrete code step is now to keep this matrix current while finishing the remaining semantic gaps.

2. The highest remaining structural gaps are no longer structural conversion work.
   Both clippy and deny now use rule-specific `*_tests/` directories for every rule.

3. The highest-value semantic gaps after the recent migrations are:
   - broader parity framing for already-migrated rules
   - multi-root rework for any remaining root/profile-sensitive clippy or deny checks that still rely only on helper-built trees
   - deeper mixed-profile and mixed-root deny attacks

4. No old corpus item should be ported mechanically.
   Each legacy test or fixture should still be re-expressed as one modern attack vector with exact owned hits, exact owned non-hits, and exact severity.
