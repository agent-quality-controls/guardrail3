use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::dependency_policy::rs_hexarch_13_dependency_direction as assertions;

#[test]
fn cross_app_normal_edge_is_owned_by_rule_24_not_rule_13() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbackend-domain-types = { path = \"../types\" }\nworker-app-processor = { path = \"../../../../worker/crates/app/processor\" }\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
    assertions::assert_error_count(&results, "RS-HEXARCH-24", 1);
}
