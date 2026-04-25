use g3ts_astro_config_checks_assertions::run as assertions;

use super::helpers::{
    astro_check_wrapper_forms, astro_lane_missing_pipeline_effectiveness,
    endpoint_only_pipeline_scope_options, endpoint_only_pipeline_scope_without_route_coverage,
    fake_astro_check_text_only, fake_syncpack_lint_text_only, golden,
    local_syncpack_package_source_covers_nested_app, malformed_syncpack_config,
    missing_astro_check, missing_astro_plugin_wiring, missing_content_data_module_scope_options,
    missing_content_source_scope_options, missing_inline_public_content_rule,
    missing_package_eslint_and_astro_config_surfaces, missing_pipeline_rule_enforcement,
    missing_pipeline_scope_options, missing_pipeline_wiring, missing_required_packages,
    missing_syncpack_config, missing_syncpack_lint_script, missing_syncpack_package,
    missing_syncpack_package_with_unsafe_script,
    optional_contracts_not_required, root_syncpack_exact_source_covers_nested_app,
    root_syncpack_package_source_does_not_cover_nested_app, route_only_pipeline_wiring,
    syncpack_catch_all_forbidden_ban, syncpack_ignored_forbidden_ban,
    syncpack_lint_or_chain_fail_open, syncpack_lint_wrapper_forms, syncpack_missing_forbidden_ban,
    syncpack_missing_stack_pin, syncpack_pinned_forbidden_ban, syncpack_scoped_away_forbidden_ban,
    syncpack_scoped_away_stack_pin, syncpack_shadowed_forbidden_ban, syncpack_shadowed_stack_pin,
    syncpack_source_excludes_package, syncpack_specifier_scoped_forbidden_ban,
    syncpack_specifier_scoped_stack_pin, syncpack_wrong_astro_pipeline_stack_pin,
    syncpack_wrong_forbidden_ban_dependency_types, syncpack_wrong_stack_pin,
    ts_lane_missing_pipeline_effectiveness,
    tsx_lane_missing_pipeline_effectiveness, unreadable_syncpack_config,
    velite_package_with_syncpack_ban,
};

const PIPELINE_CONTENT_INFO_TITLE: &str = "Astro content ESLint plugins are wired and effective";
const PIPELINE_CONTENT_ERROR_TITLE: &str =
    "Astro ESLint lanes are not enforcing the required content rules";
const PIPELINE_CONTENT_INFO_MJS: &str = "`eslint.config.mjs` activates `astro-pipeline` and `i18next` and enforces the required Astro pipeline rules plus `i18next/no-literal-string` at error severity on the Astro, TS, and TSX source lanes. The route-scoped rules have route coverage for Astro page routes and endpoint coverage for Astro endpoints; the content-data rule has non-empty `contentDataModuleGlobs`; the authored-content rules have non-empty `authoredContentGlobs` or `specContentGlobs`; and `i18next/no-literal-string` uses `mode: \"all\"`, `framework: \"react\"`, `should-validate-template: true`, and an Astro-content message without broad `words`, `jsx-components`, `callees`, `object-properties`, or `jsx-attributes` allowlists that would hide authored copy.";
const PIPELINE_CONTENT_ERROR_MJS: &str = "`eslint.config.mjs` does not activate `astro-pipeline` and `i18next` with all required Astro content-pipeline rules at error severity on the Astro, TS, and TSX source lanes. Enable `astro-pipeline` with the required Astro pipeline rules, route coverage for Astro page routes, endpoint coverage for Astro endpoints, non-empty `contentDataModuleGlobs` on `astro-pipeline/no-content-data-modules-in-routes`, and non-empty `authoredContentGlobs` or `specContentGlobs` on `astro-pipeline/no-authored-content-fs-read`, `astro-pipeline/no-authored-content-glob`, and `astro-pipeline/no-authored-content-imports`. Also enable `i18next` and `i18next/no-literal-string` with `mode: \"all\"`, `framework: \"react\"`, `should-validate-template: true`, and an Astro-content message; do not add broad `words`, `jsx-components`, `callees`, `object-properties`, or `jsx-attributes` allowlists that hide authored copy. Without this delegated literal-string rule, agents can hardcode public landing copy in routes, UI components, or source data objects while the Astro pipeline checks still pass.";
const PIPELINE_CONTENT_ERROR_GLOB: &str = "`eslint.config.*` does not activate `astro-pipeline` and `i18next` with all required Astro content-pipeline rules at error severity on the Astro, TS, and TSX source lanes. Enable `astro-pipeline` with the required Astro pipeline rules, route coverage for Astro page routes, endpoint coverage for Astro endpoints, non-empty `contentDataModuleGlobs` on `astro-pipeline/no-content-data-modules-in-routes`, and non-empty `authoredContentGlobs` or `specContentGlobs` on `astro-pipeline/no-authored-content-fs-read`, `astro-pipeline/no-authored-content-glob`, and `astro-pipeline/no-authored-content-imports`. Also enable `i18next` and `i18next/no-literal-string` with `mode: \"all\"`, `framework: \"react\"`, `should-validate-template: true`, and an Astro-content message; do not add broad `words`, `jsx-components`, `callees`, `object-properties`, or `jsx-attributes` allowlists that hide authored copy. Without this delegated literal-string rule, agents can hardcode public landing copy in routes, UI components, or source data objects while the Astro pipeline checks still pass.";

