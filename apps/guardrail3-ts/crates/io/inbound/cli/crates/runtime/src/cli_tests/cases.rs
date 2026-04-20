#[test]
fn parse_command_accepts_family_and_inventory_flags() {
    guardrail3_ts_assertions::cli::assert_validate_command_from(
        [
            "g3ts",
            "validate",
            "--path",
            ".",
            "--family",
            "eslint",
            "--inventory",
        ],
        ".",
        &["eslint"],
        true,
    );
}

#[test]
fn parse_command_rejects_unknown_family() {
    let error = super::super::parse_command_from([
        "g3ts", "validate", "--path", ".", "--family", "hexarch",
    ])
    .expect_err("unknown family should fail CLI parsing");

    guardrail3_ts_assertions::cli::assert_parse_error_mentions(&error, "hexarch");
}
