pub(super) fn fixture(source_files: &[(&str, &str)], rust_policy_toml: &str) -> super::super::Fixture {
    super::super::fixture(source_files, rust_policy_toml)
}

pub(super) fn invalid_policy_fixture(source_files: &[(&str, &str)], message: &str) -> super::super::Fixture {
    super::super::invalid_policy_fixture(source_files, message)
}

pub(super) fn default_guardrail_toml() -> &'static str {
    super::super::default_guardrail_toml()
}
