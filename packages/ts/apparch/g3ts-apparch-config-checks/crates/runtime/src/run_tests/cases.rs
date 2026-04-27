use g3ts_apparch_types::{
    G3TsApparchConfigChecksInput, G3TsApparchExternalImport, G3TsApparchImportKind,
    G3TsApparchInternalEdge, G3TsApparchLayer, G3TsApparchSourceFile,
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
    g3ts_apparch_config_checks_assertions::run::assert_has_error(
        &results,
        "g3ts-apparch/types-dependency-direction",
    );
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
        "g3ts-apparch/logic-dependency-direction",
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
    g3ts_apparch_config_checks_assertions::run::assert_has_error(
        &results,
        "g3ts-apparch/app-no-direct-outbound",
    );
}

#[test]
fn config_checks_flag_types_importing_next_runtime() {
    let input = G3TsApparchConfigChecksInput {
        files: vec![G3TsApparchSourceFile {
            rel_path: "src/types/user.ts".to_owned(),
            layer: G3TsApparchLayer::Types,
        }],
        internal_edges: Vec::new(),
        external_imports: vec![G3TsApparchExternalImport {
            from_rel_path: "src/types/user.ts".to_owned(),
            from_layer: G3TsApparchLayer::Types,
            module_name: "next/navigation".to_owned(),
            kind: G3TsApparchImportKind::Import,
        }],
    };

    let results = crate::run::check(&input);
    g3ts_apparch_config_checks_assertions::run::assert_has_error(
        &results,
        "g3ts-apparch/types-purity",
    );
}

#[test]
fn config_checks_flag_logic_importing_react_runtime() {
    let input = G3TsApparchConfigChecksInput {
        files: vec![G3TsApparchSourceFile {
            rel_path: "src/logic/get_user.ts".to_owned(),
            layer: G3TsApparchLayer::Logic,
        }],
        internal_edges: Vec::new(),
        external_imports: vec![G3TsApparchExternalImport {
            from_rel_path: "src/logic/get_user.ts".to_owned(),
            from_layer: G3TsApparchLayer::Logic,
            module_name: "react".to_owned(),
            kind: G3TsApparchImportKind::Import,
        }],
    };

    let results = crate::run::check(&input);
    g3ts_apparch_config_checks_assertions::run::assert_has_error(
        &results,
        "g3ts-apparch/logic-purity",
    );
}

#[test]
fn config_checks_flag_logic_importing_react_runtime_subpath() {
    let input = G3TsApparchConfigChecksInput {
        files: vec![G3TsApparchSourceFile {
            rel_path: "src/logic/get_user.ts".to_owned(),
            layer: G3TsApparchLayer::Logic,
        }],
        internal_edges: Vec::new(),
        external_imports: vec![G3TsApparchExternalImport {
            from_rel_path: "src/logic/get_user.ts".to_owned(),
            from_layer: G3TsApparchLayer::Logic,
            module_name: "react/jsx-runtime".to_owned(),
            kind: G3TsApparchImportKind::Import,
        }],
    };

    let results = crate::run::check(&input);
    g3ts_apparch_config_checks_assertions::run::assert_has_error(
        &results,
        "g3ts-apparch/logic-purity",
    );
}

#[test]
fn config_checks_allow_logic_importing_non_framework_external() {
    let input = G3TsApparchConfigChecksInput {
        files: vec![G3TsApparchSourceFile {
            rel_path: "src/logic/get_user.ts".to_owned(),
            layer: G3TsApparchLayer::Logic,
        }],
        internal_edges: Vec::new(),
        external_imports: vec![G3TsApparchExternalImport {
            from_rel_path: "src/logic/get_user.ts".to_owned(),
            from_layer: G3TsApparchLayer::Logic,
            module_name: "zod".to_owned(),
            kind: G3TsApparchImportKind::Import,
        }],
    };

    let results = crate::run::check(&input);
    g3ts_apparch_config_checks_assertions::run::assert_has_inventory(
        &results,
        "g3ts-apparch/logic-purity",
    );
}
