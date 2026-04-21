use g3_workspace_crawl::crawl;

#[test]
fn returns_missing_when_root_package_json_is_absent() {
    let tempdir = tempfile::tempdir().expect("tempdir should be created");
    super::helpers::write(
        tempdir.path(),
        "pnpm-workspace.yaml",
        "packages:\n  - apps/*\n",
    );
    let crawl = crawl(tempdir.path()).expect("crawl should succeed");

    let input = super::super::ingest_for_config_checks(&crawl);

    g3ts_package_ingestion_assertions::run::assert_root_missing(&input);
}

#[test]
fn parses_root_and_local_manifests() {
    let tempdir = tempfile::tempdir().expect("tempdir should be created");
    let root = tempdir.path();
    super::helpers::write(root, "pnpm-workspace.yaml", "packages:\n  - apps/*\n");
    super::helpers::write(
        root,
        "package.json",
        r#"
        {
          "private": true,
          "packageManager": "pnpm@10.32.0",
          "engines": {
            "node": ">=24",
            "pnpm": "10"
          },
          "scripts": {
            "preinstall": "npx only-allow pnpm",
            "prepare": "echo prepare",
            "lint": "eslint .",
            "typecheck": "tsc --noEmit"
          },
          "pnpm": {
            "overrides": {
              "@eslint/js": "^9.0.0",
              "zod": "^4.0.0"
            },
            "onlyBuiltDependencies": ["esbuild"]
          }
        }
        "#,
    );
    super::helpers::write(
        root,
        "apps/web/package.json",
        r#"
        {
          "dependencies": {
            "react": "^19.0.0"
          }
        }
        "#,
    );

    let crawl = crawl(root).expect("crawl should succeed");
    let input = super::super::ingest_for_config_checks(&crawl);

    g3ts_package_ingestion_assertions::run::assert_root_parsed(&input, "package.json");
    g3ts_package_ingestion_assertions::run::assert_local_paths(&input, &["apps/web/package.json"]);
    g3ts_package_ingestion_assertions::run::assert_local_dependency_names(
        &input,
        "apps/web/package.json",
        &["react"],
    );
}

#[test]
fn surfaces_parse_error_for_invalid_root_package_json() {
    let tempdir = tempfile::tempdir().expect("tempdir should be created");
    super::helpers::write(
        tempdir.path(),
        "pnpm-workspace.yaml",
        "packages:\n  - apps/*\n",
    );
    super::helpers::write(tempdir.path(), "package.json", "{ invalid ");

    let crawl = crawl(tempdir.path()).expect("crawl should succeed");
    let input = super::super::ingest_for_config_checks(&crawl);

    g3ts_package_ingestion_assertions::run::assert_root_parse_error(&input, "package.json");
}

#[test]
fn surfaces_parse_error_for_invalid_local_package_json() {
    let tempdir = tempfile::tempdir().expect("tempdir should be created");
    let root = tempdir.path();
    super::helpers::write(root, "pnpm-workspace.yaml", "packages:\n  - apps/*\n");
    super::helpers::write(root, "package.json", r#"{ "private": true }"#);
    super::helpers::write(root, "apps/web/package.json", "{ invalid ");

    let crawl = crawl(root).expect("crawl should succeed");
    let input = super::super::ingest_for_config_checks(&crawl);

    g3ts_package_ingestion_assertions::run::assert_root_parsed(&input, "package.json");
    g3ts_package_ingestion_assertions::run::assert_local_paths(&input, &["apps/web/package.json"]);
    g3ts_package_ingestion_assertions::run::assert_local_parse_error(
        &input,
        "apps/web/package.json",
    );
}

#[test]
fn treats_explicit_non_workspace_root_manifest_as_local_only() {
    let tempdir = tempfile::tempdir().expect("tempdir should be created");
    super::helpers::write(
        tempdir.path(),
        "package.json",
        r#"
        {
          "private": true,
          "dependencies": {
            "react": "^19.0.0"
          }
        }
        "#,
    );

    let crawl = crawl(tempdir.path()).expect("crawl should succeed");
    let input = super::super::ingest_for_config_checks(&crawl);

    g3ts_package_ingestion_assertions::run::assert_root_not_package_manager_root(&input);
    g3ts_package_ingestion_assertions::run::assert_local_paths(&input, &["package.json"]);
}
