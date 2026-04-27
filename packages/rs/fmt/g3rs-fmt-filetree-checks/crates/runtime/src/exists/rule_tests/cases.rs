use g3rs_fmt_filetree_checks_assertions::exists::rule as assertions;
use test_support::input;

#[test]
fn errors_when_no_root_config_exists() {
    let mut results = Vec::new();
    super::super::check(&input(None, None, Vec::new(), Vec::new()), &mut results);

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
    let mut results = Vec::new();
    super::super::check(
        &input(Some("rustfmt.toml"), None, Vec::new(), Vec::new()),
        &mut results,
    );

    assertions::assert_no_findings(&results);
}

#[test]
fn stays_quiet_when_only_root_dot_rustfmt_toml_exists() {
    let mut results = Vec::new();
    super::super::check(
        &input(None, Some(".rustfmt.toml"), Vec::new(), Vec::new()),
        &mut results,
    );

    assertions::assert_no_findings(&results);
}
