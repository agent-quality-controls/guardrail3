use std::collections::BTreeSet;

use super::super::super::test_support::{
    copy_fixture, create_dir, errors_by_id, run_family, write_file,
};

#[test]
fn newly_discovered_rust_app_without_crates_is_owned_by_rule_01() {
    let tmp = copy_fixture();
    create_dir(tmp.path(), "apps/phantom");
    write_file(
        tmp.path(),
        "apps/phantom/Cargo.toml",
        "[workspace]\nmembers = []\n",
    );

    let results = run_family(tmp.path());
    let actual_files = errors_by_id(&results, "RS-HEXARCH-01")
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/phantom".to_owned()]
        .into_iter()
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {results:#?}"
    );
}

#[test]
fn newly_discovered_app_with_empty_cargo_toml_is_still_owned_by_rule_01() {
    let tmp = copy_fixture();
    create_dir(tmp.path(), "apps/phantom");
    write_file(tmp.path(), "apps/phantom/Cargo.toml", "");

    let results = run_family(tmp.path());
    let actual_files = errors_by_id(&results, "RS-HEXARCH-01")
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/phantom".to_owned()]
        .into_iter()
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set for empty Cargo.toml: {results:#?}"
    );
}

#[test]
fn newly_discovered_app_with_malformed_cargo_toml_is_still_owned_by_rule_01() {
    let tmp = copy_fixture();
    create_dir(tmp.path(), "apps/phantom");
    write_file(
        tmp.path(),
        "apps/phantom/Cargo.toml",
        "this is not valid toml {{{{\n",
    );

    let results = run_family(tmp.path());
    let actual_files = errors_by_id(&results, "RS-HEXARCH-01")
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/phantom".to_owned()]
        .into_iter()
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set for malformed Cargo.toml: {results:#?}"
    );
}

#[test]
fn cargo_toml_directory_is_not_discovered_as_a_rule_01_app() {
    let tmp = copy_fixture();
    create_dir(tmp.path(), "apps/broken/Cargo.toml");

    let results = run_family(tmp.path());
    assert_eq!(
        errors_by_id(&results, "RS-HEXARCH-01").len(),
        0,
        "{results:#?}"
    );
}

#[test]
fn broken_cargo_toml_symlink_is_not_discovered_as_a_rule_01_app() {
    let tmp = copy_fixture();
    create_dir(tmp.path(), "apps/broken");
    std::os::unix::fs::symlink("/nonexistent", tmp.path().join("apps/broken/Cargo.toml"))
        .expect("symlink");

    let results = run_family(tmp.path());
    assert_eq!(
        errors_by_id(&results, "RS-HEXARCH-01").len(),
        0,
        "{results:#?}"
    );
}

#[test]
fn newly_discovered_unicode_app_name_is_owned_by_rule_01() {
    let tmp = copy_fixture();
    create_dir(tmp.path(), "apps/über-service");
    write_file(
        tmp.path(),
        "apps/über-service/Cargo.toml",
        "[workspace]\nmembers = []\n",
    );

    let results = run_family(tmp.path());
    let actual_files = errors_by_id(&results, "RS-HEXARCH-01")
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/über-service".to_owned()]
        .into_iter()
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set for unicode app name: {results:#?}"
    );
}

#[test]
fn newly_discovered_app_name_with_spaces_is_owned_by_rule_01() {
    let tmp = copy_fixture();
    create_dir(tmp.path(), "apps/uber service");
    write_file(
        tmp.path(),
        "apps/uber service/Cargo.toml",
        "[workspace]\nmembers = []\n",
    );

    let results = run_family(tmp.path());
    let actual_files = errors_by_id(&results, "RS-HEXARCH-01")
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/uber service".to_owned()]
        .into_iter()
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set for spaced app name: {results:#?}"
    );
}

#[test]
fn newly_discovered_app_with_present_crates_and_banned_src_is_not_owned_by_rule_01() {
    let tmp = copy_fixture();
    create_dir(tmp.path(), "apps/phantom/crates/app/core/src");
    create_dir(tmp.path(), "apps/phantom/crates/domain/models/src");
    create_dir(tmp.path(), "apps/phantom/crates/ports/inbound/http/src");
    create_dir(tmp.path(), "apps/phantom/crates/ports/outbound/storage/src");
    create_dir(tmp.path(), "apps/phantom/crates/adapters/inbound/rest/src");
    create_dir(tmp.path(), "apps/phantom/crates/adapters/outbound/db/src");
    write_file(
        tmp.path(),
        "apps/phantom/Cargo.toml",
        "[workspace]\nmembers = [\"crates/app/core\", \"crates/domain/models\", \"crates/ports/inbound/http\", \"crates/ports/outbound/storage\", \"crates/adapters/inbound/rest\", \"crates/adapters/outbound/db\"]\n",
    );
    write_file(
        tmp.path(),
        "apps/phantom/crates/app/core/Cargo.toml",
        "[package]\nname = \"phantom-app-core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        tmp.path(),
        "apps/phantom/crates/domain/models/Cargo.toml",
        "[package]\nname = \"phantom-domain-models\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        tmp.path(),
        "apps/phantom/crates/ports/inbound/http/Cargo.toml",
        "[package]\nname = \"phantom-ports-inbound-http\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        tmp.path(),
        "apps/phantom/crates/ports/outbound/storage/Cargo.toml",
        "[package]\nname = \"phantom-ports-outbound-storage\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        tmp.path(),
        "apps/phantom/crates/adapters/inbound/rest/Cargo.toml",
        "[package]\nname = \"phantom-adapters-inbound-rest\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        tmp.path(),
        "apps/phantom/crates/adapters/outbound/db/Cargo.toml",
        "[package]\nname = \"phantom-adapters-outbound-db\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(tmp.path(), "apps/phantom/src/main.rs", "fn main() {}\n");

    let results = run_family(tmp.path());

    assert_eq!(
        errors_by_id(&results, "RS-HEXARCH-01").len(),
        0,
        "{results:#?}"
    );
    let actual_files = errors_by_id(&results, "RS-HEXARCH-12")
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/phantom/src".to_owned()]
        .into_iter()
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected rule-12 hit set: {results:#?}"
    );
    for id in [
        "RS-HEXARCH-02",
        "RS-HEXARCH-03",
        "RS-HEXARCH-04",
        "RS-HEXARCH-05",
        "RS-HEXARCH-06",
    ] {
        assert_eq!(errors_by_id(&results, id).len(), 0, "{id}: {results:#?}");
    }
}
