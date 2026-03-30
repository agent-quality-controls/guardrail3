use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_25_public_result_error_type::assert_no_hits;
use test_support::write_file;

#[test]
fn remains_silent_even_when_rs_code_33_cases_exist() {
    let fixture = copy_fixture();
    let root = fixture.path();
    write_file(
        root,
        "apps/worker/crates/domain/jobs/src/lib.rs",
        "pub fn parse() -> Result<(), String> { Ok(()) }\npub fn label() -> Result<(), &str> { Ok(()) }\n",
    );

    assert_no_hits(&run_family(root));
}
