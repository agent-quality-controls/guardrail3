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
            "--family",
            "astro",
            "--family",
            "arch",
            "--family",
            "apparch",
            "--family",
            "tsconfig",
            "--family",
            "package",
            "--family",
            "npmrc",
            "--family",
            "jscpd",
            "--family",
            "typecov",
            "--inventory",
        ],
        ".",
        &[
            "eslint", "astro", "arch", "apparch", "tsconfig", "package", "npmrc", "jscpd",
            "typecov",
        ],
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

#[test]
fn parse_command_rejects_rust_family_name() {
    let error =
        super::super::parse_command_from(["g3ts", "validate", "--path", ".", "--family", "clippy"])
            .expect_err("rust family should fail CLI parsing");

    guardrail3_ts_assertions::cli::assert_parse_error_mentions(&error, "clippy");
}

#[test]
fn parse_command_accepts_validate_without_family_flag() {
    guardrail3_ts_assertions::cli::assert_validate_command_from(
        ["g3ts", "validate", "--path", "."],
        ".",
        &[],
        false,
    );
}

#[test]
fn parse_command_accepts_astro_family() {
    guardrail3_ts_assertions::cli::assert_validate_command_from(
        ["g3ts", "validate", "--path", ".", "--family", "astro"],
        ".",
        &["astro"],
        false,
    );
}

#[test]
fn parse_command_accepts_spelling_family() {
    guardrail3_ts_assertions::cli::assert_validate_command_from(
        ["g3ts", "validate", "--path", ".", "--family", "spelling"],
        ".",
        &["spelling"],
        false,
    );
}

#[test]
fn parse_command_accepts_typecov_family() {
    guardrail3_ts_assertions::cli::assert_validate_command_from(
        ["g3ts", "validate", "--path", ".", "--family", "typecov"],
        ".",
        &["typecov"],
        false,
    );
}
