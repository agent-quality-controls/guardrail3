use std::path::Path;

use tempfile::TempDir;

pub(super) fn crawl(root: &Path) -> g3rs_workspace_crawl::G3RsWorkspaceCrawl {
    super::helpers::crawl(root)
}

pub(super) fn new_root() -> TempDir {
    super::helpers::new_root()
}

pub(super) fn write(path: impl AsRef<Path>, content: &str) {
    super::helpers::write(path, content);
}

#[cfg(unix)]
pub(super) fn make_unreadable(path: &Path) {
    super::helpers::make_unreadable(path);
}

pub(super) fn ingest_for_source_checks(
    crawl: &g3rs_workspace_crawl::G3RsWorkspaceCrawl,
) -> Result<g3rs_garde_types::G3RsGardeSourceChecksInput, super::IngestionError> {
    super::ingest_for_source_checks(crawl)
}

pub(super) use super::IngestionError;

#[cfg(test)]
mod tests {
    use g3rs_garde_ingestion_assertions::run as assertions;

    use super::*;

    #[test]
    fn pipeline_stays_quiet_for_clean_garde_root() {
        let temp = new_root();
        let root = temp.path();

        write(
            root.join("Cargo.toml"),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\ngarde = \"0.22\"\n",
        );
        write(
            root.join("src/lib.rs"),
            "use garde::Validate;\nuse serde::Deserialize;\n\n#[derive(Deserialize, Validate)]\nstruct Input {\n    #[garde(length(min = 1))]\n    name: String,\n}\n",
        );

        let crawl = crawl(root);
        let input = ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
        let results = g3rs_garde_source_checks::check(&input);

        assertions::assert_no_results(&results);
    }

    #[test]
    fn pipeline_can_report_input_failures_and_ast_findings_together() {
        let temp = new_root();
        let root = temp.path();

        write(
            root.join("Cargo.toml"),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\ngarde = \"0.22\"\n",
        );
        write(root.join("src/broken.rs"), "fn broken( {\n");
        write(
            root.join("src/input.rs"),
            "use serde::Deserialize;\n\n#[derive(Deserialize)]\nstruct Input {\n    name: String,\n}\n",
        );

        let crawl = crawl(root);
        let input = ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
        let results = g3rs_garde_source_checks::check(&input);

        assertions::assert_rule_present(&results, "g3rs-garde/input-failures", "src/broken.rs");
        assertions::assert_rule_present(
            &results,
            "g3rs-garde/struct-derive-validate",
            "src/input.rs",
        );
    }

    #[test]
    fn pipeline_ignores_legacy_guardrail_config_parse_sites() {
        let temp = new_root();
        let root = temp.path();

        write(
            root.join("Cargo.toml"),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\ngarde = \"0.22\"\n",
        );
        write(
            root.join("src/load_config.rs"),
            "struct GuardrailConfig;\n\nfn load_config(content: &str) -> Option<GuardrailConfig> {\n    toml::from_str(content).ok()\n}\n",
        );

        let crawl = crawl(root);
        let input = ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
        let results = g3rs_garde_source_checks::check(&input);

        assertions::assert_no_results(&results);
    }

    #[test]
    fn pipeline_uses_rust_policy_waivers_for_query_as() {
        let temp = new_root();
        let root = temp.path();

        write(
            root.join("Cargo.toml"),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
        );
        write(
            root.join("guardrail3-rs.toml"),
            "profile = \"service\"\n\n[checks]\ngarde = true\n\n[[waivers]]\nrule = \"g3rs-garde/query-as-inventory\"\nfile = \"src/db.rs\"\nselector = \"qa@L4\"\nreason = \"Temporary SQLx row mapping until validated DTO extraction lands.\"\n",
        );
        write(
            root.join("src/db.rs"),
            "use sqlx::query_as as qa;\n\nfn load() {\n    let _row = qa!(User, \"select 1\");\n}\n",
        );

        let crawl = crawl(root);
        let input = ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
        let results = g3rs_garde_source_checks::check(&input);

        assertions::assert_rule_present(&results, "g3rs-garde/query-as-inventory", "src/db.rs");
        assertions::assert_rule_absent(
            &results,
            "g3rs-garde/query-as-inventory",
            "sqlx query_as missing reason",
        );
    }

