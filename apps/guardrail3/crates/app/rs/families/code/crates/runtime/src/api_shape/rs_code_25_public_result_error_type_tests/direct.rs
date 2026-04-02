use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::api_shape::rs_code_25_public_result_error_type::assert_no_hits;

#[test]
fn stays_quiet_for_legacy_weak_public_error_cases() {
    assert_no_hits(&check_source(
        "src/lib.rs",
        "pub fn parse() -> Result<(), String> { Ok(()) }",
        false,
    ));
    assert_no_hits(&check_source(
        "src/lib.rs",
        "pub fn boxed() -> Result<(), Box<dyn std::error::Error>> { Ok(()) }",
        false,
    ));
}