#[test]
fn golden_config_reports_expected_inventory() {
    let input = golden();
    let results = super::super::check(&input);
    let pins_message = format!(
        "`.syncpackrc` pins the required Syncpack package policy: {}.",
        crate::support::required_syncpack_pins_message(&input.integration_contracts[0])
    );
    let bans_message = format!(
        "`.syncpackrc` bans forbidden Astro landing deps through Syncpack: {}.",
        crate::support::forbidden_syncpack_deps_message(&input.integration_contracts[0])
    );

    assertions::assert_exact(
        &results,
        &[
            assertions::info(
                "TS-ASTRO-CONFIG-01",
                "astro package present",
                "`package.json` includes `astro`.",
                Some("package.json"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-02",
                "astro check present",
                "`package.json` invokes `astro check` in the app script surface.",
                Some("package.json"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-03",
                "astro ESLint plugin package present",
                "`package.json` includes `eslint-plugin-astro`.",
                Some("package.json"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-05",
                "astro ESLint plugin wired",
                "`eslint.config.mjs` activates `astro` for the required Astro source lanes.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-06",
                "astro pipeline ESLint plugin package present",
                "`package.json` includes `eslint-plugin-astro-pipeline`.",
                Some("package.json"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-07",
                PIPELINE_CONTENT_INFO_TITLE,
                PIPELINE_CONTENT_INFO_MJS,
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-08",
                "Syncpack package policy validator is installed and wired",
                "`package.json` includes `syncpack` and invokes `syncpack lint`.",
                Some("package.json"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-09",
                "Syncpack pins the required Astro stack",
                &pins_message,
                Some(".syncpackrc"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-10",
                "Syncpack bans forbidden Astro landing deps",
                &bans_message,
                Some(".syncpackrc"),
                true,
            ),
        ],
    );
}

#[test]
fn missing_syncpack_package_reports_validator_setup_error() {
    let input = missing_syncpack_package();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-08",
            "Syncpack package policy validator is not installed and wired",
            "`package.json` runs `syncpack lint` but does not list `syncpack` in dependencies or devDependencies. Add `syncpack` so the app uses the repo-pinned validator.",
            Some("package.json"),
            false,
        )],
    );
}

#[test]
fn missing_syncpack_package_with_unsafe_script_reports_both_setup_errors() {
    let input = missing_syncpack_package_with_unsafe_script();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-08",
            "Syncpack package policy validator is not installed and wired",
            "`package.json` invokes `syncpack lint`, but not in a supported fail-closed app script position, and does not list `syncpack` in dependencies or devDependencies. Add `syncpack` and remove fail-open `||` chains and unsupported shell syntax so Syncpack failure cannot be hidden.",
            Some("package.json"),
            false,
        )],
    );
}

#[test]
fn missing_syncpack_lint_script_reports_validator_setup_error() {
    let input = missing_syncpack_lint_script();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-08",
            "Syncpack package policy validator is not installed and wired",
            "`package.json` lists `syncpack` but does not run `syncpack lint` in any app script. Add `syncpack lint` to the script surface so the package policy validator actually runs.",
            Some("package.json"),
            false,
        )],
    );
}

