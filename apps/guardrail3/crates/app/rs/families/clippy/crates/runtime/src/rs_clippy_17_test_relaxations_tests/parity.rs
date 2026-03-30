use guardrail3_app_rs_family_clippy_assertions::rs_clippy_17_test_relaxations as assertions;
use test_support::build_fixture_clippy_toml;

#[test]
fn generated_service_baseline_keeps_test_relaxation_policy_exact() {
    let parsed =
        toml::from_str::<toml::Value>(&build_fixture_clippy_toml("service", false, true, "", ""))
            .expect("valid clippy TOML");
    assertions::assert_service_relaxations_exact(&parsed);
}
