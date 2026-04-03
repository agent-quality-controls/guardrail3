use super::helpers::check_source;
use guardrail3_app_rs_family_code_assertions::api_shape::rs_code_31_public_struct_named_fields::assert_no_hits;

#[test]
fn skips_private_fields_tuple_structs_and_private_modules() {
    assert_no_hits(&check_source(
        "src/lib.rs",
        "pub struct User { id: String, email: String }",
    ));
    assert_no_hits(&check_source(
        "src/lib.rs",
        "pub struct UserId(pub String);",
    ));
    assert_no_hits(&check_source(
        "src/lib.rs",
        "mod internal { pub struct User { pub id: String } }",
    ));
}
