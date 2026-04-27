use g3rs_garde_source_checks_assertions::manual_deserialize_impl::rule as assertions;

#[test]
fn stays_quiet_when_validate_impl_exists_in_another_file() {
    let fixture = super::helpers::fixture(vec![super::helpers::target(
        "src/input.rs",
        6,
        "Input",
        true,
        true,
    )]);

    let results = fixture.run();
    assertions::assert_rule_results(&results, &[]);
}
