use super::helpers::check_source;
use g3rs_code_source_checks_assertions::rs_code_33_public_weak_error_forms::assert_rule_results;

#[test]
fn skips_typed_errors_and_private_reachability() {
    assert_rule_results(
        &check_source(
            "src/lib.rs",
            "pub fn parse() -> Result<(), ParseError> { Ok(()) }",
            false,
        ),
        &[],
    );
    assert_rule_results(
        &check_source(
            "src/lib.rs",
            "fn parse() -> Result<(), String> { Ok(()) }",
            false,
        ),
        &[],
    );
    assert_rule_results(
        &check_source(
            "src/lib.rs",
            "mod internal { pub fn parse() -> Result<(), anyhow::Error> { Ok(()) } }",
            false,
        ),
        &[],
    );
}
