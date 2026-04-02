use super::{copy_fixture, empty_dir, write_file};
use guardrail3_app_rs_family_hexarch_assertions::structure::rs_hexarch_05_container_not_empty as assertions;

#[test]
fn gitkeep_plus_files_does_not_trigger_rule_05() {
    let tmp = copy_fixture();
    let container = "apps/devctl/crates/domain";
    empty_dir(tmp.path(), container);
    write_file(tmp.path(), &format!("{container}/.gitkeep"), "");
    write_file(tmp.path(), &format!("{container}/README.md"), "# stray");

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}
