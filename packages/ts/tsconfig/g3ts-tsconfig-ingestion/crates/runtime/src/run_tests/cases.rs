#![expect(
    clippy::too_many_lines,
    reason = "Test cases inline their fixture construction so each test reads top-to-bottom; \
              extracting fixture builders would require shared mutable state and obscure the \
              one-test-one-scenario architecture"
)]
#![expect(
    clippy::panic,
    reason = "Test cases panic to surface unexpected fixture state; this is the standard \
              pattern for variant assertions inside #[test] functions"
)]
#![expect(
    clippy::disallowed_methods,
    reason = "Test fixtures write into tempdirs via std::fs; routing through the production \
              fs port would require the test sidecar to call a sibling module, which is \
              forbidden by the runtime-assertions-split rule"
)]

use std::path::Path;

use g3_workspace_crawl::crawl_any_root as crawl;
use g3ts_tsconfig_ingestion_assertions::run::{
    assert_effective_flags, assert_missing, assert_parse_error, assert_parsed_root_rel_path,
};
use g3ts_tsconfig_types::{G3TsTsconfigExtendsState, G3TsTsconfigState};

#[test]
fn returns_missing_when_no_root_tsconfig_surface_exists() {
    let tempdir = tempfile::tempdir().expect("tempdir should be created");
    let crawl = crawl(tempdir.path()).expect("crawl should succeed");

    let input = super::super::ingest_for_config_checks(&crawl);

    assert_missing(&input);
}

#[test]
fn falls_back_to_root_tsconfig_base_when_root_tsconfig_is_absent() {
    let tempdir = tempfile::tempdir().expect("tempdir should be created");
    let root = tempdir.path();
    super::helpers::write(
        root,
        "tsconfig.base.json",
        r#"
        {
          "compilerOptions": {
            "strict": true,
            "noImplicitReturns": true,
            "noUnusedLocals": true,
            "noUnusedParameters": true,
            "noUncheckedIndexedAccess": true,
            "exactOptionalPropertyTypes": true,
            "noPropertyAccessFromIndexSignature": true,
            "noImplicitOverride": true,
            "noFallthroughCasesInSwitch": true,
            "forceConsistentCasingInFileNames": true,
            "allowUnreachableCode": false,
            "allowUnusedLabels": false
          }
        }
        "#,
    );

    let crawl = crawl(root).expect("crawl should succeed");
    let input = super::super::ingest_for_config_checks(&crawl);

    assert_parsed_root_rel_path(&input, "tsconfig.base.json");
    assert_effective_flags(
        &input,
        &[
            ("strict", Some(true)),
            ("noImplicitReturns", Some(true)),
            ("noUnusedLocals", Some(true)),
            ("noUnusedParameters", Some(true)),
            ("allowUnreachableCode", Some(false)),
        ],
    );
}

#[test]
fn prefers_root_tsconfig_when_both_root_surfaces_exist() {
    let tempdir = tempfile::tempdir().expect("tempdir should be created");
    let root = tempdir.path();
    super::helpers::write(
        root,
        "tsconfig.json",
        r#"
        {
          "compilerOptions": {
            "strict": true,
            "noImplicitReturns": true,
            "noUnusedLocals": true,
            "noUnusedParameters": true,
            "noUncheckedIndexedAccess": true,
            "exactOptionalPropertyTypes": true,
            "noPropertyAccessFromIndexSignature": true,
            "noImplicitOverride": true,
            "noFallthroughCasesInSwitch": true,
            "forceConsistentCasingInFileNames": true,
            "allowUnreachableCode": false,
            "allowUnusedLabels": false
          }
        }
        "#,
    );
    super::helpers::write(
        root,
        "tsconfig.base.json",
        r#"
        {
          "compilerOptions": {
            "strict": false
          }
        }
        "#,
    );

    let crawl = crawl(root).expect("crawl should succeed");
    let input = super::super::ingest_for_config_checks(&crawl);

    assert_parsed_root_rel_path(&input, "tsconfig.json");
    assert_effective_flags(&input, &[("strict", Some(true))]);
}

