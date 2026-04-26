#[test]
fn parse_command_accepts_family_and_inventory_flags() {
    guardrail3_rs_assertions::cli::assert_validate_command_from(
        [
            "guardrail3-rs",
            "validate",
            "--path",
            ".",
            "--family",
            "fmt",
            "--inventory",
        ],
        ".",
        &["fmt"],
        true,
    );
}

#[test]
fn parse_command_rejects_removed_hexarch_family() {
    let error = super::super::parse_command_from([
        "guardrail3-rs",
        "validate",
        "--path",
        ".",
        "--family",
        "hexarch",
    ])
    .expect_err("removed family should fail CLI parsing");

    guardrail3_rs_assertions::cli::assert_parse_error_mentions(&error, "hexarch");
}
