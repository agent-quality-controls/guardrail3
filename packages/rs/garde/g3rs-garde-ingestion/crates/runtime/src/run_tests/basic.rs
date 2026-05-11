use g3rs_garde_types::{G3RsGardeApplicability, G3RsGardeClippyInput};

#[test]
fn ingests_with_both_cargo_and_clippy() {
    let temp = super::helpers::new_root();
    let root = temp.path();

    super::helpers::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = []\nresolver = \"2\"\n[workspace.dependencies]\ngarde = \"0.22\"\n",
    );
    super::helpers::write(root.join("clippy.toml"), "msrv = \"1.85\"\n");

    let crawl = super::helpers::crawl(root);
    let result = super::ingest_for_config_checks(&crawl);

    let input =
        result.expect("ingestion should succeed when both Cargo.toml and clippy.toml are present");
    assert_eq!(
        input.cargo_rel_path, "Cargo.toml",
        "cargo_rel_path should reference the root Cargo.toml"
    );
    assert_eq!(input.applicability, G3RsGardeApplicability::Active);
    assert!(
        input.cargo.workspace.is_some(),
        "parsed Cargo.toml should contain a [workspace] section"
    );
    assert!(
        matches!(
            input.clippy_input,
            G3RsGardeClippyInput::Parsed { ref rel_path, .. } if rel_path == "clippy.toml"
        ),
        "clippy_input should preserve the parsed root clippy.toml"
    );
}

#[test]
fn ingests_with_dot_clippy_toml() {
    let temp = super::helpers::new_root();
    let root = temp.path();

    super::helpers::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\n[dependencies]\ngarde = \"0.22\"\n",
    );
    super::helpers::write(root.join(".clippy.toml"), "msrv = \"1.85\"\n");

    let crawl = super::helpers::crawl(root);
    let result = super::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should succeed with .clippy.toml variant");
    assert_eq!(input.applicability, G3RsGardeApplicability::Active);
    assert!(
        matches!(
            input.clippy_input,
            G3RsGardeClippyInput::Parsed { ref rel_path, .. } if rel_path == ".clippy.toml"
        ),
        "clippy_input should reference .clippy.toml when only the dotfile variant exists"
    );
}

#[test]
fn clippy_is_missing_without_clippy_config() {
    let temp = super::helpers::new_root();
    let root = temp.path();

    super::helpers::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\n[dependencies]\ngarde = \"0.22\"\n",
    );

    let crawl = super::helpers::crawl(root);
    let result = super::ingest_for_config_checks(&crawl);

    let input =
        result.expect("ingestion should succeed even without clippy config (it is optional)");
    assert_eq!(input.applicability, G3RsGardeApplicability::Active);
    assert_eq!(
        input.cargo_rel_path, "Cargo.toml",
        "cargo_rel_path should still be present without clippy config"
    );
    assert!(
        matches!(input.clippy_input, G3RsGardeClippyInput::Missing),
        "clippy_input should be Missing when no clippy config file exists"
    );
}

#[test]
fn malformed_clippy_toml_is_preserved_for_package_warnings() {
    let temp = super::helpers::new_root();
    let root = temp.path();

    super::helpers::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = []\n[workspace.dependencies]\ngarde = \"0.22\"\n",
    );
    super::helpers::write(root.join("clippy.toml"), "{{{{not valid toml}}}}");

    let crawl = super::helpers::crawl(root);
    let result = super::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should preserve invalid clippy for package warnings");
    assert!(
        matches!(
            input.clippy_input,
            G3RsGardeClippyInput::Invalid { ref rel_path, .. } if rel_path == "clippy.toml"
        ),
        "invalid clippy input should still carry its path"
    );
}

#[test]
fn fails_when_cargo_toml_is_missing() {
    let temp = super::helpers::new_root();
    let root = temp.path();

    super::helpers::write(root.join("clippy.toml"), "msrv = \"1.85\"\n");

    let crawl = super::helpers::crawl(root);
    let result = super::ingest_for_config_checks(&crawl);

    assert!(
        matches!(result, Err(super::IngestionError::CargoTomlNotFound)),
        "ingestion should return CargoTomlNotFound when Cargo.toml is missing even if clippy.toml exists"
    );
}

#[test]
fn fails_on_malformed_cargo_toml() {
    let temp = super::helpers::new_root();
    let root = temp.path();

    super::helpers::write(root.join("Cargo.toml"), "{{{{not valid toml}}}}");
    super::helpers::write(root.join("clippy.toml"), "msrv = \"1.85\"\n");

    let crawl = super::helpers::crawl(root);
    let result = super::ingest_for_config_checks(&crawl);

    assert!(
        matches!(result, Err(super::IngestionError::ParseFailed { .. })),
        "ingestion should return ParseFailed when Cargo.toml contains invalid TOML"
    );
}

#[test]
fn ignored_but_recovered_cargo_toml_is_ingested() {
    let temp = super::helpers::new_root();
    let root = temp.path();

    super::helpers::write(root.join(".gitignore"), "Cargo.toml\n");
    super::helpers::write(root.join("Cargo.toml"), "[package]\nname = \"recovered\"\n");

    let crawl = super::helpers::crawl(root);

    let crawl_entry = g3_workspace_crawl::entry(&crawl, "Cargo.toml")
        .expect("Cargo.toml should be present in crawl via recovery even when gitignored");
    assert_eq!(
        crawl_entry.ignore_state,
        g3_workspace_crawl::G3WorkspaceIgnoreState::Ignored,
        "Cargo.toml should have Ignored state when gitignored, proving recovery path was exercised"
    );

    let result = super::ingest_for_config_checks(&crawl);
    let input = result.expect(
        "ingestion should succeed for a gitignored Cargo.toml recovered by the crawl recovery phase",
    );
    assert_eq!(
        input.cargo_rel_path, "Cargo.toml",
        "recovered Cargo.toml should still resolve to the root-relative path"
    );
}
