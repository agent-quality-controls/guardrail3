use super::helpers::check_source;
use g3rs_code_source_checks_assertions::rs_code_31_public_struct_named_fields::assert_rule_results;

#[test]
fn skips_private_fields_tuple_structs_and_private_modules() {
    assert_rule_results(
        &check_source(
            "src/lib.rs",
            "pub struct User { id: String, email: String }",
        ),
        &[],
    );
    assert_rule_results(
        &check_source("src/lib.rs", "pub struct UserId(pub String);"),
        &[],
    );
    assert_rule_results(
        &check_source(
            "src/lib.rs",
            "mod internal { pub struct User { pub id: String } }",
        ),
        &[],
    );
}
