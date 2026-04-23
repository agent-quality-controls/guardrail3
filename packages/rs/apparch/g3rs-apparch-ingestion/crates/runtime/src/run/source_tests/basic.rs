#[test]
fn root_without_workspace_fails() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        "[package]\nname = \"standalone\"\nversion = \"0.1.0\"\n",
    );

    assert!(super::super::ingest_for_source_checks(&super::helpers::crawl_workspace(root.path())).is_err());
}

#[test]
fn source_ingest_collects_public_traits_from_nested_modules() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"io/outbound/db\"]\n",
    );
    super::helpers::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        "[package]\nname = \"db-outbound\"\nversion = \"0.1.0\"\n",
    );
    super::helpers::write(root.path().join("io/outbound/db/src/lib.rs"), "pub mod adapter;\n");
    super::helpers::write(
        root.path().join("io/outbound/db/src/adapter.rs"),
        "pub trait DbTrait {}\n#[cfg(test)] pub trait HiddenTrait {}\n",
    );

    let input = super::helpers::source_input(root.path());

    assert_eq!(input.io_traits_checks.len(), 1);
    assert_eq!(input.io_traits_checks[0].public_traits.len(), 1);
    assert_eq!(
        input.io_traits_checks[0].public_traits[0].rel_path,
        "io/outbound/db/src/adapter.rs"
    );
    assert_eq!(input.io_traits_checks[0].public_traits[0].item_name, "DbTrait");
}

#[test]
fn source_ingest_ignores_public_trait_in_private_child_module() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"io/outbound/db\"]\n",
    );
    super::helpers::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        "[package]\nname = \"db-outbound\"\nversion = \"0.1.0\"\n",
    );
    super::helpers::write(root.path().join("io/outbound/db/src/lib.rs"), "mod adapter;\n");
    super::helpers::write(
        root.path().join("io/outbound/db/src/adapter.rs"),
        "pub trait HiddenPort {}\n",
    );

    let input = super::helpers::source_input(root.path());

    assert!(!contains_io_trait(
        &input,
        "io/outbound/db/Cargo.toml",
        "io/outbound/db/src/adapter.rs",
        "HiddenPort"
    ));
}

#[test]
fn source_ingest_collects_public_trait_reexported_from_private_module() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"io/outbound/db\"]\n",
    );
    super::helpers::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        "[package]\nname = \"db-outbound\"\nversion = \"0.1.0\"\n",
    );
    super::helpers::write(
        root.path().join("io/outbound/db/src/lib.rs"),
        "mod adapter;\npub use adapter::DbTrait;\n",
    );
    super::helpers::write(
        root.path().join("io/outbound/db/src/adapter.rs"),
        "pub trait DbTrait {}\n",
    );

    let input = super::helpers::source_input(root.path());

    assert!(contains_io_trait(
        &input,
        "io/outbound/db/Cargo.toml",
        "io/outbound/db/src/adapter.rs",
        "DbTrait"
    ));
}

#[test]
fn source_ingest_does_not_lose_public_trait_when_private_path_is_seen_first() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"io/outbound/db\"]\n",
    );
    super::helpers::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        r#"
[package]
name = "db-outbound"
version = "0.1.0"

[lib]
path = "src/z_lib.rs"

[[bin]]
name = "db-bin"
path = "src/a_main.rs"
"#,
    );
    super::helpers::write(
        root.path().join("io/outbound/db/src/z_lib.rs"),
        "#[path = \"shared.rs\"] pub mod shared;\n",
    );
    super::helpers::write(
        root.path().join("io/outbound/db/src/a_main.rs"),
        "#[path = \"shared.rs\"] mod shared;\nfn main() {}\n",
    );
    super::helpers::write(
        root.path().join("io/outbound/db/src/shared.rs"),
        "pub trait SharedTrait {}\n",
    );

    let input = super::helpers::source_input(root.path());

    assert!(contains_io_trait(
        &input,
        "io/outbound/db/Cargo.toml",
        "io/outbound/db/src/shared.rs",
        "SharedTrait"
    ));
}

#[test]
fn source_ingest_scans_default_entrypoint_alongside_explicit_one() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"io/outbound/db\"]\n",
    );
    super::helpers::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        r#"
[package]
name = "db-outbound"
version = "0.1.0"

[[bin]]
name = "worker"
path = "src/custom_bin.rs"
"#,
    );
    super::helpers::write(root.path().join("io/outbound/db/src/custom_bin.rs"), "fn main() {}\n");
    super::helpers::write(root.path().join("io/outbound/db/src/lib.rs"), "pub trait LibTrait {}\n");

    let input = super::helpers::source_input(root.path());

    assert!(contains_io_trait(
        &input,
        "io/outbound/db/Cargo.toml",
        "io/outbound/db/src/lib.rs",
        "LibTrait"
    ));
}

#[test]
fn source_ingest_missing_declared_module_fails_closed() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"io/outbound/db\"]\n",
    );
    super::helpers::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        "[package]\nname = \"db-outbound\"\nversion = \"0.1.0\"\n",
    );
    super::helpers::write(root.path().join("io/outbound/db/src/lib.rs"), "pub mod adapter;\n");

    assert!(super::super::ingest_for_source_checks(&super::helpers::crawl_workspace(root.path())).is_err());
}

#[test]
fn source_ingest_ignores_file_level_cfg_test_module() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"io/outbound/db\"]\n",
    );
    super::helpers::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        "[package]\nname = \"db-outbound\"\nversion = \"0.1.0\"\n",
    );
    super::helpers::write(root.path().join("io/outbound/db/src/lib.rs"), "pub mod adapter;\n");
    super::helpers::write(
        root.path().join("io/outbound/db/src/adapter.rs"),
        "#![cfg(test)]\npub trait TestOnlyTrait {}\n",
    );

    let input = super::helpers::source_input(root.path());

    assert!(input
        .io_traits_checks
        .iter()
        .all(|check| check.public_traits.is_empty()));
    assert!(input
        .types_public_surface_checks
        .iter()
        .all(|check| check.public_behavior_items.is_empty()));
}

fn contains_io_trait(
    input: &g3rs_apparch_types::G3RsApparchSourceChecksInput,
    cargo_rel_path: &str,
    rel_path: &str,
    item_name: &str,
) -> bool {
    input
        .io_traits_checks
        .iter()
        .find(|check| check.krate.cargo_rel_path == cargo_rel_path)
        .into_iter()
        .flat_map(|check| &check.public_traits)
        .any(|fact| fact.rel_path == rel_path && fact.item_name == item_name)
}
