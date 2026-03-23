use super::super::super::test_support::{
    copy_fixture, empty_dir, errors_by_id, run_family, write_file,
};

#[test]
fn gitkeep_plus_files_does_not_trigger_rule_05() {
    let tmp = copy_fixture();
    let container = "apps/devctl/crates/domain";
    empty_dir(tmp.path(), container);
    write_file(tmp.path(), &format!("{container}/.gitkeep"), "");
    write_file(tmp.path(), &format!("{container}/README.md"), "# stray");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-05");
    assert!(
        errors.is_empty(),
        ".gitkeep should suppress rule 05 even when files are present: {errors:#?}"
    );
}
