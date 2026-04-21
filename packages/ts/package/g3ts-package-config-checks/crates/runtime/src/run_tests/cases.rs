use g3ts_package_config_checks_assertions::run as assertions;

use super::helpers::{
    golden_root, local_banned_and_parse_error, local_pg_dependency_allowed, local_root_only,
    missing_root, root_parse_error, weak_root,
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
            "local manifests avoid banned dependencies",
            "Checked 1 local package manifests without banned dependency declarations.",
            Some("apps/web/package.json"),
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
                "The root package manifest script baseline is broken: scripts.preinstall must contain `only-allow pnpm`; scripts.prepare is missing; scripts.lint is missing.",
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
fn local_banned_dependencies_and_parse_errors_report_under_local_rule() {
    let results = super::super::check(&local_banned_and_parse_error());

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "TS-PACKAGE-CONFIG-08",
                "local package.json parse blocker",
                "Failed to prove local manifest policy for `apps/landing/package.json`: synthetic parse failure",
                Some("apps/landing/package.json"),
                false,
            ),
            assertions::error(
                "TS-PACKAGE-CONFIG-08",
                "banned dependency declared",
                "Local manifest `apps/web/package.json` declares banned dependency `axios`.",
                Some("apps/web/package.json"),
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
            "local manifests avoid banned dependencies",
            "Checked 1 local package manifests without banned dependency declarations.",
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
            "local manifests avoid banned dependencies",
            "Checked 1 local package manifests without banned dependency declarations.",
            Some("apps/web/package.json"),
            true,
        )],
    );
}
