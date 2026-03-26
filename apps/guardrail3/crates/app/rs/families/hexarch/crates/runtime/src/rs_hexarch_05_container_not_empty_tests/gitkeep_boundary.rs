use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_05_container_not_empty as assertions;
use test_support::{copy_fixture, empty_dir, write_file};

#[test]
fn gitkeep_plus_files_does_not_trigger_rule_05() {
    let tmp = copy_fixture();
    let container = "apps/devctl/crates/domain";
    empty_dir(tmp.path(), container);
    write_file(tmp.path(), &format!("{container}/.gitkeep"), "");
    write_file(tmp.path(), &format!("{container}/README.md"), "# stray");

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-05");
    assert!(
        errors.is_empty(),
        ".gitkeep should suppress rule 05 even when files are present: {errors:#?}"
    );
}