    #[test]
    fn ast_ingestion_allows_missing_rust_policy_when_garde_is_present() {
        let temp = new_root();
        let root = temp.path();

        write(
            root.join("Cargo.toml"),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\ngarde = \"0.22\"\n",
        );
        write(root.join("src/lib.rs"), "fn ok() {}\n");

        let crawl = crawl(root);
        let result = ingest_for_source_checks(&crawl).expect("source ingestion should succeed");

        assert!(result.input_failures.is_empty(), "{result:#?}");
        assert_eq!(result.struct_targets.len(), 0, "{result:#?}");
    }

    #[test]
    fn ast_ingestion_fails_when_cargo_is_missing() {
        let temp = new_root();
        let root = temp.path();

        write(root.join("src/lib.rs"), "fn ok() {}\n");

        let crawl = crawl(root);
        let result = ingest_for_source_checks(&crawl);

        assert!(
            matches!(result, Err(IngestionError::CargoTomlNotFound)),
            "{result:#?}"
        );
    }

    #[test]
    fn ast_ingestion_fails_when_cargo_is_malformed() {
        let temp = new_root();
        let root = temp.path();

        write(root.join("Cargo.toml"), "{{{{not valid toml}}}}");
        write(root.join("src/lib.rs"), "fn ok() {}\n");

        let crawl = crawl(root);
        let result = ingest_for_source_checks(&crawl);

        assert!(
            matches!(result, Err(IngestionError::ParseFailed { .. })),
            "{result:#?}"
        );
    }

    #[cfg(unix)]
    #[test]
    fn ast_ingestion_fails_when_cargo_is_unreadable() {
        let temp = new_root();
        let root = temp.path();

        write(
            root.join("Cargo.toml"),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\ngarde = \"0.22\"\n",
        );
        write(root.join("src/lib.rs"), "fn ok() {}\n");
        make_unreadable(&root.join("Cargo.toml"));

        let crawl = crawl(root);
        let result = ingest_for_source_checks(&crawl);

        assert!(
            matches!(result, Err(IngestionError::Unreadable { .. })),
            "{result:#?}"
        );
    }

    #[test]
    fn pipeline_reports_malformed_source_via_garde_10() {
        let temp = new_root();
        let root = temp.path();

        write(
            root.join("Cargo.toml"),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\ngarde = \"0.22\"\n",
        );
        write(root.join("src/lib.rs"), "fn broken( {\n");

        let crawl = crawl(root);
        let input = ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
        let results = g3rs_garde_source_checks::check(&input);

        assertions::assert_rule_present(&results, "g3rs-garde/input-failures", "src/lib.rs");
    }

    #[test]
    fn pipeline_reports_malformed_guardrail_via_garde_10() {
        let temp = new_root();
        let root = temp.path();

        write(
            root.join("Cargo.toml"),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\ngarde = \"0.22\"\n",
        );
        write(root.join("guardrail3-rs.toml"), "[[broken");
        write(
            root.join("src/lib.rs"),
            "use garde::Validate;\nuse sqlx::query_as;\n\n#[derive(Validate)]\nstruct Input;\n\nfn load() { let _ = query_as!(User, \"select 1\"); }\n",
        );

        let crawl = crawl(root);
        let input = ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
        let results = g3rs_garde_source_checks::check(&input);

        assertions::assert_rule_present(
            &results,
            "g3rs-garde/input-failures",
            "guardrail3-rs.toml",
        );
        assertions::assert_rule_id_absent(&results, "g3rs-garde/query-as-inventory");
    }

    #[cfg(unix)]
    #[test]
    fn pipeline_reports_unreadable_source_via_garde_10() {
        let temp = new_root();
        let root = temp.path();

        write(
            root.join("Cargo.toml"),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\ngarde = \"0.22\"\n",
        );
        write(root.join("src/lib.rs"), "fn ok() {}\n");
        make_unreadable(&root.join("src/lib.rs"));

        let crawl = crawl(root);
        let input = ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
        let results = g3rs_garde_source_checks::check(&input);

        assertions::assert_rule_present(&results, "g3rs-garde/input-failures", "src/lib.rs");
    }