#[test]
fn parses_root_and_relative_base_chain() {
    let tempdir = tempfile::tempdir().expect("tempdir should be created");
    let root = tempdir.path();
    super::helpers::write(
        root,
        "tsconfig.json",
        r#"
        {
          "extends": "../tsconfig.base.json",
          "compilerOptions": {
            "noUnusedLocals": true
          }
        }
        "#,
    );
    write_parent_file(
        root,
        "../tsconfig.base.json",
        r#"
        {
          "compilerOptions": {
            "strict": true,
            "noImplicitReturns": true,
            "noUnusedParameters": true,
            "noUncheckedIndexedAccess": true,
            "exactOptionalPropertyTypes": true,
            "isolatedModules": true,
            "noPropertyAccessFromIndexSignature": true,
            "noImplicitOverride": true,
            "noFallthroughCasesInSwitch": true,
            "forceConsistentCasingInFileNames": true,
            "allowUnreachableCode": false,
            "allowUnusedLabels": false
          }
        }
        "#,
    );

    let crawl = crawl(root).expect("crawl should succeed");
    let input = super::super::ingest_for_config_checks(&crawl);

    assert_parsed_root_rel_path(&input, "tsconfig.json");
    assert_effective_flags(
        &input,
        &[
            ("strict", Some(true)),
            ("noImplicitReturns", Some(true)),
            ("noUnusedLocals", Some(true)),
            ("noUnusedParameters", Some(true)),
            ("allowUnreachableCode", Some(false)),
        ],
    );
}

#[test]
fn surfaces_parse_error_for_invalid_root_tsconfig() {
    let tempdir = tempfile::tempdir().expect("tempdir should be created");
    super::helpers::write(tempdir.path(), "tsconfig.json", "{ invalid ");

    let crawl = crawl(tempdir.path()).expect("crawl should succeed");
    let input = super::super::ingest_for_config_checks(&crawl);

    assert_parse_error(&input, "tsconfig.json");
}

#[test]
fn shared_parent_in_extends_array_is_not_treated_as_circular() {
    let tempdir = tempfile::tempdir().expect("tempdir should be created");
    let root = tempdir.path();
    super::helpers::write(
        root,
        "tsconfig.json",
        r#"
        {
          "extends": ["./strict.json", "./app.json"]
        }
        "#,
    );
    super::helpers::write(
        root,
        "strict.json",
        r#"
        {
          "extends": "./base.json",
          "compilerOptions": {
            "strict": true,
            "noImplicitReturns": true,
            "noUnusedParameters": true,
            "noUncheckedIndexedAccess": true,
            "exactOptionalPropertyTypes": true,
            "noPropertyAccessFromIndexSignature": true,
            "noImplicitOverride": true,
            "noFallthroughCasesInSwitch": true,
            "forceConsistentCasingInFileNames": true,
            "allowUnreachableCode": false,
            "allowUnusedLabels": false
          }
        }
        "#,
    );
    super::helpers::write(
        root,
        "app.json",
        r#"
        {
          "extends": "./base.json",
          "compilerOptions": {
            "noUnusedLocals": true
          }
        }
        "#,
    );
    super::helpers::write(
        root,
        "base.json",
        r#"
        {
          "compilerOptions": {
            "isolatedModules": true
          }
        }
        "#,
    );

    let crawl = crawl(root).expect("crawl should succeed");
    let input = super::super::ingest_for_config_checks(&crawl);

    assert_parsed_root_rel_path(&input, "tsconfig.json");
    assert_effective_flags(
        &input,
        &[
            ("strict", Some(true)),
            ("noImplicitReturns", Some(true)),
            ("noUnusedLocals", Some(true)),
            ("noUnusedParameters", Some(true)),
            ("allowUnreachableCode", Some(false)),
        ],
    );
    let G3TsTsconfigState::Parsed { extends_chain, .. } = &input.config else {
        panic!("expected parsed tsconfig input");
    };
    assert_eq!(
        extends_chain.len(),
        4,
        "both branches and the shared parent should remain visible in the chain"
    );
    assert!(
        extends_chain
            .iter()
            .all(|entry| matches!(entry, G3TsTsconfigExtendsState::Resolved { .. })),
        "valid shared-parent extends arrays must not produce synthetic chain errors: {extends_chain:?}"
    );
}

fn write_parent_file(root: &Path, relative_parent: &str, contents: &str) {
    let abs_path = root.join(relative_parent);
    if let Some(parent) = abs_path.parent() {
        std::fs::create_dir_all(parent).expect("parent directories should be created");
    }
    std::fs::write(abs_path, contents).expect("parent config should be written");
}
