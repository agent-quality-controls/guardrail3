use g3rs_deny_config_checks_assertions::advisories::stricter_advisories_inventory::rule as assertions;

use super::helpers::run_check;

#[test]
fn yanked_deny() {
    let results = run_check(
        r#"
[advisories]
unmaintained = "workspace"
yanked = "deny"
"#,
    );
    assertions::assert_no_findings(&results);
}

#[test]
fn unmaintained_all_is_stricter_than_workspace_baseline() {
    let results = run_check(
        r#"
[advisories]
unmaintained = "all"
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::info(
            "advisories `unmaintained` stricter than baseline",
            "`deny.toml` sets `[advisories].unmaintained = \"all\"`.",
            "deny.toml",
            true,
        )],
    );
}

#[test]
fn unmaintained_transitive_is_not_stricter_than_workspace_baseline() {
    let results = run_check(
        r#"
[advisories]
unmaintained = "transitive"
"#,
    );
    assertions::assert_no_findings(&results);
}

#[test]
fn yanked_allow_is_not_stricter_than_warn_baseline() {
    let results = run_check(
        r#"
[advisories]
yanked = "allow"
"#,
    );
    assertions::assert_no_findings(&results);
}
