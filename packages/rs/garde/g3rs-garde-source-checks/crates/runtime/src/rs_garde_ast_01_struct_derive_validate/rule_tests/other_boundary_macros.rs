use g3rs_garde_source_checks_assertions::rs_garde_ast_01_struct_derive_validate::rule as assertions;

#[test]
fn errors_when_parser_boundary_is_missing_validate() {
    let fixture = super::helpers::fixture(
        &[(
            "src/input.rs",
            "use clap::Parser;\n\n#[derive(Parser)]\nstruct Cli {\n    config_path: String,\n}\n",
        )],
        super::helpers::default_guardrail_toml(),
    );

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
    let fixture = super::helpers::fixture(
        &[(
            "src/input.rs",
            "use clap::Parser as CliParser;\n\n#[derive(CliParser)]\nstruct Cli {\n    config_path: String,\n}\n",
        )],
        super::helpers::default_guardrail_toml(),
    );

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
    let fixture = super::helpers::fixture(
        &[(
            "src/input.rs",
            "use clap::Args;\n\n#[derive(Args)]\nstruct InputArgs {\n    pattern: String,\n}\n",
        )],
        super::helpers::default_guardrail_toml(),
    );

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
    let fixture = super::helpers::fixture(
        &[(
            "src/input.rs",
            "use clap::Args as CommandArgs;\n\n#[derive(CommandArgs)]\nstruct InputArgs {\n    pattern: String,\n}\n",
        )],
        super::helpers::default_guardrail_toml(),
    );

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
    let fixture = super::helpers::fixture(
        &[(
            "src/input.rs",
            "use sqlx::FromRow;\n\n#[derive(FromRow)]\nstruct UserRecord {\n    name: String,\n}\n",
        )],
        super::helpers::default_guardrail_toml(),
    );

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
    let fixture = super::helpers::fixture(
        &[(
            "src/input.rs",
            "use sqlx::FromRow as DbRow;\n\n#[derive(DbRow)]\nstruct UserRecord {\n    name: String,\n}\n",
        )],
        super::helpers::default_guardrail_toml(),
    );

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
