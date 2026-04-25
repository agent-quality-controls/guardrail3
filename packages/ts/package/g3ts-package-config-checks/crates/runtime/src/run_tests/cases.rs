use g3ts_package_config_checks_assertions::run as assertions;

use super::helpers::{
    fail_open_syncpack_script, fake_only_allow_preinstall, golden_root, local_pg_dependency_allowed,
    local_root_only, missing_root, missing_syncpack_config, missing_syncpack_source_and_bans,
    root_parse_error, weak_root,
};

#[test]
fn missing_root_reports_only_exists_error() {
    let results = super::super::check(&missing_root());

    assertions::assert_exact(
        &results,
        &[assertions::error(
            "TS-PACKAGE-CONFIG-01",
            "root package.json missing",
            "No root `package.json` file was found. Add a root workspace manifest.",
            None,
            false,
        )],
    );
}

#[test]
fn root_parse_error_reports_exists_inventory_and_parse_error() {
    let results = super::super::check(&root_parse_error());

    assertions::assert_exact_ids(&results, &["TS-PACKAGE-CONFIG-01", "TS-PACKAGE-CONFIG-02"]);
    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-PACKAGE-CONFIG-02",
            "root package.json parse error",
            "Failed to parse root `package.json`: synthetic parse failure",
            Some("package.json"),
            false,
        )],
    );
}

#[test]
fn golden_root_reports_inventory_only() {
    let results = super::super::check(&golden_root());

    assertions::assert_exact_ids(
        &results,
        &[
            "TS-PACKAGE-CONFIG-01",
            "TS-PACKAGE-CONFIG-02",
            "TS-PACKAGE-CONFIG-03",
            "TS-PACKAGE-CONFIG-04",
            "TS-PACKAGE-CONFIG-05",
            "TS-PACKAGE-CONFIG-06",
            "TS-PACKAGE-CONFIG-07",
            "TS-PACKAGE-CONFIG-08",
        ],
    );
    assertions::assert_contains(
        &results,
        &[assertions::info(
            "TS-PACKAGE-CONFIG-08",
            "Syncpack bans forbidden package dependencies",
            "`package.json` lists `syncpack`, runs `syncpack lint` fail-closed, and `.syncpackrc` bans forbidden package deps through Syncpack: `axios`.",
            Some("package.json"),
            true,
        )],
    );
}

#[test]
fn weak_root_reports_root_policy_errors() {
    let results = super::super::check(&weak_root());

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "TS-PACKAGE-CONFIG-03",
                "root package.json is publishable",
                "The root package manifest must set `private: true`.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-PACKAGE-CONFIG-04",
                "root packageManager missing or not pinned to pnpm",
                "The root package manifest must set a pinned `packageManager` such as `pnpm@10.32.0`.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-PACKAGE-CONFIG-05",
                "root engines are incomplete",
                "The root package manifest must declare engines.pnpm.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-PACKAGE-CONFIG-06",
                "root package scripts are incomplete",
                "The root package manifest script baseline is broken: scripts.preinstall must run `only-allow pnpm` in a supported fail-closed command position; scripts.prepare is missing; scripts.lint is missing.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-PACKAGE-CONFIG-07",
                "root pnpm policy is incomplete",
                "The root package manifest must declare pnpm.overrides and pnpm.onlyBuiltDependencies.",
                Some("package.json"),
                false,
            ),
        ],
    );
}

#[test]
fn fake_only_allow_text_does_not_satisfy_preinstall_guard() {
    let results = super::super::check(&fake_only_allow_preinstall());

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-PACKAGE-CONFIG-06",
            "root package scripts are incomplete",
            "The root package manifest script baseline is broken: scripts.preinstall must run `only-allow pnpm` in a supported fail-closed command position.",
            Some("package.json"),
            false,
        )],
    );
}

#[test]
fn syncpack_fail_open_script_does_not_satisfy_dependency_policy() {
    let results = super::super::check(&fail_open_syncpack_script());

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-PACKAGE-CONFIG-08",
            "Syncpack package policy validator is not installed and wired",
            "`package.json` invokes `syncpack lint`, but not in a supported fail-closed root script position. Remove fail-open `||` chains and unsupported shell syntax so Syncpack failure cannot be hidden.",
            Some("package.json"),
            false,
        )],
    );
}

#[test]
fn missing_syncpack_config_reports_validator_contract_error() {
    let results = super::super::check(&missing_syncpack_config());

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-PACKAGE-CONFIG-08",
            "Syncpack does not ban forbidden package dependencies",
            "`.syncpackrc` is missing, so the package family cannot prove Syncpack bans forbidden package deps for `package.json`. Add a parseable `.syncpackrc` with exact `source` entries and canonical `isBanned: true` versionGroups for `axios`.",
            Some(".syncpackrc"),
            false,
        )],
    );
}

#[test]
fn missing_syncpack_source_and_bans_report_validator_contract_errors() {
    let results = super::super::check(&missing_syncpack_source_and_bans());

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "TS-PACKAGE-CONFIG-08",
                "Syncpack does not cover package manifests",
                "`.syncpackrc` is missing exact `source` entries for package manifests: `apps/web/package.json`. Add the listed paths so `syncpack lint` can reject forbidden dependencies everywhere the package family applies.",
                Some(".syncpackrc"),
                false,
            ),
            assertions::error(
                "TS-PACKAGE-CONFIG-08",
                "Syncpack does not ban forbidden package dependencies",
                "`.syncpackrc` is missing Syncpack banned versionGroups for: `axios`. Add one canonical banned versionGroup per listed dependency before any repo-specific groups, with exact `dependencies`, `dependencyTypes: [\"prod\", \"dev\", \"optional\", \"peer\"]`, `isBanned: true`, and no `packages` or `specifierTypes`.",
                Some(".syncpackrc"),
                false,
            ),
        ],
    );
}

#[test]
fn local_only_root_skips_workspace_root_rules() {
    let results = super::super::check(&local_root_only());

    assertions::assert_exact(
        &results,
        &[assertions::info(
            "TS-PACKAGE-CONFIG-08",
            "Syncpack dependency policy is not required",
            "No pnpm package-manager root was detected, so the package-family Syncpack dependency policy is not applied.",
            Some("package.json"),
            true,
        )],
    );
}

#[test]
fn pg_dependency_is_not_treated_as_generic_banned_manifest_policy() {
    let results = super::super::check(&local_pg_dependency_allowed());

    assertions::assert_exact_ids(
        &results,
        &[
            "TS-PACKAGE-CONFIG-01",
            "TS-PACKAGE-CONFIG-02",
            "TS-PACKAGE-CONFIG-03",
            "TS-PACKAGE-CONFIG-04",
            "TS-PACKAGE-CONFIG-05",
            "TS-PACKAGE-CONFIG-06",
            "TS-PACKAGE-CONFIG-07",
            "TS-PACKAGE-CONFIG-08",
        ],
    );
    assertions::assert_contains(
        &results,
        &[assertions::info(
            "TS-PACKAGE-CONFIG-08",
            "Syncpack bans forbidden package dependencies",
            "`package.json` lists `syncpack`, runs `syncpack lint` fail-closed, and `.syncpackrc` bans forbidden package deps through Syncpack: `axios`.",
            Some("package.json"),
            true,
        )],
    );
}