#[test]
fn fake_syncpack_lint_text_does_not_satisfy_the_script_contract() {
    let input = fake_syncpack_lint_text_only();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-08",
            "Syncpack package policy validator is not installed and wired",
            "`package.json` lists `syncpack` but does not run `syncpack lint` in any app script. Add `syncpack lint` to the script surface so the package policy validator actually runs.",
            Some("package.json"),
            false,
        )],
    );
}

#[test]
fn fail_open_syncpack_lint_or_chain_does_not_satisfy_the_script_contract() {
    let input = syncpack_lint_or_chain_fail_open();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-08",
            "Syncpack package policy validator is not installed and wired",
            "`package.json` invokes `syncpack lint`, but not in a supported fail-closed app script position. Remove fail-open `||` chains and unsupported shell syntax so Syncpack failure cannot be hidden.",
            Some("package.json"),
            false,
        )],
    );
}

#[test]
fn syncpack_lint_wrapper_forms_satisfy_the_script_contract() {
    let input = syncpack_lint_wrapper_forms();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::info(
            "TS-ASTRO-CONFIG-08",
            "Syncpack package policy validator is installed and wired",
            "`package.json` includes `syncpack` and invokes `syncpack lint`.",
            Some("package.json"),
            true,
        )],
    );
}

#[test]
fn missing_syncpack_config_reports_stack_pin_and_ban_errors() {
    let input = missing_syncpack_config();
    let results = super::super::check(&input);
    let pins_message = format!(
        "`.syncpackrc` is missing, so the Astro family cannot prove Syncpack pins the required Astro stack for `package.json`. Add a parseable `.syncpackrc` with canonical pinned `versionGroups` for: {}.",
        crate::support::required_syncpack_pins_message(&input.integration_contracts[0])
    );
    let bans_message = format!(
        "`.syncpackrc` is missing, so the Astro family cannot prove Syncpack bans forbidden Astro deps for `package.json`. Add a parseable `.syncpackrc` with canonical `isBanned: true` versionGroups for {}.",
        crate::support::forbidden_syncpack_deps_message(&input.integration_contracts[0])
    );

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "TS-ASTRO-CONFIG-09",
                "Syncpack does not pin the required Astro stack",
                &pins_message,
                Some(".syncpackrc"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-10",
                "Syncpack does not ban forbidden Astro landing deps",
                &bans_message,
                Some(".syncpackrc"),
                false,
            ),
        ],
    );
}

#[test]
fn unavailable_syncpack_config_reports_unreadable_and_parse_error_reasons() {
    for (input, reason) in [
        (unreadable_syncpack_config(), "permission denied"),
        (
            malformed_syncpack_config(),
            "Syncpack config field `versionGroups` must be an array",
        ),
    ] {
        let results = super::super::check(&input);
        let pins_message = format!(
            "`.syncpackrc` {reason}, so the Astro family cannot prove Syncpack pins the required Astro stack for `package.json`. Add a parseable `.syncpackrc` with canonical pinned `versionGroups` for: {}.",
            crate::support::required_syncpack_pins_message(&input.integration_contracts[0])
        );
        let bans_message = format!(
            "`.syncpackrc` {reason}, so the Astro family cannot prove Syncpack bans forbidden Astro deps for `package.json`. Add a parseable `.syncpackrc` with canonical `isBanned: true` versionGroups for {}.",
            crate::support::forbidden_syncpack_deps_message(&input.integration_contracts[0])
        );

        assertions::assert_contains(
            &results,
            &[
                assertions::error(
                    "TS-ASTRO-CONFIG-09",
                    "Syncpack does not pin the required Astro stack",
                    &pins_message,
                    Some(".syncpackrc"),
                    false,
                ),
                assertions::error(
                    "TS-ASTRO-CONFIG-10",
                    "Syncpack does not ban forbidden Astro landing deps",
                    &bans_message,
                    Some(".syncpackrc"),
                    false,
                ),
            ],
        );
    }
}

#[test]
fn syncpack_source_must_cover_the_app_package_manifest() {
    let input = syncpack_source_excludes_package();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "TS-ASTRO-CONFIG-09",
                "Syncpack does not pin the required Astro stack",
                "`.syncpackrc` does not include exact `source` entry `package.json` for `package.json`, so `syncpack lint` cannot prove package policy for this Astro app.",
                Some(".syncpackrc"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-10",
                "Syncpack does not ban forbidden Astro landing deps",
                "`.syncpackrc` does not include exact `source` entry `package.json` for `package.json`, so `syncpack lint` cannot reject forbidden dependencies for this Astro app.",
                Some(".syncpackrc"),
                false,
            ),
        ],
    );
}

