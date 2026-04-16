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
