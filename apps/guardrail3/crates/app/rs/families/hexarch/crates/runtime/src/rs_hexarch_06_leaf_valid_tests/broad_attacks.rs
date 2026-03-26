use std::collections::BTreeSet;

use super::cases::owned_leaf_dirs;
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_06_leaf_valid as assertions;
use crate::test_support::{copy_fixture, create_dir, write_file};

#[test]
fn orphan_leaf_without_cargo_or_crates_errors_everywhere_it_is_owned() {
    let tmp = copy_fixture();
    let expected_files = owned_leaf_dirs(tmp.path(), "orphan");

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-06");
    assert_eq!(errors.len(), expected_files.len(), "{errors:#?}");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
    for error in &errors {
        assert!(error.title.contains("missing Cargo.toml"));
        assert!(error.title.contains("orphan"));
    }
}

#[test]
fn leaf_with_both_cargo_and_crates_errors_everywhere_it_is_owned() {
    let tmp = copy_fixture();
    let expected_files = owned_leaf_dirs(tmp.path(), "hybrid");
    for rel in &expected_files {
        write_file(
            tmp.path(),
            &format!("{rel}/Cargo.toml"),
            "[package]\nname = \"hybrid\"\nversion = \"0.1.0\"\n",
        );
        for container in [
            "app",
            "domain",
            "adapters/inbound",
            "adapters/outbound",
            "ports/inbound",
            "ports/outbound",
        ] {
            write_file(
                tmp.path(),
                &format!("{rel}/crates/{container}/.gitkeep"),
                "",
            );
        }
        write_file(tmp.path(), &format!("{rel}/.gitkeep"), "");
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-06");
    assert_eq!(errors.len(), expected_files.len(), "{errors:#?}");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
    for error in &errors {
        assert!(error.title.contains("both Cargo.toml and crates/"));
        assert!(error.title.contains("hybrid"));
    }
}

#[test]
fn gitkeep_plus_source_files_fires_everywhere() {
    let tmp = copy_fixture();
    let expected_files = owned_leaf_dirs(tmp.path(), "broken_placeholder");
    for rel in &expected_files {
        write_file(tmp.path(), &format!("{rel}/.gitkeep"), "");
        write_file(
            tmp.path(),
            &format!("{rel}/src/lib.rs"),
            "// placeholder with source is not valid",
        );
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-06");
    assert_eq!(errors.len(), expected_files.len(), "{errors:#?}");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set for .gitkeep+source placeholder: {errors:#?}"
    );
    for error in &errors {
        assert!(error.title.contains("missing Cargo.toml"));
        assert!(error.title.contains("broken_placeholder"));
    }
}

#[test]
fn gitkeep_plus_subdir_fires_everywhere() {
    let tmp = copy_fixture();
    let expected_files = owned_leaf_dirs(tmp.path(), "broken_subdir");
    for rel in &expected_files {
        write_file(tmp.path(), &format!("{rel}/.gitkeep"), "");
        create_dir(tmp.path(), &format!("{rel}/src"));
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-06");
    assert_eq!(errors.len(), expected_files.len(), "{errors:#?}");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set for .gitkeep+subdir placeholder: {errors:#?}"
    );
    for error in &errors {
        assert!(error.title.contains("missing Cargo.toml"));
        assert!(error.title.contains("broken_subdir"));
    }
}

#[test]
fn gitkeep_as_directory_fires_everywhere() {
    let tmp = copy_fixture();
    let expected_files = owned_leaf_dirs(tmp.path(), "fake_placeholder");
    for rel in &expected_files {
        create_dir(tmp.path(), &format!("{rel}/.gitkeep"));
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-06");
    assert_eq!(errors.len(), expected_files.len(), "{errors:#?}");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set for .gitkeep-as-directory placeholder: {errors:#?}"
    );
    for error in &errors {
        assert!(error.title.contains("missing Cargo.toml"));
        assert!(error.title.contains("fake_placeholder"));
    }
}

#[test]
fn flat_files_only_leaf_fires_everywhere() {
    let tmp = copy_fixture();
    let expected_files = owned_leaf_dirs(tmp.path(), "flat_files");
    for rel in &expected_files {
        write_file(tmp.path(), &format!("{rel}/README.md"), "# stray");
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-06");
    assert_eq!(errors.len(), expected_files.len(), "{errors:#?}");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual_files, expected_files, "{errors:#?}");
    for error in &errors {
        assert!(error.title.contains("missing Cargo.toml"));
        assert!(error.title.contains("flat_files"));
    }
}

#[test]
fn packages_noise_is_ignored_by_rule_06() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "packages/shared-types/crates/app/packages_orphan/src/lib.rs",
        "",
    );
    write_file(
        tmp.path(),
        "packages/ui-kit/crates/domain/packages_hex/crates/app/.gitkeep",
        "",
    );

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-06");
    assert!(
        errors.is_empty(),
        "packages/ noise should not be owned by rule 06: {errors:#?}"
    );
}

#[test]
fn non_rust_apps_noise_is_ignored_by_rule_06() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/crates/app/admin_orphan/src/lib.rs",
        "",
    );
    write_file(
        tmp.path(),
        "apps/landing/crates/domain/admin_hybrid/Cargo.toml",
        "[package]\nname = \"admin-hybrid\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "apps/landing/crates/domain/admin_hybrid/crates/app/.gitkeep",
        "",
    );

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-06");
    assert!(
        errors.is_empty(),
        "non-Rust apps should stay out of scope for rule 06: {errors:#?}"
    );
}
