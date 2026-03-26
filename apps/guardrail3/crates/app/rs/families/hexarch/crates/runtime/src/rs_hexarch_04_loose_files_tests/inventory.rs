use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_04_loose_files as assertions;
use crate::test_support::{copy_fixture, write_file};

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

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-04");
    assert_eq!(errors.len(), 1, "{errors:#?}");
    assert_eq!(errors[0].file.as_deref(), Some(container), "{errors:#?}");
    for name in ["mod.rs", "README.md", "notes.txt"] {
        assert!(
            errors[0].message.contains(name),
            "expected message to mention {name}: {errors:#?}"
        );
    }
}