#[test]
fn root_syncpack_source_package_json_does_not_cover_nested_app_manifest() {
    let input = root_syncpack_package_source_does_not_cover_nested_app();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "TS-ASTRO-CONFIG-09",
                "Syncpack does not pin the required Astro stack",
                "`.syncpackrc` does not include exact `source` entry `apps/landing/package.json` for `apps/landing/package.json`, so `syncpack lint` cannot prove package policy for this Astro app.",
                Some(".syncpackrc"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-10",
                "Syncpack does not ban forbidden Astro landing deps",
                "`.syncpackrc` does not include exact `source` entry `apps/landing/package.json` for `apps/landing/package.json`, so `syncpack lint` cannot reject forbidden dependencies for this Astro app.",
                Some(".syncpackrc"),
                false,
            ),
        ],
    );
}

#[test]
fn root_syncpack_source_exact_path_covers_nested_app_manifest() {
    let input = root_syncpack_exact_source_covers_nested_app();
    let results = super::super::check(&input);
    let pins_message = format!(
        "`.syncpackrc` pins the required Syncpack package policy: {}.",
        crate::support::required_syncpack_pins_message(&input.integration_contracts[0])
    );
    let bans_message = format!(
        "`.syncpackrc` bans forbidden Astro landing deps through Syncpack: {}.",
        crate::support::forbidden_syncpack_deps_message(&input.integration_contracts[0])
    );

    assertions::assert_contains(
        &results,
        &[
            assertions::info(
                "TS-ASTRO-CONFIG-09",
                "Syncpack pins the required Astro stack",
                &pins_message,
                Some(".syncpackrc"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-10",
                "Syncpack bans forbidden Astro landing deps",
                &bans_message,
                Some(".syncpackrc"),
                true,
            ),
        ],
    );
}

#[test]
fn app_local_syncpack_source_package_json_covers_nested_app_manifest() {
    let input = local_syncpack_package_source_covers_nested_app();
    let results = super::super::check(&input);
    let pins_message = format!(
        "`apps/landing/.syncpackrc` pins the required Syncpack package policy: {}.",
        crate::support::required_syncpack_pins_message(&input.integration_contracts[0])
    );
    let bans_message = format!(
        "`apps/landing/.syncpackrc` bans forbidden Astro landing deps through Syncpack: {}.",
        crate::support::forbidden_syncpack_deps_message(&input.integration_contracts[0])
    );

    assertions::assert_contains(
        &results,
        &[
            assertions::info(
                "TS-ASTRO-CONFIG-09",
                "Syncpack pins the required Astro stack",
                &pins_message,
                Some("apps/landing/.syncpackrc"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-10",
                "Syncpack bans forbidden Astro landing deps",
                &bans_message,
                Some("apps/landing/.syncpackrc"),
                true,
            ),
        ],
    );
}

#[test]
fn missing_syncpack_stack_pin_reports_policy_error() {
    let input = syncpack_missing_stack_pin();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-09",
            "Syncpack does not pin the required Astro stack",
            "`.syncpackrc` is missing required Syncpack pinned versionGroups: `astro` -> `6.1.9`. Add one canonical versionGroup per listed package before any app-specific groups, with exact `dependencies`, `dependencyTypes: [\"prod\", \"dev\"]`, no `packages`, no `specifierTypes`, and the listed `pinVersion`.",
            Some(".syncpackrc"),
            false,
        )],
    );
}

#[test]
fn wrong_syncpack_stack_pin_reports_policy_error() {
    let input = syncpack_wrong_stack_pin();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-09",
            "Syncpack does not pin the required Astro stack",
            "`.syncpackrc` is missing required Syncpack pinned versionGroups: `astro` -> `6.1.9`. Add one canonical versionGroup per listed package before any app-specific groups, with exact `dependencies`, `dependencyTypes: [\"prod\", \"dev\"]`, no `packages`, no `specifierTypes`, and the listed `pinVersion`.",
            Some(".syncpackrc"),
            false,
        )],
    );
}

