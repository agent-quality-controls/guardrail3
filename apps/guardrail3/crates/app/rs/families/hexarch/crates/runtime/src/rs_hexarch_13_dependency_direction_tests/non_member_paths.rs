use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_13_dependency_direction as assertions;

#[test]
fn out_of_tree_path_with_layer_like_names_does_not_trigger_direction_rule() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/Cargo.toml",
        "[package]\nname = \"backend-domain-engine\"\nversion = \"0.1.0\"\n[dependencies]\nshared-adapter-kit = { path = \"../../../../packages/adapters/http\" }\n",
    );
    write_file(
        tmp.path(),
        "packages/adapters/http/Cargo.toml",
        "[package]\nname = \"shared-adapter-kit\"\nversion = \"0.1.0\"\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}
