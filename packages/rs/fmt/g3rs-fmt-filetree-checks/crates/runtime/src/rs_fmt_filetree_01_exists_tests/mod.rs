use g3rs_fmt_filetree_checks_assertions::rs_fmt_filetree_01_exists as assertions;

use crate::test_support::input;

#[test]
fn errors_when_no_root_config_exists() {
    let results = crate::check(&input(None, None, Vec::new(), Vec::new()));

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "rustfmt config missing",
            "Expected `rustfmt.toml` at workspace root. Create one with the required formatting settings.",
            "rustfmt.toml",
            false,
        )],
    );
}

#[test]
fn stays_quiet_when_root_rustfmt_toml_exists() {
    let results = crate::check(&input(Some("rustfmt.toml"), None, Vec::new(), Vec::new()));

    assertions::assert_no_findings(&results);
}

#[test]
fn stays_quiet_when_only_root_dot_rustfmt_toml_exists() {
    let results = crate::check(&input(None, Some(".rustfmt.toml"), Vec::new(), Vec::new()));

    assertions::assert_no_findings(&results);
}
