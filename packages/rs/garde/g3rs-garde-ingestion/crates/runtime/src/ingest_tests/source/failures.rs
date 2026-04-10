#[test]
fn ast_ingestion_fails_when_guardrail_is_missing() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    );
    super::write(root.join("src/lib.rs"), "fn ok() {}\n");

    let crawl = super::crawl(root);
    let result = crate::ingest_for_source_checks(&crawl);

    assert!(
        matches!(result, Err(crate::IngestionError::GuardrailTomlNotFound)),
        "{result:#?}"
    );
}

#[test]
fn ast_ingestion_fails_when_cargo_is_missing() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(root.join("guardrail3.toml"), "[profile]\nname = \"service\"\n");
    super::write(root.join("src/lib.rs"), "fn ok() {}\n");

    let crawl = super::crawl(root);
    let result = crate::ingest_for_source_checks(&crawl);

    assert!(
        matches!(result, Err(crate::IngestionError::CargoTomlNotFound)),
        "{result:#?}"
    );
}

#[test]
fn ast_ingestion_fails_when_cargo_is_malformed() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(root.join("Cargo.toml"), "{{{{not valid toml}}}}");
    super::write(root.join("guardrail3.toml"), "[profile]\nname = \"service\"\n");
    super::write(root.join("src/lib.rs"), "fn ok() {}\n");

    let crawl = super::crawl(root);
    let result = crate::ingest_for_source_checks(&crawl);

    assert!(
        matches!(result, Err(crate::IngestionError::ParseFailed { .. })),
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
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    );
    super::write(root.join("guardrail3.toml"), "[profile]\nname = \"service\"\n");
    super::write(root.join("src/lib.rs"), "fn ok() {}\n");
    super::make_unreadable(&root.join("Cargo.toml"));

    let crawl = super::crawl(root);
    let result = crate::ingest_for_source_checks(&crawl);

    assert!(
        matches!(result, Err(crate::IngestionError::Unreadable { .. })),
        "{result:#?}"
    );
}

#[test]
fn pipeline_reports_malformed_source_via_garde_10() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    );
    super::write(root.join("guardrail3.toml"), "[profile]\nname = \"service\"\n");
    super::write(root.join("src/lib.rs"), "fn broken( {\n");

    let crawl = super::crawl(root);
    let input = crate::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    let results = g3rs_garde_source_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-10" && result.file() == Some("src/lib.rs")
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_reports_malformed_guardrail_via_garde_10() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    );
    super::write(root.join("guardrail3.toml"), "[[broken");
    super::write(
        root.join("src/lib.rs"),
        "use sqlx::query_as;\nfn load() { let _ = query_as!(User, \"select 1\"); }\n",
    );

    let crawl = super::crawl(root);
    let input = crate::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    let results = g3rs_garde_source_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-10" && result.file() == Some("guardrail3.toml")
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().all(|result| result.id() != "RS-GARDE-AST-04"),
        "{results:#?}"
    );
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
    super::write(root.join("guardrail3.toml"), "[profile]\nname = \"service\"\n");
    super::write(root.join("src/lib.rs"), "fn ok() {}\n");
    super::make_unreadable(&root.join("src/lib.rs"));

    let crawl = super::crawl(root);
    let input = crate::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    let results = g3rs_garde_source_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-10" && result.file() == Some("src/lib.rs")
        }),
        "{results:#?}"
    );
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
    super::write(root.join("guardrail3.toml"), "[profile]\nname = \"service\"\n");
    super::write(root.join("src/lib.rs"), "fn ok() {}\n");
    super::make_unreadable(&root.join("guardrail3.toml"));

    let crawl = super::crawl(root);
    let input = crate::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    let results = g3rs_garde_source_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-10" && result.file() == Some("guardrail3.toml")
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().all(|result| result.id() != "RS-GARDE-AST-04"),
        "{results:#?}"
    );
}
