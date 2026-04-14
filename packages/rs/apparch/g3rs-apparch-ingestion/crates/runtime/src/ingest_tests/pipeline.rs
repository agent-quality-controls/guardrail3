use g3rs_apparch_config_checks::check as check_config;
use g3rs_apparch_source_checks::check as check_source;
use guardrail3_check_types::G3Severity;
use tempfile::tempdir;

#[test]
fn end_to_end_dependency_and_source_violations_fire() {
    let root = tempdir().expect("tempdir");
    super::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"types/core\", \"logic/service\", \"io/outbound/db\"]\n",
    );
    super::write(
        root.path().join("types/core/Cargo.toml"),
        "[package]\nname = \"types-core\"\nversion = \"0.1.0\"\n",
    );
    super::write(
        root.path().join("logic/service/Cargo.toml"),
        r#"
[package]
name = "logic-service"
version = "0.1.0"

[dependencies]
db-outbound = { path = "../../io/outbound/db", package = "db-outbound" }
"#,
    );
    super::write(
        root.path().join("logic/service/src/lib.rs"),
        "pub fn orchestrate() {}\n",
    );
    super::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        r#"
[package]
name = "db-outbound"
version = "0.1.0"
"#,
    );
    super::write(root.path().join("io/outbound/db/src/lib.rs"), "pub trait DbTrait {}\n");

    let crawl = super::crawl_workspace(root.path());
    let config_results = check_config(&crate::ingest_for_config_checks(&crawl).expect("config ingest"));
    let source_results = check_source(&crate::ingest_for_source_checks(&crawl).expect("source ingest"));

    let config_result = config_results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-02")
        .expect("logic->io violation");
    assert_eq!(config_result.severity(), G3Severity::Error);
    assert_eq!(config_result.file(), Some("logic/service/Cargo.toml"));

    let source_result = source_results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-SOURCE-04")
        .expect("io trait violation");
    assert_eq!(source_result.severity(), G3Severity::Error);
    assert_eq!(source_result.file(), Some("io/outbound/db/src/lib.rs"));
}

#[test]
fn hidden_patch_alias_into_io_outbound_is_reported() {
    let root = tempdir().expect("tempdir");
    super::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["types/core", "logic/service", "io/outbound/db"]

[patch.crates-io]
serde = { path = "io/outbound/db" }
"#,
    );
    super::write(
        root.path().join("types/core/Cargo.toml"),
        "[package]\nname = \"types-core\"\nversion = \"0.1.0\"\n",
    );
    super::write(
        root.path().join("logic/service/Cargo.toml"),
        r#"
[package]
name = "logic-service"
version = "0.1.0"

[dependencies]
types-core = { path = "../../types/core", package = "types-core" }
serde = "1"
"#,
    );
    super::write(root.path().join("logic/service/src/lib.rs"), "");
    super::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        "[package]\nname = \"db-outbound\"\nversion = \"0.1.0\"\n",
    );
    super::write(root.path().join("io/outbound/db/src/lib.rs"), "");

    let crawl = super::crawl_workspace(root.path());
    let config_results = check_config(&crate::ingest_for_config_checks(&crawl).expect("config ingest"));

    let patch_result = config_results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-05")
        .expect("hidden patch bypass should be reported");
    assert_eq!(patch_result.severity(), G3Severity::Error);
}

#[test]
fn hidden_replace_into_logic_is_reported() {
    let root = tempdir().expect("tempdir");
    super::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["types/core", "logic/service"]

[replace]
"serde:1.0.0" = { path = "logic/service" }
"#,
    );
    super::write(
        root.path().join("types/core/Cargo.toml"),
        r#"
[package]
name = "types-core"
version = "0.1.0"

[dependencies]
serde = "1"
"#,
    );
    super::write(root.path().join("types/core/src/lib.rs"), "");
    super::write(
        root.path().join("logic/service/Cargo.toml"),
        "[package]\nname = \"logic-service\"\nversion = \"0.1.0\"\n",
    );
    super::write(root.path().join("logic/service/src/lib.rs"), "");

    let crawl = super::crawl_workspace(root.path());
    let config_results = check_config(&crate::ingest_for_config_checks(&crawl).expect("config ingest"));

    let replace_result = config_results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-05")
        .expect("hidden replace bypass should be reported");
    assert_eq!(replace_result.severity(), G3Severity::Error);
}

