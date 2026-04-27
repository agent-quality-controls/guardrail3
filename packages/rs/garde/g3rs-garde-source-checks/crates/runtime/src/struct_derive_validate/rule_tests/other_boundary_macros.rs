use g3rs_garde_source_checks_assertions::struct_derive_validate::rule as assertions;

#[test]
fn errors_when_parser_boundary_is_missing_validate() {
    let fixture = super::helpers::fixture(vec![super::helpers::target(
        "src/input.rs",
        4,
        "Cli",
        &["Parser"],
        false,
    )]);

    let results = fixture.run();
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Error),
            file: Some("src/input.rs"),
            title: Some("struct `Cli` missing Validate derive"),
            message_contains: Some("Parser"),
            ..Default::default()
        }],
    );
}

#[test]
fn errors_when_aliased_parser_boundary_is_missing_validate() {
    let fixture = super::helpers::fixture(vec![super::helpers::target(
        "src/input.rs",
        4,
        "Cli",
        &["CliParser"],
        false,
    )]);

    let results = fixture.run();
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Error),
            file: Some("src/input.rs"),
            title: Some("struct `Cli` missing Validate derive"),
            message_contains: Some("CliParser"),
            ..Default::default()
        }],
    );
}

#[test]
fn errors_when_args_boundary_is_missing_validate() {
    let fixture = super::helpers::fixture(vec![super::helpers::target(
        "src/input.rs",
        4,
        "InputArgs",
        &["Args"],
        false,
    )]);

    let results = fixture.run();
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Error),
            file: Some("src/input.rs"),
            title: Some("struct `InputArgs` missing Validate derive"),
            message_contains: Some("Args"),
            ..Default::default()
        }],
    );
}

#[test]
fn errors_when_aliased_args_boundary_is_missing_validate() {
    let fixture = super::helpers::fixture(vec![super::helpers::target(
        "src/input.rs",
        4,
        "InputArgs",
        &["CommandArgs"],
        false,
    )]);

    let results = fixture.run();
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Error),
            file: Some("src/input.rs"),
            title: Some("struct `InputArgs` missing Validate derive"),
            message_contains: Some("CommandArgs"),
            ..Default::default()
        }],
    );
}

#[test]
fn errors_when_from_row_boundary_is_missing_validate() {
    let fixture = super::helpers::fixture(vec![super::helpers::target(
        "src/input.rs",
        4,
        "UserRecord",
        &["FromRow"],
        false,
    )]);

    let results = fixture.run();
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Error),
            file: Some("src/input.rs"),
            title: Some("struct `UserRecord` missing Validate derive"),
            message_contains: Some("FromRow"),
            ..Default::default()
        }],
    );
}

#[test]
fn errors_when_aliased_from_row_boundary_is_missing_validate() {
    let fixture = super::helpers::fixture(vec![super::helpers::target(
        "src/input.rs",
        4,
        "UserRecord",
        &["DbRow"],
        false,
    )]);

    let results = fixture.run();
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Error),
            file: Some("src/input.rs"),
            title: Some("struct `UserRecord` missing Validate derive"),
            message_contains: Some("DbRow"),
            ..Default::default()
        }],
    );
}
