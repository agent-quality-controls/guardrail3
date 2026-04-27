use g3rs_garde_ingestion_assertions::run as assertions;

#[test]
fn pipeline_reports_missing_garde_dependency() {
    let temp = super::helpers::new_root();
    let root = temp.path();

    super::helpers::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    );
    super::helpers::write(
        root.join("guardrail3-rs.toml"),
        "profile = \"service\"\n\n[checks]\ngarde = true\n",
    );

    let crawl = super::helpers::crawl(root);
    let input = super::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_garde_config_checks::check(&input);

    assertions::assert_missing_garde_dependency(&results);
}

#[test]
fn pipeline_warns_when_clippy_is_missing_for_garde_root() {
    let temp = super::helpers::new_root();
    let root = temp.path();

    super::helpers::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = []\n[workspace.dependencies]\ngarde = \"0.22\"\n",
    );

    let crawl = super::helpers::crawl(root);
    let input = super::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_garde_config_checks::check(&input);

    assertions::assert_missing_clippy_config_warnings(&results);
}

#[test]
fn pipeline_keeps_ban_rules_quiet_when_garde_is_missing() {
    let temp = super::helpers::new_root();
    let root = temp.path();

    super::helpers::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = []\nversion = \"0.1.0\"\n",
    );
    super::helpers::write(
        root.join("clippy.toml"),
        "disallowed-methods = []\ndisallowed-types = []\n",
    );

    let crawl = super::helpers::crawl(root);
    let input = super::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_garde_config_checks::check(&input);

    assertions::assert_no_results(&results);
}

#[test]
fn pipeline_warns_when_clippy_is_invalid_for_garde_root() {
    let temp = super::helpers::new_root();
    let root = temp.path();

    super::helpers::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = []\n[workspace.dependencies]\ngarde = \"0.22\"\n",
    );
    super::helpers::write(root.join("clippy.toml"), "{{{{not valid toml}}}}");

    let crawl = super::helpers::crawl(root);
    let input = super::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_garde_config_checks::check(&input);

    assertions::assert_invalid_clippy_config_warnings(&results);
}

#[test]
fn pipeline_marks_family_inactive_when_no_garde_dependency_and_no_guardrail_toml() {
    let temp = super::helpers::new_root();
    let root = temp.path();

    super::helpers::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = []\n[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    super::helpers::write(root.join("src/lib.rs"), "pub fn run() {}\n");

    let crawl = super::helpers::crawl(root);

    let config_input =
        super::ingest_for_config_checks(&crawl).expect("config ingestion should succeed");
    let config_results = g3rs_garde_config_checks::check(&config_input);
    assertions::assert_no_results(&config_results);

    let source_input =
        super::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    let source_results = g3rs_garde_source_checks::check(&source_input);
    assertions::assert_no_results(&source_results);
}

#[test]
fn pipeline_does_not_false_positive_on_duplicate_simple_type_names() {
    let temp = super::helpers::new_root();
    let root = temp.path();

    super::helpers::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\n[dependencies]\ngarde = \"0.22\"\nserde = { version = \"1\", features = [\"derive\"] }\n",
    );
    super::helpers::write(
        root.join("src/a.rs"),
        "use garde::Validate;\nuse serde::Deserialize;\n\n#[derive(Deserialize, Validate)]\nstruct Input {\n    payload: Payload,\n}\n\nstruct Payload {\n    value: String,\n}\n",
    );
    super::helpers::write(
        root.join("src/z.rs"),
        "use garde::Validate;\n\n#[derive(Validate)]\nstruct Payload {\n    value: String,\n}\n",
    );

    let crawl = super::helpers::crawl(root);
    let input = super::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    let results = g3rs_garde_source_checks::check(&input);

    assertions::assert_rule_absent(
        &results,
        "g3rs-garde/nested-validation-dive",
        "nested dive false positive",
    );
}
