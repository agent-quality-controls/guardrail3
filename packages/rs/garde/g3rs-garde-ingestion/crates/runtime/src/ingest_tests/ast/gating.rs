#[test]
fn pipeline_stays_quiet_for_non_garde_root_without_adoption_markers() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    );
    super::write(root.join("guardrail3.toml"), "[profile]\nname = \"service\"\n");
    super::write(
        root.join("src/load_config.rs"),
        "use guardrail3_domain_config::types::GuardrailConfig;\n\nfn load_config(content: &str) -> Option<GuardrailConfig> {\n    toml::from_str(content).ok()\n}\n",
    );

    let crawl = super::crawl(root);
    let input = crate::ingest_for_ast_checks(&crawl).expect("AST ingestion should succeed");
    let results = g3rs_garde_ast_checks::check(&input);

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn pipeline_activates_for_source_adoption_markers_without_garde_dependency() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    );
    super::write(root.join("guardrail3.toml"), "[profile]\nname = \"service\"\n");
    super::write(
        root.join("src/input.rs"),
        "use serde::Deserialize;\n\n#[derive(Deserialize)]\nstruct Input {\n    name: String,\n}\n",
    );

    let crawl = super::crawl(root);
    let input = crate::ingest_for_ast_checks(&crawl).expect("AST ingestion should succeed");
    let results = g3rs_garde_ast_checks::check(&input);

    assert!(
        results.iter().any(|result| result.id() == "RS-GARDE-AST-01"),
        "{results:#?}"
    );
}

#[test]
fn pipeline_activates_for_manual_deserialize_adoption_without_garde_dependency() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    );
    super::write(root.join("guardrail3.toml"), "[profile]\nname = \"service\"\n");
    super::write(
        root.join("src/input.rs"),
        "use serde::Deserialize;\n\nstruct Input {\n    name: String,\n}\n\nimpl<'de> Deserialize<'de> for Input {\n    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>\n    where\n        D: serde::Deserializer<'de>,\n    {\n        todo!()\n    }\n}\n",
    );

    let crawl = super::crawl(root);
    let input = crate::ingest_for_ast_checks(&crawl).expect("AST ingestion should succeed");
    let results = g3rs_garde_ast_checks::check(&input);

    assert!(
        results
            .iter()
            .any(|result| result.id() == "RS-GARDE-AST-02" && result.file() == Some("src/input.rs")),
        "{results:#?}"
    );
}

#[test]
fn pipeline_activates_for_manual_validate_adoption_without_garde_dependency() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    );
    super::write(root.join("guardrail3.toml"), "[profile]\nname = \"service\"\n");
    super::write(
        root.join("src/validate.rs"),
        "use guardrail3_domain_config::types::GuardrailConfig;\n\nstruct Input;\n\nimpl garde::Validate for Input {\n    type Context = ();\n\n    fn validate_into(&self, _ctx: &Self::Context, _parent: &mut dyn FnMut(garde::Error)) {}\n}\n\nfn load_config(content: &str) -> Option<GuardrailConfig> {\n    toml::from_str(content).ok()\n}\n",
    );

    let crawl = super::crawl(root);
    let input = crate::ingest_for_ast_checks(&crawl).expect("AST ingestion should succeed");
    let results = g3rs_garde_ast_checks::check(&input);

    assert!(
        results
            .iter()
            .any(|result| result.id() == "RS-GARDE-AST-08" && result.file() == Some("src/validate.rs")),
        "{results:#?}"
    );
}

#[test]
fn pipeline_activates_for_derived_validate_adoption_without_garde_dependency() {
    let temp = super::new_root();
    let root = temp.path();

    super::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    );
    super::write(root.join("guardrail3.toml"), "[profile]\nname = \"service\"\n");
    super::write(
        root.join("src/validate.rs"),
        "use garde::Validate;\nuse guardrail3_domain_config::types::GuardrailConfig;\n\n#[derive(Validate)]\nstruct Input;\n\nfn load_config(content: &str) -> Option<GuardrailConfig> {\n    toml::from_str(content).ok()\n}\n",
    );

    let crawl = super::crawl(root);
    let input = crate::ingest_for_ast_checks(&crawl).expect("AST ingestion should succeed");
    let results = g3rs_garde_ast_checks::check(&input);

    assert!(
        results
            .iter()
            .any(|result| result.id() == "RS-GARDE-AST-08" && result.file() == Some("src/validate.rs")),
        "{results:#?}"
    );
}
