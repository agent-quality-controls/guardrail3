use g3rs_garde_source_checks_assertions::rs_garde_ast_01_struct_derive_validate::rule as assertions;

#[test]
fn stays_quiet_for_primitive_only_boundary_without_validate() {
    let fixture = super::helpers::fixture(Vec::new());

    let results = fixture.run();
    assertions::assert_rule_results(&results, &[]);
}
