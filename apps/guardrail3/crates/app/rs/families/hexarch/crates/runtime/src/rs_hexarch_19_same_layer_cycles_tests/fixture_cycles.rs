use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_19_same_layer_cycles as assertions;

#[test]
fn fixture_backed_same_layer_cycle_reports_once() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/types/Cargo.toml",
        "[package]\nname = \"backend-domain-types\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nengine_alias = { package = \"backend-domain-engine\", path = \"../engine\" }\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_error_count(&results, "", 1);
    assertions::assert_error_results(&results, "", 1, &["same-layer domain dependency cycle"]);
    assertions::assert_error_file_set(&results, "", 1, &[]);
}
