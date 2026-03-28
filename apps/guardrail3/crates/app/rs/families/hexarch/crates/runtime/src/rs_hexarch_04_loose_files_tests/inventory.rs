use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_04_loose_files as assertions;

#[test]
fn multiple_loose_entries_in_one_container_produce_one_error_listing_every_entry() {
    let tmp = copy_fixture();
    let container = "apps/devctl/crates/app";
    write_file(tmp.path(), &format!("{container}/mod.rs"), "// stray");
    write_file(tmp.path(), &format!("{container}/README.md"), "# stray");
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/Cargo.toml"),
        tmp.path().join(format!("{container}/notes.txt")),
    )
    .expect("symlink");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        container,
        1,
        &["loose files"],
        &[],
        &["mod.rs", "README.md", "notes.txt"],
        &[],
    );
}
