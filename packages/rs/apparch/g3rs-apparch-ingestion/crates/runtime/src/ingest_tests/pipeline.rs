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
