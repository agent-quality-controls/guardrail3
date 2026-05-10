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
            "g3ts-jscpd/root-exists",
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
                "g3ts-jscpd/root-exists",
                "root .jscpd.json exists",
                "Found root JSCpd config `.jscpd.json`.",
                Some(".jscpd.json"),
                true,
            ),
            assertions::error(
                "g3ts-jscpd/root-parseable",
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
                "g3ts-jscpd/root-exists",
                "root .jscpd.json exists",
                "Found root JSCpd config `.jscpd.json`.",
                Some(".jscpd.json"),
                true,
            ),
            assertions::info(
                "g3ts-jscpd/root-parseable",
                "root .jscpd.json parseable",
                "`.jscpd.json` parsed successfully as jscpd JSON.",
                Some(".jscpd.json"),
                true,
            ),
            assertions::info(
                "g3ts-jscpd/threshold-zero",
                "jscpd threshold set to zero",
                "The root `.jscpd.json` enforces zero duplication tolerance with `threshold: 0`.",
                Some(".jscpd.json"),
                true,
            ),
            assertions::info(
                "g3ts-jscpd/absolute-true",
                "jscpd absolute paths enabled",
                "The root `.jscpd.json` sets `absolute: true`.",
                Some(".jscpd.json"),
                true,
            ),
            assertions::info(
                "g3ts-jscpd/required-ignores",
                "jscpd required ignore patterns present",
                "The root `.jscpd.json` includes the required ignore-pattern baseline.",
                Some(".jscpd.json"),
                true,
            ),
            assertions::info(
                "g3ts-jscpd/format-and-inventory",
                "jscpd format includes typescript",
                "The root `.jscpd.json` format list includes `typescript`.",
                Some(".jscpd.json"),
                true,
            ),
        ],
    );
}

#[test]
fn rule_specific_inputs_emit_their_expected_findings() {
    type Input = g3ts_jscpd_types::G3TsJscpdChecksInput;
    type Build = fn() -> Input;

    struct Case {
        build: Build,
        expected: assertions::Finding<'static>,
    }

    let cases = [
        Case {
            build: weak_threshold,
            expected: assertions::error(
                "g3ts-jscpd/threshold-zero",
                "jscpd threshold is not zero",
                "Root `.jscpd.json` sets `threshold` to `1`, but the current baseline requires `0`.",
                Some(".jscpd.json"),
                false,
            ),
        },
        Case {
            build: missing_absolute,
            expected: assertions::error(
                "g3ts-jscpd/absolute-true",
                "jscpd absolute field missing",
                "Root `.jscpd.json` must set `absolute: true`.",
                Some(".jscpd.json"),
                false,
            ),
        },
        Case {
            build: missing_ignores,
            expected: assertions::error(
                "g3ts-jscpd/required-ignores",
                "jscpd required ignore patterns missing",
                "Root `.jscpd.json` is missing required ignore patterns: **/.next/**, **/dist/**, **/target/**, **/components/ui/**.",
                Some(".jscpd.json"),
                false,
            ),
        },
        Case {
            build: missing_typescript_format,
            expected: assertions::error(
                "g3ts-jscpd/format-and-inventory",
                "jscpd format misses typescript",
                "Root `.jscpd.json` must include `typescript` in `format`.",
                Some(".jscpd.json"),
                false,
            ),
        },
        Case {
            build: extra_inventory_key,
            expected: assertions::info(
                "g3ts-jscpd/format-and-inventory",
                "jscpd extra top-level key present",
                "Extra root `.jscpd.json` key `gitignore` is outside the current wave-1 baseline. Keep it only if intentional.",
                Some(".jscpd.json"),
                true,
            ),
        },
    ];

    for case in &cases {
        let input = (case.build)();
        let results = super::super::check(&input);
        assertions::assert_contains(&results, std::slice::from_ref(&case.expected));
    }
}