#[test]
fn wrong_astro_pipeline_syncpack_stack_pin_reports_policy_error() {
    let input = syncpack_wrong_astro_pipeline_stack_pin();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-09",
            "Syncpack does not pin the required Astro stack",
            "`.syncpackrc` is missing required Syncpack pinned versionGroups: `eslint-plugin-astro-pipeline` -> `0.1.4`. Add one canonical versionGroup per listed package before any app-specific groups, with exact `dependencies`, `dependencyTypes: [\"prod\", \"dev\"]`, no `packages`, no `specifierTypes`, and the listed `pinVersion`.",
            Some(".syncpackrc"),
            false,
        )],
    );
}

#[test]
fn shadowed_syncpack_stack_pin_reports_policy_error() {
    let input = syncpack_shadowed_stack_pin();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-09",
            "Syncpack does not pin the required Astro stack",
            "`.syncpackrc` is missing required Syncpack pinned versionGroups: `astro` -> `6.1.9`. Add one canonical versionGroup per listed package before any app-specific groups, with exact `dependencies`, `dependencyTypes: [\"prod\", \"dev\"]`, no `packages`, no `specifierTypes`, and the listed `pinVersion`.",
            Some(".syncpackrc"),
            false,
        )],
    );
}

#[test]
fn package_scoped_away_syncpack_stack_pin_reports_policy_error() {
    let input = syncpack_scoped_away_stack_pin();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-09",
            "Syncpack does not pin the required Astro stack",
            "`.syncpackrc` is missing required Syncpack pinned versionGroups: `astro` -> `6.1.9`. Add one canonical versionGroup per listed package before any app-specific groups, with exact `dependencies`, `dependencyTypes: [\"prod\", \"dev\"]`, no `packages`, no `specifierTypes`, and the listed `pinVersion`.",
            Some(".syncpackrc"),
            false,
        )],
    );
}

#[test]
fn specifier_scoped_syncpack_stack_pin_reports_policy_error() {
    let input = syncpack_specifier_scoped_stack_pin();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-09",
            "Syncpack does not pin the required Astro stack",
            "`.syncpackrc` is missing required Syncpack pinned versionGroups: `astro` -> `6.1.9`. Add one canonical versionGroup per listed package before any app-specific groups, with exact `dependencies`, `dependencyTypes: [\"prod\", \"dev\"]`, no `packages`, no `specifierTypes`, and the listed `pinVersion`.",
            Some(".syncpackrc"),
            false,
        )],
    );
}

#[test]
fn syncpack_catch_all_forbidden_ban_does_not_satisfy_canonical_contract() {
    let input = syncpack_catch_all_forbidden_ban();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-10",
            "Syncpack does not ban forbidden Astro landing deps",
            "`.syncpackrc` is missing Syncpack banned versionGroups for: `next`, `velite`, `eslint-mdx`, `eslint-plugin-i18next`. Add one canonical banned versionGroup per listed dependency before any app-specific groups, with exact `dependencies`, `dependencyTypes: [\"prod\", \"dev\", \"optional\", \"peer\"]`, `isBanned: true`, and no `packages` or `specifierTypes`.",
            Some(".syncpackrc"),
            false,
        )],
    );
}

#[test]
fn noncanonical_syncpack_forbidden_bans_report_policy_error() {
    for (case_name, input) in [
        (
            "shadowed",
            syncpack_shadowed_forbidden_ban as fn() -> g3ts_astro_types::G3TsAstroConfigChecksInput,
        ),
        ("package scoped", syncpack_scoped_away_forbidden_ban),
        ("specifier scoped", syncpack_specifier_scoped_forbidden_ban),
        (
            "wrong dependency types",
            syncpack_wrong_forbidden_ban_dependency_types,
        ),
        ("ignored", syncpack_ignored_forbidden_ban),
        ("pinned", syncpack_pinned_forbidden_ban),
    ] {
        let results = super::super::check(&input());

        assertions::assert_contains(
            &results,
            &[assertions::error(
                "TS-ASTRO-CONFIG-10",
                "Syncpack does not ban forbidden Astro landing deps",
                "`.syncpackrc` is missing Syncpack banned versionGroups for: `next`. Add one canonical banned versionGroup per listed dependency before any app-specific groups, with exact `dependencies`, `dependencyTypes: [\"prod\", \"dev\", \"optional\", \"peer\"]`, `isBanned: true`, and no `packages` or `specifierTypes`.",
                Some(".syncpackrc"),
                false,
            )],
        );
        assert!(
            results.iter().any(|finding| {
                finding.id() == "TS-ASTRO-CONFIG-10" && finding.message().contains("`next`")
            }),
            "case {case_name} should report `next` as missing: {results:?}"
        );
    }
}

