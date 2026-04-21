use g3ts_apparch_types::{
    G3TsApparchConfigChecksInput, G3TsApparchImportKind, G3TsApparchInternalEdge, G3TsApparchLayer,
    G3TsApparchSourceFile,
};

#[test]
fn config_checks_flag_types_importing_logic() {
    let input = G3TsApparchConfigChecksInput {
        files: vec![
            G3TsApparchSourceFile {
                rel_path: "src/types/user.ts".to_owned(),
                layer: G3TsApparchLayer::Types,
            },
            G3TsApparchSourceFile {
                rel_path: "src/logic/format_user.ts".to_owned(),
                layer: G3TsApparchLayer::Logic,
            },
        ],
        internal_edges: vec![G3TsApparchInternalEdge {
            from_rel_path: "src/types/user.ts".to_owned(),
            from_layer: G3TsApparchLayer::Types,
            to_rel_path: "src/logic/format_user.ts".to_owned(),
            to_layer: G3TsApparchLayer::Logic,
            kind: G3TsApparchImportKind::Import,
        }],
        external_imports: Vec::new(),
    };

    let results = crate::run::check(&input);
    g3ts_apparch_config_checks_assertions::run::assert_has_error(&results, "TS-APPARCH-CONFIG-01");
}

#[test]
fn config_checks_allow_logic_importing_types() {
    let input = G3TsApparchConfigChecksInput {
        files: vec![
            G3TsApparchSourceFile {
                rel_path: "src/logic/get_user.ts".to_owned(),
                layer: G3TsApparchLayer::Logic,
            },
            G3TsApparchSourceFile {
                rel_path: "src/types/user.ts".to_owned(),
                layer: G3TsApparchLayer::Types,
            },
        ],
        internal_edges: vec![G3TsApparchInternalEdge {
            from_rel_path: "src/logic/get_user.ts".to_owned(),
            from_layer: G3TsApparchLayer::Logic,
            to_rel_path: "src/types/user.ts".to_owned(),
            to_layer: G3TsApparchLayer::Types,
            kind: G3TsApparchImportKind::Import,
        }],
        external_imports: Vec::new(),
    };

    let results = crate::run::check(&input);
    g3ts_apparch_config_checks_assertions::run::assert_has_inventory(
        &results,
        "TS-APPARCH-CONFIG-02",
    );
}

#[test]
fn config_checks_flag_app_dynamic_importing_outbound() {
    let input = G3TsApparchConfigChecksInput {
        files: vec![
            G3TsApparchSourceFile {
                rel_path: "src/app/page.tsx".to_owned(),
                layer: G3TsApparchLayer::App,
            },
            G3TsApparchSourceFile {
                rel_path: "src/io/outbound/db.ts".to_owned(),
                layer: G3TsApparchLayer::IoOutbound,
            },
        ],
        internal_edges: vec![G3TsApparchInternalEdge {
            from_rel_path: "src/app/page.tsx".to_owned(),
            from_layer: G3TsApparchLayer::App,
            to_rel_path: "src/io/outbound/db.ts".to_owned(),
            to_layer: G3TsApparchLayer::IoOutbound,
            kind: G3TsApparchImportKind::DynamicImport,
        }],
        external_imports: Vec::new(),
    };

    let results = crate::run::check(&input);
    g3ts_apparch_config_checks_assertions::run::assert_has_error(&results, "TS-APPARCH-CONFIG-05");
}