    #[cfg(unix)]
    #[test]
    fn pipeline_reports_unreadable_guardrail_via_garde_10() {
        let temp = new_root();
        let root = temp.path();

        write(
            root.join("Cargo.toml"),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\ngarde = \"0.22\"\n",
        );
        write(root.join("guardrail3-rs.toml"), "profile = \"service\"\n");
        write(
            root.join("src/lib.rs"),
            "use garde::Validate;\n#[derive(Validate)]\nstruct Input;\n",
        );
        make_unreadable(&root.join("guardrail3-rs.toml"));

        let crawl = crawl(root);
        let input = ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
        let results = g3rs_garde_source_checks::check(&input);

        assertions::assert_rule_present(
            &results,
            "g3rs-garde/input-failures",
            "guardrail3-rs.toml",
        );
        assertions::assert_rule_id_absent(&results, "g3rs-garde/query-as-inventory");
    }

    #[test]
    fn pipeline_stays_quiet_for_non_garde_root_without_adoption_markers() {
        let temp = new_root();
        let root = temp.path();

        write(
            root.join("Cargo.toml"),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
        );
        write(root.join("src/lib.rs"), "fn load() {}\n");

        let crawl = crawl(root);
        let input = ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
        let results = g3rs_garde_source_checks::check(&input);

        assertions::assert_no_results(&results);
    }

    #[test]
    fn pipeline_activates_for_source_adoption_markers_without_garde_dependency() {
        let temp = new_root();
        let root = temp.path();

        write(
            root.join("Cargo.toml"),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
        );
        write(
            root.join("guardrail3-rs.toml"),
            "profile = \"service\"\n\n[checks]\ngarde = true\n",
        );
        write(
            root.join("src/input.rs"),
            "use serde::Deserialize;\n\n#[derive(Deserialize)]\nstruct Input {\n    name: String,\n}\n",
        );

        let crawl = crawl(root);
        let input = ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
        let results = g3rs_garde_source_checks::check(&input);

        assertions::assert_rule_present(
            &results,
            "g3rs-garde/struct-derive-validate",
            "src/input.rs",
        );
    }

    #[test]
    fn pipeline_activates_for_manual_deserialize_adoption_without_garde_dependency() {
        let temp = new_root();
        let root = temp.path();

        write(
            root.join("Cargo.toml"),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
        );
        write(
            root.join("guardrail3-rs.toml"),
            "profile = \"service\"\n\n[checks]\ngarde = true\n",
        );
        write(
            root.join("src/input.rs"),
            "use serde::Deserialize;\n\nstruct Input {\n    name: String,\n}\n\nimpl<'de> Deserialize<'de> for Input {\n    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>\n    where\n        D: serde::Deserializer<'de>,\n    {\n        todo!()\n    }\n}\n",
        );

        let crawl = crawl(root);
        let input = ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
        let results = g3rs_garde_source_checks::check(&input);

        assertions::assert_rule_present(
            &results,
            "g3rs-garde/manual-deserialize-impl",
            "src/input.rs",
        );
    }

    #[test]
    fn pipeline_stays_quiet_for_manual_validate_without_explicit_enablement() {
        let temp = new_root();
        let root = temp.path();

        write(
            root.join("Cargo.toml"),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
        );
        write(
            root.join("src/validate.rs"),
            "struct GuardrailConfig;\n\nstruct Input;\n\nimpl garde::Validate for Input {\n    type Context = ();\n\n    fn validate_into(&self, _ctx: &Self::Context, _parent: &mut dyn FnMut(garde::Error)) {}\n}\n\nfn load_config(content: &str) -> Option<GuardrailConfig> {\n    toml::from_str(content).ok()\n}\n",
        );

        let crawl = crawl(root);
        let input = ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
        let results = g3rs_garde_source_checks::check(&input);

        assertions::assert_no_results(&results);
    }

    #[test]
    fn pipeline_stays_quiet_for_derived_validate_without_explicit_enablement() {
        let temp = new_root();
        let root = temp.path();

        write(
            root.join("Cargo.toml"),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
        );
        write(
            root.join("src/validate.rs"),
            "use garde::Validate;\n\nstruct GuardrailConfig;\n\n#[derive(Validate)]\nstruct Input;\n\nfn load_config(content: &str) -> Option<GuardrailConfig> {\n    toml::from_str(content).ok()\n}\n",
        );

        let crawl = crawl(root);
        let input = ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
        let results = g3rs_garde_source_checks::check(&input);

        assertions::assert_no_results(&results);
    }
}