#[test]
fn missing_syncpack_forbidden_ban_reports_policy_error() {
    let input = syncpack_missing_forbidden_ban();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-10",
            "Syncpack does not ban forbidden Astro landing deps",
            "`.syncpackrc` is missing Syncpack banned versionGroups for: `next`. Add one canonical banned versionGroup per listed dependency before any app-specific groups, with exact `dependencies`, `dependencyTypes: [\"prod\", \"dev\", \"optional\", \"peer\"]`, `isBanned: true`, and no `packages` or `specifierTypes`.",
            Some(".syncpackrc"),
            false,
        )],
    );
}

#[test]
fn direct_velite_package_is_not_scanned_when_syncpack_ban_contract_is_valid() {
    let input = velite_package_with_syncpack_ban();
    let results = super::super::check(&input);
    let bans_message = format!(
        "`.syncpackrc` bans forbidden Astro landing deps through Syncpack: {}.",
        crate::support::forbidden_syncpack_deps_message(&input.integration_contracts[0])
    );

    assertions::assert_no_findings_for_id(&results, "TS-ASTRO-CONFIG-04");
    assertions::assert_contains(
        &results,
        &[assertions::info(
            "TS-ASTRO-CONFIG-10",
            "Syncpack bans forbidden Astro landing deps",
            &bans_message,
            Some(".syncpackrc"),
            true,
        )],
    );
}

#[test]
fn missing_astro_check_reports_only_that_error() {
    let input = missing_astro_check();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-02",
            "Astro app scripts do not run `astro check`",
            "`package.json` does not run `astro check` in any app script. Add `astro check` to the script surface in `package.json`. Without that script entry, CI and local validation can pass while Astro type and content errors stay unchecked.",
            Some("package.json"),
            false,
        )],
    );
}

#[test]
fn quoted_astro_check_text_does_not_satisfy_the_script_contract() {
    let input = fake_astro_check_text_only();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-02",
            "Astro app scripts do not run `astro check`",
            "`package.json` does not run `astro check` in any app script. Add `astro check` to the script surface in `package.json`. Without that script entry, CI and local validation can pass while Astro type and content errors stay unchecked.",
            Some("package.json"),
            false,
        )],
    );
}

#[test]
fn wrapper_forms_satisfy_the_astro_check_contract() {
    let input = astro_check_wrapper_forms();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::info(
            "TS-ASTRO-CONFIG-02",
            "astro check present",
            "`package.json` invokes `astro check` in the app script surface.",
            Some("package.json"),
            true,
        )],
    );
}

#[test]
fn missing_required_packages_report_package_contract_errors() {
    let input = missing_required_packages();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "TS-ASTRO-CONFIG-01",
                "Astro app package is missing `astro`",
                "`package.json` does not list `astro` in dependencies or devDependencies. Add `astro` to `package.json`. Without that dependency entry, this app can drift away from the Astro framework contract without the package surface showing it.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-03",
                "Astro app package is missing `eslint-plugin-astro`",
                "`package.json` does not list `eslint-plugin-astro` in dependencies or devDependencies. Add `eslint-plugin-astro` to `package.json`. Astro source files need the Astro ESLint plugin so Astro-specific lint rules can run.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-06",
                "Astro app package is missing `eslint-plugin-astro-pipeline`",
                "`package.json` does not list `eslint-plugin-astro-pipeline` in dependencies or devDependencies. Add `eslint-plugin-astro-pipeline` to `package.json`. Astro source-pipeline rules must come from the shared ESLint plugin so route bypasses fail in lint.",
                Some("package.json"),
                false,
            ),
        ],
    );
}

