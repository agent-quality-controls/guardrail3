use g3ts_jscpd_config_checks_assertions::run as assertions;

use super::helpers::{
    extra_inventory_key, golden_root, missing_absolute, missing_ignores, missing_root,
    missing_typescript_format, root_parse_error, weak_threshold,
};

#[test]
fn missing_root_reports_only_exists_error() {
    let results = super::super::check(&missing_root());

    assertions::assert_exact(
        &results,
        &[assertions::error(
            "TS-JSCPD-CONFIG-01",
            "root .jscpd.json missing",
            "No root `.jscpd.json` file was found. Add a root duplication-policy config.",
            None,
            false,
        )],
    );
}

#[test]
fn parse_error_reports_exists_inventory_and_parse_error() {
    let results = super::super::check(&root_parse_error());

    assertions::assert_exact(
        &results,
        &[
            assertions::info(
                "TS-JSCPD-CONFIG-01",
                "root .jscpd.json exists",
                "Found root JSCpd config `.jscpd.json`.",
                Some(".jscpd.json"),
                true,
            ),
            assertions::error(
                "TS-JSCPD-CONFIG-02",
                "root .jscpd.json parse error",
                "Failed to parse root `.jscpd.json`: synthetic parse failure",
                Some(".jscpd.json"),
                false,
            ),
        ],
    );
}

#[test]
fn golden_root_reports_expected_inventory() {
    let results = super::super::check(&golden_root());

    assertions::assert_exact(
        &results,
        &[
            assertions::info(
                "TS-JSCPD-CONFIG-01",
                "root .jscpd.json exists",
                "Found root JSCpd config `.jscpd.json`.",
                Some(".jscpd.json"),
                true,
            ),
            assertions::info(
                "TS-JSCPD-CONFIG-02",
                "root .jscpd.json parseable",
                "`.jscpd.json` parsed successfully as jscpd JSON.",
                Some(".jscpd.json"),
                true,
            ),
            assertions::info(
                "TS-JSCPD-CONFIG-03",
                "jscpd threshold set to zero",
                "The root `.jscpd.json` enforces zero duplication tolerance with `threshold: 0`.",
                Some(".jscpd.json"),
                true,
            ),
            assertions::info(
                "TS-JSCPD-CONFIG-04",
                "jscpd absolute paths enabled",
                "The root `.jscpd.json` sets `absolute: true`.",
                Some(".jscpd.json"),
                true,
            ),
            assertions::info(
                "TS-JSCPD-CONFIG-05",
                "jscpd required ignore patterns present",
                "The root `.jscpd.json` includes the required ignore-pattern baseline.",
                Some(".jscpd.json"),
                true,
            ),
            assertions::info(
                "TS-JSCPD-CONFIG-06",
                "jscpd format includes typescript",
                "The root `.jscpd.json` format list includes `typescript`.",
                Some(".jscpd.json"),
                true,
            ),
        ],
    );
}

#[test]
fn weak_threshold_reports_threshold_error() {
    let results = super::super::check(&weak_threshold());

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-JSCPD-CONFIG-03",
            "jscpd threshold is not zero",
            "Root `.jscpd.json` sets `threshold` to `1`, but the current baseline requires `0`.",
            Some(".jscpd.json"),
            false,
        )],
    );
}

#[test]
fn missing_absolute_reports_absolute_error() {
    let results = super::super::check(&missing_absolute());

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-JSCPD-CONFIG-04",
            "jscpd absolute field missing",
            "Root `.jscpd.json` must set `absolute: true`.",
            Some(".jscpd.json"),
            false,
        )],
    );
}

#[test]
fn missing_ignores_report_ignore_error() {
    let results = super::super::check(&missing_ignores());

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-JSCPD-CONFIG-05",
            "jscpd required ignore patterns missing",
            "Root `.jscpd.json` is missing required ignore patterns: **/.next/**, **/dist/**, **/target/**, **/components/ui/**.",
            Some(".jscpd.json"),
            false,
        )],
    );
}

#[test]
fn missing_typescript_format_reports_format_error() {
    let results = super::super::check(&missing_typescript_format());

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-JSCPD-CONFIG-06",
            "jscpd format misses typescript",
            "Root `.jscpd.json` must include `typescript` in `format`.",
            Some(".jscpd.json"),
            false,
        )],
    );
}

#[test]
fn extra_inventory_key_reports_inventory_finding() {
    let results = super::super::check(&extra_inventory_key());

    assertions::assert_contains(
        &results,
        &[assertions::info(
            "TS-JSCPD-CONFIG-06",
            "jscpd extra top-level key present",
            "Extra root `.jscpd.json` key `gitignore` is outside the current wave-1 baseline. Keep it only if intentional.",
            Some(".jscpd.json"),
            true,
        )],
    );
}