#[test]
fn same_layer_cycle_hidden_through_multiple_types_crates_is_reported() {
    let root = tempdir().expect("tempdir");
    super::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"types/a\", \"types/b\", \"types/c\"]\n",
    );
    super::write(
        root.path().join("types/a/Cargo.toml"),
        r#"
[package]
name = "types-a"
version = "0.1.0"

[dependencies]
types-b = { path = "../b", package = "types-b" }
"#,
    );
    super::write(
        root.path().join("types/b/Cargo.toml"),
        r#"
[package]
name = "types-b"
version = "0.1.0"

[dependencies]
types-c = { path = "../c", package = "types-c" }
"#,
    );
    super::write(
        root.path().join("types/c/Cargo.toml"),
        r#"
[package]
name = "types-c"
version = "0.1.0"

[dependencies]
types-a = { path = "../a", package = "types-a" }
"#,
    );

    let crawl = super::crawl_workspace(root.path());
    let config_results = check_config(&crate::ingest_for_config_checks(&crawl).expect("config ingest"));

    let cycle_result = config_results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-06")
        .expect("same-layer cycle should be reported");
    assert_eq!(cycle_result.severity(), G3Severity::Error);
}

#[test]
fn target_specific_dev_dependency_violation_warns_separately() {
    let root = tempdir().expect("tempdir");
    super::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"logic/service\", \"io/outbound/db\", \"types/core\"]\n",
    );
    super::write(
        root.path().join("types/core/Cargo.toml"),
        "[package]\nname = \"types-core\"\nversion = \"0.1.0\"\n",
    );
    super::write(
        root.path().join("logic/service/Cargo.toml"),
        r#"
[package]
name = "logic-service"
version = "0.1.0"

[dependencies]
types-core = { path = "../../types/core", package = "types-core" }

[target.'cfg(unix)'.dev-dependencies]
db-outbound = { path = "../../io/outbound/db", package = "db-outbound" }
"#,
    );
    super::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        "[package]\nname = \"db-outbound\"\nversion = \"0.1.0\"\n",
    );

    let crawl = super::crawl_workspace(root.path());
    let config_results = check_config(&crate::ingest_for_config_checks(&crawl).expect("config ingest"));

    let dev_result = config_results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-07")
        .expect("dev-direction violation should be reported separately");
    assert_eq!(dev_result.severity(), G3Severity::Warn);
}

#[test]
fn impure_external_deps_in_types_and_logic_are_reported() {
    let root = tempdir().expect("tempdir");
    super::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"types/core\", \"logic/service\"]\n",
    );
    super::write(
        root.path().join("types/core/Cargo.toml"),
        r#"
[package]
name = "types-core"
version = "0.1.0"

[build-dependencies]
sqlx = "0.8"
"#,
    );
    super::write(
        root.path().join("logic/service/Cargo.toml"),
        r#"
[package]
name = "logic-service"
version = "0.1.0"

[dependencies]
reqwest = "0.12"
"#,
    );

    let crawl = super::crawl_workspace(root.path());
    let config_results = check_config(&crate::ingest_for_config_checks(&crawl).expect("config ingest"));

    let types_purity = config_results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-08")
        .expect("types impurity should be reported");
    assert_eq!(types_purity.severity(), G3Severity::Error);

    let logic_purity = config_results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-CONFIG-09")
        .expect("logic impurity should be reported");
    assert_eq!(logic_purity.severity(), G3Severity::Error);
}

#[test]
fn types_public_behavior_is_reported_even_without_public_traits() {
    let root = tempdir().expect("tempdir");
    super::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"types/contracts\"]\n",
    );
    super::write(
        root.path().join("types/contracts/Cargo.toml"),
        "[package]\nname = \"types-contracts\"\nversion = \"0.1.0\"\n",
    );
    super::write(
        root.path().join("types/contracts/src/lib.rs"),
        r#"
pub struct OrderDto;

impl OrderDto {
    pub fn save_to_db(&self) {}
}

pub fn choose_retry_strategy() {}
"#,
    );

    let crawl = super::crawl_workspace(root.path());
    let source_results = check_source(&crate::ingest_for_source_checks(&crawl).expect("source ingest"));

    let source_result = source_results
        .iter()
        .find(|result| result.id() == "RS-APPARCH-SOURCE-05")
        .expect("types public behavior should be reported");
    assert_eq!(source_result.severity(), G3Severity::Error);
}