#[test]
fn missing_pipeline_wiring_reports_wiring_error() {
    let input = missing_astro_plugin_wiring();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-05",
            "Astro ESLint lanes are not wired to the `astro` plugin",
            "`eslint.config.mjs` does not activate `astro` on the required Astro source lanes. Add the `astro` plugin to the Astro, TS, and TSX lane configs in `eslint.config.mjs`. Astro source files must run through the Astro plugin so framework lint rules actually execute.",
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_pipeline_wiring_reports_pipeline_wiring_error() {
    let input = missing_pipeline_wiring();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_pipeline_rule_enforcement_reports_effectiveness_error() {
    let input = missing_pipeline_rule_enforcement();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_pipeline_scope_options_reports_effectiveness_error() {
    let input = missing_pipeline_scope_options();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_content_data_module_scope_options_reports_effectiveness_error() {
    let input = missing_content_data_module_scope_options();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_content_source_scope_options_reports_effectiveness_error() {
    let input = missing_content_source_scope_options();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_inline_public_content_rule_reports_effectiveness_error() {
    let input = missing_inline_public_content_rule();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn route_only_pipeline_wiring_still_fails_the_source_lane_contract() {
    let input = route_only_pipeline_wiring();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn endpoint_only_scope_options_satisfy_pipeline_effectiveness() {
    let input = endpoint_only_pipeline_scope_options();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::info(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_INFO_TITLE,
            PIPELINE_CONTENT_INFO_MJS,
            Some("eslint.config.mjs"),
            true,
        )],
    );
}

#[test]
fn endpoint_only_scope_options_fail_when_route_coverage_is_missing() {
    let input = endpoint_only_pipeline_scope_without_route_coverage();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_pipeline_effectiveness_in_one_lane_still_fails_the_contract() {
    let input = tsx_lane_missing_pipeline_effectiveness();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_pipeline_effectiveness_in_the_astro_lane_still_fails_the_contract() {
    let input = astro_lane_missing_pipeline_effectiveness();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_pipeline_effectiveness_in_the_ts_lane_still_fails_the_contract() {
    let input = ts_lane_missing_pipeline_effectiveness();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            PIPELINE_CONTENT_ERROR_TITLE,
            PIPELINE_CONTENT_ERROR_MJS,
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn optional_contracts_do_not_fire_when_policy_is_disabled() {
    let input = optional_contracts_not_required();
    let results = super::super::check(&input);

    assertions::assert_no_findings_for_id(&results, "TS-ASTRO-CONFIG-06");
    assertions::assert_no_findings_for_id(&results, "TS-ASTRO-CONFIG-07");
}

#[test]
fn missing_package_eslint_and_astro_config_surfaces_fail_closed() {
    let input = missing_package_eslint_and_astro_config_surfaces();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "TS-ASTRO-CONFIG-01",
                "Astro app package is missing `astro`",
                "`package.json` does not list `astro` in dependencies or devDependencies. Add `astro` to `package.json`. Without that dependency entry, this app can drift away from the Astro framework contract without the package surface showing it.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-02",
                "Astro app scripts do not run `astro check`",
                "`package.json` does not run `astro check` in any app script. Add `astro check` to the script surface in `package.json`. Without that script entry, CI and local validation can pass while Astro type and content errors stay unchecked.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-03",
                "Astro app package is missing `eslint-plugin-astro`",
                "`package.json` does not list `eslint-plugin-astro` in dependencies or devDependencies. Add `eslint-plugin-astro` to `package.json`. Astro source files need the Astro ESLint plugin so Astro-specific lint rules can run.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-05",
                "Astro ESLint lanes are not wired to the `astro` plugin",
                "`eslint.config.*` does not activate `astro` on the required Astro source lanes. Add the `astro` plugin to the Astro, TS, and TSX lane configs in `eslint.config.*`. Astro source files must run through the Astro plugin so framework lint rules actually execute.",
                Some("eslint.config.*"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-06",
                "Astro app package is missing `eslint-plugin-astro-pipeline`",
                "`package.json` does not list `eslint-plugin-astro-pipeline` in dependencies or devDependencies. Add `eslint-plugin-astro-pipeline` to `package.json`. Astro source-pipeline rules must come from the shared ESLint plugin so route bypasses fail in lint.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-07",
                PIPELINE_CONTENT_ERROR_TITLE,
                PIPELINE_CONTENT_ERROR_GLOB,
                Some("eslint.config.*"),
                false,
            ),
        ],
    );
}
