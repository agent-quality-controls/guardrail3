use g3rs_apparch_config_checks::check as check_config;
use g3rs_apparch_ingestion_assertions::run::config as assertions;
use guardrail3_check_types::G3Severity;

#[test]
fn end_to_end_dependency_violation_fires() {
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
    super::helpers::write(
        root.path().join("logic/service/src/lib.rs"),
        "pub fn orchestrate() {}\n",
    );
    super::helpers::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        "[package]\nname = \"db-outbound\"\nversion = \"0.1.0\"\n",
    );
    super::helpers::write(
        root.path().join("io/outbound/db/src/lib.rs"),
        "pub trait DbTrait {}\n",
    );

    let results = check_config(&super::helpers::config_input(root.path()));

    assertions::assert_has_result(
        &results,
        "RS-APPARCH-CONFIG-02",
        G3Severity::Error,
        Some("logic/service/Cargo.toml"),
    );
}

#[test]
fn hidden_patch_alias_into_io_outbound_is_reported() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["types/core", "logic/service", "io/outbound/db"]

[patch.crates-io]
serde = { path = "io/outbound/db" }
"#,
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
types-core = { path = "../../types/core", package = "types-core" }
serde = "1"
"#,
    );
    super::helpers::write(root.path().join("logic/service/src/lib.rs"), "");
    super::helpers::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        "[package]\nname = \"db-outbound\"\nversion = \"0.1.0\"\n",
    );
    super::helpers::write(root.path().join("io/outbound/db/src/lib.rs"), "");

    let results = check_config(&super::helpers::config_input(root.path()));

    assertions::assert_has_result(&results, "RS-APPARCH-CONFIG-05", G3Severity::Error, None);
}

#[test]
fn hidden_replace_into_logic_is_reported() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["types/core", "logic/service"]

[replace]
"serde:1.0.0" = { path = "logic/service" }
"#,
    );
    super::helpers::write(
        root.path().join("types/core/Cargo.toml"),
        r#"
[package]
name = "types-core"
version = "0.1.0"

[dependencies]
serde = "1"
"#,
    );
    super::helpers::write(root.path().join("types/core/src/lib.rs"), "");
    super::helpers::write(
        root.path().join("logic/service/Cargo.toml"),
        "[package]\nname = \"logic-service\"\nversion = \"0.1.0\"\n",
    );
    super::helpers::write(root.path().join("logic/service/src/lib.rs"), "");

    let results = check_config(&super::helpers::config_input(root.path()));

    assertions::assert_has_result(&results, "RS-APPARCH-CONFIG-05", G3Severity::Error, None);
}

#[test]
fn same_layer_cycle_hidden_through_multiple_types_crates_is_reported() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"types/a\", \"types/b\", \"types/c\"]\n",
    );
    super::helpers::write(
        root.path().join("types/a/Cargo.toml"),
        r#"
[package]
name = "types-a"
version = "0.1.0"

[dependencies]
types-b = { path = "../b", package = "types-b" }
"#,
    );
    super::helpers::write(
        root.path().join("types/b/Cargo.toml"),
        r#"
[package]
name = "types-b"
version = "0.1.0"

[dependencies]
types-c = { path = "../c", package = "types-c" }
"#,
    );
    super::helpers::write(
        root.path().join("types/c/Cargo.toml"),
        r#"
[package]
name = "types-c"
version = "0.1.0"

[dependencies]
types-a = { path = "../a", package = "types-a" }
"#,
    );

    let results = check_config(&super::helpers::config_input(root.path()));

    assertions::assert_has_result(&results, "RS-APPARCH-CONFIG-06", G3Severity::Error, None);
}

#[test]
fn target_specific_dev_dependency_violation_warns_separately() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"logic/service\", \"io/outbound/db\", \"types/core\"]\n",
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
types-core = { path = "../../types/core", package = "types-core" }

[target.'cfg(unix)'.dev-dependencies]
db-outbound = { path = "../../io/outbound/db", package = "db-outbound" }
"#,
    );
    super::helpers::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        "[package]\nname = \"db-outbound\"\nversion = \"0.1.0\"\n",
    );

    let results = check_config(&super::helpers::config_input(root.path()));

    assertions::assert_has_result(&results, "RS-APPARCH-CONFIG-07", G3Severity::Warn, None);
}

#[test]
fn impure_external_deps_in_types_and_logic_are_reported() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"types/core\", \"logic/service\"]\n",
    );
    super::helpers::write(
        root.path().join("types/core/Cargo.toml"),
        r#"
[package]
name = "types-core"
version = "0.1.0"

[build-dependencies]
sqlx = "0.8"
"#,
    );
    super::helpers::write(
        root.path().join("logic/service/Cargo.toml"),
        r#"
[package]
name = "logic-service"
version = "0.1.0"

[dependencies]
reqwest = "0.12"
"#,
    );

    let results = check_config(&super::helpers::config_input(root.path()));

    assertions::assert_has_result(&results, "RS-APPARCH-CONFIG-08", G3Severity::Error, None);
    assertions::assert_has_result(&results, "RS-APPARCH-CONFIG-09", G3Severity::Error, None);
}
