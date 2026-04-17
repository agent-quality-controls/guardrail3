use g3rs_garde_ingestion_assertions::run as assertions;

#[test]
fn pipeline_stays_quiet_for_clean_garde_root() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\ngarde = \"0.22\"\n",
    );
    super::write(
        root.join("src/lib.rs"),
        "use garde::Validate;\nuse serde::Deserialize;\n\n#[derive(Deserialize, Validate)]\nstruct Input {\n    #[garde(length(min = 1))]\n    name: String,\n}\n",
    );

    let crawl = super::crawl(root);
    let input = super::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    let results = g3rs_garde_source_checks::check(&input);

    assertions::assert_no_results(&results);
}

#[test]
fn pipeline_can_report_input_failures_and_ast_findings_together() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\ngarde = \"0.22\"\n",
    );
    super::write(root.join("src/broken.rs"), "fn broken( {\n");
    super::write(
        root.join("src/input.rs"),
        "use serde::Deserialize;\n\n#[derive(Deserialize)]\nstruct Input {\n    name: String,\n}\n",
    );

    let crawl = super::crawl(root);
    let input = super::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    let results = g3rs_garde_source_checks::check(&input);

    assertions::assert_rule_present(&results, "RS-GARDE-SOURCE-10", "src/broken.rs");
    assertions::assert_rule_present(&results, "RS-GARDE-SOURCE-01", "src/input.rs");
}

#[test]
fn pipeline_ignores_legacy_guardrail_config_parse_sites() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\ngarde = \"0.22\"\n",
    );
    super::write(
        root.join("src/load_config.rs"),
        "struct GuardrailConfig;\n\nfn load_config(content: &str) -> Option<GuardrailConfig> {\n    toml::from_str(content).ok()\n}\n",
    );

    let crawl = super::crawl(root);
    let input = super::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    let results = g3rs_garde_source_checks::check(&input);

    assertions::assert_no_results(&results);
}

#[test]
fn pipeline_uses_rust_policy_waivers_for_query_as() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    );
    super::write(
        root.join("guardrail3-rs.toml"),
        "profile = \"service\"\n\n[checks]\ngarde = true\n\n[[waivers]]\nrule = \"RS-GARDE-SOURCE-04\"\nfile = \"src/db.rs\"\nselector = \"qa@L4\"\nreason = \"Temporary SQLx row mapping until validated DTO extraction lands.\"\n",
    );
    super::write(
        root.join("src/db.rs"),
        "use sqlx::query_as as qa;\n\nfn load() {\n    let _row = qa!(User, \"select 1\");\n}\n",
    );

    let crawl = super::crawl(root);
    let input = super::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    let results = g3rs_garde_source_checks::check(&input);

    assertions::assert_rule_present(&results, "RS-GARDE-SOURCE-04", "src/db.rs");
    assertions::assert_rule_absent(&results, "RS-GARDE-SOURCE-04", "sqlx query_as missing reason");
}
