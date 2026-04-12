use cargo_toml_parser::parse as parse_cargo;
use g3rs_test_config_checks_assertions::common::assert_has_result;
use guardrail3_check_types::G3Severity;

#[test]
fn reports_missing_mutants_profile() {
    let mut input = crate::test_helpers::input();
    input.mutants_exists = true;

    let results = crate::check(&input);

    assert_has_result(
        &results,
        "RS-TEST-CONFIG-13",
        G3Severity::Error,
        "profile.mutants missing",
        "Cargo.toml",
    );
}

#[test]
fn reports_mutants_profile_as_inventory() {
    let mut input = crate::test_helpers::input();
    input.cargo = parse_cargo(
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[profile.mutants]\ninherits = \"dev\"\n",
    )
    .expect("valid Cargo fixture");
    input.mutants_exists = true;

    let results = crate::check(&input);

    assert_has_result(
        &results,
        "RS-TEST-CONFIG-13",
        G3Severity::Info,
        "profile.mutants configured",
        "Cargo.toml",
    );
}
