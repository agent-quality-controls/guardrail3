use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_33_public_weak_error_forms::assert_no_hits;

#[test]
fn skips_typed_errors_and_private_reachability() {
    assert_no_hits(&check_source(
        "src/lib.rs",
        "pub fn parse() -> Result<(), ParseError> { Ok(()) }",
        false,
    ));
    assert_no_hits(&check_source(
        "src/lib.rs",
        "fn parse() -> Result<(), String> { Ok(()) }",
        false,
    ));
    assert_no_hits(&check_source(
        "src/lib.rs",
        "mod internal { pub fn parse() -> Result<(), anyhow::Error> { Ok(()) } }",
        false,
    ));
}
