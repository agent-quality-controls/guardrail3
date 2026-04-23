use g3rs_apparch_ingestion_assertions::run::source as assertions;
use g3rs_apparch_source_checks::check as check_source;
use guardrail3_check_types::G3Severity;

#[test]
fn end_to_end_source_violation_fires() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"types/core\", \"logic/service\", \"io/outbound/db\"]\n",
    );
    super::helpers::write(
        root.path().join("types/core/Cargo.toml"),
        "[package]\nname = \"types-core\"\nversion = \"0.1.0\"\n",
    );
    super::helpers::write(
        root.path().join("logic/service/Cargo.toml"),
        r#"
[package]
name = "logic-service"
version = "0.1.0"

[dependencies]
db-outbound = { path = "../../io/outbound/db", package = "db-outbound" }
"#,
    );
    super::helpers::write(root.path().join("logic/service/src/lib.rs"), "pub fn orchestrate() {}\n");
    super::helpers::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        "[package]\nname = \"db-outbound\"\nversion = \"0.1.0\"\n",
    );
    super::helpers::write(root.path().join("io/outbound/db/src/lib.rs"), "pub trait DbTrait {}\n");

    let results = check_source(&super::helpers::source_input(root.path()));

    assertions::assert_has_result(
        &results,
        "RS-APPARCH-SOURCE-04",
        G3Severity::Error,
        Some("io/outbound/db/src/lib.rs"),
    );
}

#[test]
fn types_public_behavior_is_reported_even_without_public_traits() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"types/contracts\"]\n",
    );
    super::helpers::write(
        root.path().join("types/contracts/Cargo.toml"),
        "[package]\nname = \"types-contracts\"\nversion = \"0.1.0\"\n",
    );
    super::helpers::write(
        root.path().join("types/contracts/src/lib.rs"),
        r#"
pub struct OrderDto;

impl OrderDto {
    pub fn save_to_db(&self) {}
}

pub fn choose_retry_strategy() {}
"#,
    );

    let results = check_source(&super::helpers::source_input(root.path()));

    assertions::assert_has_result(&results, "RS-APPARCH-SOURCE-05", G3Severity::Error, None);
}

#[test]
fn private_child_module_behavior_does_not_reach_types_public_surface_inventory() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"types/contracts\"]\n",
    );
    super::helpers::write(
        root.path().join("types/contracts/Cargo.toml"),
        "[package]\nname = \"types-contracts\"\nversion = \"0.1.0\"\n",
    );
    super::helpers::write(
        root.path().join("types/contracts/src/lib.rs"),
        "mod internal;\npub struct OrderDto;\n",
    );
    super::helpers::write(
        root.path().join("types/contracts/src/internal.rs"),
        r#"
pub fn choose_retry_strategy() {}

impl super::OrderDto {
    pub fn save_to_db(&self) {}
}
"#,
    );

    let input = super::helpers::source_input(root.path());
    let types_check = input
        .types_public_surface_checks
        .iter()
        .find(|check| check.krate.cargo_rel_path == "types/contracts/Cargo.toml")
        .expect("types crate should be present in source checks input");

    assert!(types_check.public_behavior_items.is_empty(), "{types_check:#?}");
}

#[test]
fn types_public_behavior_reexported_from_private_child_module_is_reported() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"types/contracts\"]\n",
    );
    super::helpers::write(
        root.path().join("types/contracts/Cargo.toml"),
        "[package]\nname = \"types-contracts\"\nversion = \"0.1.0\"\n",
    );
    super::helpers::write(
        root.path().join("types/contracts/src/lib.rs"),
        "mod internal;\npub use internal::choose_retry_strategy;\n",
    );
    super::helpers::write(
        root.path().join("types/contracts/src/internal.rs"),
        "pub fn choose_retry_strategy() {}\n",
    );

    let input = super::helpers::source_input(root.path());
    let types_check = input
        .types_public_surface_checks
        .iter()
        .find(|check| check.krate.cargo_rel_path == "types/contracts/Cargo.toml")
        .expect("types crate should be present in source checks input");

    assert_eq!(types_check.public_behavior_items.len(), 1, "{types_check:#?}");
    assert_eq!(
        types_check.public_behavior_items[0].rel_path,
        "types/contracts/src/internal.rs"
    );
    assert_eq!(
        types_check.public_behavior_items[0].item_name,
        "choose_retry_strategy"
    );
}
