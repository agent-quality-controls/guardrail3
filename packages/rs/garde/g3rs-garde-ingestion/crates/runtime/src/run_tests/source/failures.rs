use g3rs_garde_ingestion_assertions::run as assertions;

#[test]
fn ast_ingestion_allows_missing_rust_policy_when_garde_is_present() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\ngarde = \"0.22\"\n",
    );
    super::write(root.join("src/lib.rs"), "fn ok() {}\n");

    let crawl = super::crawl(root);
    let result = super::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");

    assert!(result.input_failures.is_empty(), "{result:#?}");
    assert_eq!(result.struct_targets.len(), 0, "{result:#?}");
}

#[test]
fn ast_ingestion_fails_when_cargo_is_missing() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(root.join("src/lib.rs"), "fn ok() {}\n");

    let crawl = super::crawl(root);
    let result = super::ingest_for_source_checks(&crawl);

    assert!(
        matches!(result, Err(super::IngestionError::CargoTomlNotFound)),
        "{result:#?}"
    );
}

#[test]
fn ast_ingestion_fails_when_cargo_is_malformed() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(root.join("Cargo.toml"), "{{{{not valid toml}}}}");
    super::write(root.join("src/lib.rs"), "fn ok() {}\n");

    let crawl = super::crawl(root);
    let result = super::ingest_for_source_checks(&crawl);

    assert!(
        matches!(result, Err(super::IngestionError::ParseFailed { .. })),
        "{result:#?}"
    );
}

#[cfg(unix)]
#[test]
fn ast_ingestion_fails_when_cargo_is_unreadable() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\ngarde = \"0.22\"\n",
    );
    super::write(root.join("src/lib.rs"), "fn ok() {}\n");
    super::make_unreadable(&root.join("Cargo.toml"));

    let crawl = super::crawl(root);
    let result = super::ingest_for_source_checks(&crawl);

    assert!(
        matches!(result, Err(super::IngestionError::Unreadable { .. })),
        "{result:#?}"
    );
}

#[test]
fn pipeline_reports_malformed_source_via_garde_10() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\ngarde = \"0.22\"\n",
    );
    super::write(root.join("src/lib.rs"), "fn broken( {\n");

    let crawl = super::crawl(root);
    let input = super::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    let results = g3rs_garde_source_checks::check(&input);

    assertions::assert_rule_present(&results, "RS-GARDE-SOURCE-10", "src/lib.rs");
}

#[test]
fn pipeline_reports_malformed_guardrail_via_garde_10() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\ngarde = \"0.22\"\n",
    );
    super::write(root.join("guardrail3-rs.toml"), "[[broken");
    super::write(
        root.join("src/lib.rs"),
        "use garde::Validate;\nuse sqlx::query_as;\n\n#[derive(Validate)]\nstruct Input;\n\nfn load() { let _ = query_as!(User, \"select 1\"); }\n",
    );

    let crawl = super::crawl(root);
    let input = super::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    let results = g3rs_garde_source_checks::check(&input);

    assertions::assert_rule_present(&results, "RS-GARDE-SOURCE-10", "guardrail3-rs.toml");
    assertions::assert_rule_id_absent(&results, "RS-GARDE-SOURCE-04");
}

#[cfg(unix)]
#[test]
fn pipeline_reports_unreadable_source_via_garde_10() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\ngarde = \"0.22\"\n",
    );
    super::write(root.join("src/lib.rs"), "fn ok() {}\n");
    super::make_unreadable(&root.join("src/lib.rs"));

    let crawl = super::crawl(root);
    let input = super::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    let results = g3rs_garde_source_checks::check(&input);

    assertions::assert_rule_present(&results, "RS-GARDE-SOURCE-10", "src/lib.rs");
}

#[cfg(unix)]
#[test]
fn pipeline_reports_unreadable_guardrail_via_garde_10() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\ngarde = \"0.22\"\n",
    );
    super::write(root.join("guardrail3-rs.toml"), "profile = \"service\"\n");
    super::write(
        root.join("src/lib.rs"),
        "use garde::Validate;\n#[derive(Validate)]\nstruct Input;\n",
    );
    super::make_unreadable(&root.join("guardrail3-rs.toml"));

    let crawl = super::crawl(root);
    let input = super::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    let results = g3rs_garde_source_checks::check(&input);

    assertions::assert_rule_present(&results, "RS-GARDE-SOURCE-10", "guardrail3-rs.toml");
    assertions::assert_rule_id_absent(&results, "RS-GARDE-SOURCE-04");
}
