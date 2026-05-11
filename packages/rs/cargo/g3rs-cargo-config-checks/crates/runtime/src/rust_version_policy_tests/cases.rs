use g3rs_cargo_config_checks_assertions::rust_version_policy as assertions;
use g3rs_toml_parser::types::RustProfile;
use test_support::{parsed_rust_policy, root};

#[test]
fn errors_when_library_profile_has_no_rust_version() {
    let root = root(
        r#"
            [package]
            name = "pkg"
            edition = "2024"
        "#,
        parsed_rust_policy(Some(RustProfile::Library), Vec::new()),
    );
    let mut results = Vec::new();

    crate::rust_version_policy::check(&root, &mut results);

    assertions::assert_has_error(&results, "library rust-version missing", false);
}

#[test]
fn inventories_when_library_profile_declares_rust_version() {
    let root = root(
        r#"
            [package]
            name = "pkg"
            edition = "2024"
            rust-version = "1.84"
        "#,
        parsed_rust_policy(Some(RustProfile::Library), Vec::new()),
    );
    let mut results = Vec::new();

    crate::rust_version_policy::check(&root, &mut results);

    assertions::assert_has_info(&results, "library rust-version declared", true);
}

#[test]
fn errors_when_rust_version_shape_is_invalid() {
    let root = root(
        r#"
            [package]
            name = "pkg"
            edition = "2024"
            rust-version = []
        "#,
        parsed_rust_policy(Some(RustProfile::Library), Vec::new()),
    );
    let mut results = Vec::new();

    crate::rust_version_policy::check(&root, &mut results);

    assertions::assert_has_error(&results, "rust-version invalid", false);
}

#[test]
fn inventories_when_non_library_omits_rust_version() {
    let root = root(
        r#"
            [package]
            name = "pkg"
            edition = "2024"
        "#,
        parsed_rust_policy(Some(RustProfile::Service), Vec::new()),
    );
    let mut results = Vec::new();

    crate::rust_version_policy::check(&root, &mut results);

    assertions::assert_has_info(&results, "rust-version inventory", true);
}

#[test]
fn inventories_when_workspace_root_library_declares_rust_version() {
    let root = root(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.package]
            rust-version = "1.84"
        "#,
        parsed_rust_policy(Some(RustProfile::Library), Vec::new()),
    );
    let mut results = Vec::new();

    crate::rust_version_policy::check(&root, &mut results);

    assertions::assert_has_info(&results, "library rust-version declared", true);
}

#[test]
fn errors_when_workspace_root_rust_version_shape_is_invalid() {
    let root = root(
        r#"
            [workspace]
            members = []
            resolver = "2"

            [workspace.package]
            rust-version = []

            [package]
            name = "pkg"
            edition = "2024"
            rust-version = "1.84"
        "#,
        parsed_rust_policy(Some(RustProfile::Library), Vec::new()),
    );
    let mut results = Vec::new();

    crate::rust_version_policy::check(&root, &mut results);

    assertions::assert_has_error(&results, "rust-version invalid", false);
}
