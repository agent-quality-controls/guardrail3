use g3_garde_ast_checks_assertions::rs_garde_07_manual_deserialize_impl as assertions;

#[test]
fn stays_quiet_when_validate_impl_exists_in_another_file() {
    let fixture = crate::test_support::fixture(
        &[
            (
                "src/input.rs",
                "use serde::Deserialize;\n\nstruct Input {\n    name: String,\n}\n\nimpl<'de> Deserialize<'de> for Input {\n    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>\n    where\n        D: serde::Deserializer<'de>,\n    {\n        todo!()\n    }\n}\n",
            ),
            (
                "src/validate.rs",
                "use crate::Input;\n\nimpl garde::Validate for Input {\n    type Context = ();\n\n    fn validate_into(&self, _ctx: &Self::Context, _parent: &mut dyn FnMut(garde::Error)) {}\n}\n",
            ),
        ],
        crate::test_support::default_guardrail_toml(),
    );

    let results = fixture.run();
    assertions::assert_rule_results(&results, &[]);
}
