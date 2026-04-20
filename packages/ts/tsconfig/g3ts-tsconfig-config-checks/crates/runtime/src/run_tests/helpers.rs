use g3ts_tsconfig_types::{G3TsTsconfigChecksInput, G3TsTsconfigExtendsState, G3TsTsconfigState};
use tsconfig_json_parser::parse_document;
use tsconfig_json_parser::types::TsconfigCompilerOptions;

pub(super) fn missing() -> G3TsTsconfigChecksInput {
    G3TsTsconfigChecksInput {
        config: G3TsTsconfigState::Missing,
    }
}

pub(super) fn parse_error() -> G3TsTsconfigChecksInput {
    G3TsTsconfigChecksInput {
        config: G3TsTsconfigState::ParseError {
            rel_path: "tsconfig.json".to_owned(),
            reason: "synthetic parse failure".to_owned(),
        },
    }
}

pub(super) fn golden_extends() -> G3TsTsconfigChecksInput {
    let root = parse_document(r#"{ "extends": "../../tsconfig.base.json" }"#)
        .expect("root tsconfig should parse");
    let parent = parse_document(
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
    )
    .expect("parent tsconfig should parse");

    G3TsTsconfigChecksInput {
        config: G3TsTsconfigState::Parsed {
            rel_path: "tsconfig.json".to_owned(),
            document: root,
            extends_chain: vec![G3TsTsconfigExtendsState::Parsed {
                specifier: "../../tsconfig.base.json".to_owned(),
                display_path: "/tmp/tsconfig.base.json".to_owned(),
                document: parent,
            }],
            effective_compiler_options: strict_baseline(),
        },
    }
}

pub(super) fn broken_chain() -> G3TsTsconfigChecksInput {
    let root = parse_document(r#"{ "extends": "../../tsconfig.base.json" }"#)
        .expect("root tsconfig should parse");

    G3TsTsconfigChecksInput {
        config: G3TsTsconfigState::Parsed {
            rel_path: "tsconfig.json".to_owned(),
            document: root,
            extends_chain: vec![G3TsTsconfigExtendsState::Missing {
                specifier: "../../tsconfig.base.json".to_owned(),
                display_path: "/tmp/tsconfig.base.json".to_owned(),
            }],
            effective_compiler_options: TsconfigCompilerOptions::default(),
        },
    }
}

pub(super) fn standalone_missing_inline() -> G3TsTsconfigChecksInput {
    let root = parse_document(
        r#"
        {
          "compilerOptions": {
            "strict": true
          }
        }
        "#,
    )
    .expect("root tsconfig should parse");

    G3TsTsconfigChecksInput {
        config: G3TsTsconfigState::Parsed {
            rel_path: "tsconfig.json".to_owned(),
            document: root,
            extends_chain: Vec::new(),
            effective_compiler_options: TsconfigCompilerOptions {
                strict: Some(true),
                ..TsconfigCompilerOptions::default()
            },
        },
    }
}

pub(super) fn weak_effective_flags() -> G3TsTsconfigChecksInput {
    let root = parse_document(r#"{ "extends": "../../tsconfig.base.json" }"#)
        .expect("root tsconfig should parse");

    let mut effective = strict_baseline();
    effective.no_unused_locals = Some(false);

    G3TsTsconfigChecksInput {
        config: G3TsTsconfigState::Parsed {
            rel_path: "tsconfig.json".to_owned(),
            document: root,
            extends_chain: vec![G3TsTsconfigExtendsState::Parsed {
                specifier: "../../tsconfig.base.json".to_owned(),
                display_path: "/tmp/tsconfig.base.json".to_owned(),
                document: parse_document("{}").expect("parent doc should parse"),
            }],
            effective_compiler_options: effective,
        },
    }
}

pub(super) fn external_extends() -> G3TsTsconfigChecksInput {
    let root = parse_document(r#"{ "extends": "@tsconfig/strictest/tsconfig.json" }"#)
        .expect("root tsconfig should parse");

    G3TsTsconfigChecksInput {
        config: G3TsTsconfigState::Parsed {
            rel_path: "tsconfig.json".to_owned(),
            document: root,
            extends_chain: vec![G3TsTsconfigExtendsState::External {
                specifier: "@tsconfig/strictest/tsconfig.json".to_owned(),
            }],
            effective_compiler_options: TsconfigCompilerOptions::default(),
        },
    }
}

fn strict_baseline() -> TsconfigCompilerOptions {
    TsconfigCompilerOptions {
        strict: Some(true),
        no_implicit_returns: Some(true),
        no_unused_locals: Some(true),
        no_unused_parameters: Some(true),
        no_unchecked_indexed_access: Some(true),
        exact_optional_property_types: Some(true),
        isolated_modules: None,
        no_property_access_from_index_signature: Some(true),
        no_implicit_override: Some(true),
        no_fallthrough_cases_in_switch: Some(true),
        force_consistent_casing_in_file_names: Some(true),
        allow_unreachable_code: Some(false),
        allow_unused_labels: Some(false),
    }
}
