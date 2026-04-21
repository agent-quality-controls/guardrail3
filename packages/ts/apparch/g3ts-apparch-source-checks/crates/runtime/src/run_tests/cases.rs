use g3ts_apparch_types::{
    G3TsApparchLayer, G3TsApparchPublicItem, G3TsApparchPublicItemKind,
    G3TsApparchSourceChecksInput, G3TsApparchSourceFile,
};

#[test]
fn source_checks_flag_exported_function_in_types() {
    let input = G3TsApparchSourceChecksInput {
        files: vec![G3TsApparchSourceFile {
            rel_path: "src/types/index.ts".to_owned(),
            layer: G3TsApparchLayer::Types,
        }],
        public_items: vec![G3TsApparchPublicItem {
            rel_path: "src/types/index.ts".to_owned(),
            layer: G3TsApparchLayer::Types,
            item_name: "helper".to_owned(),
            kind: G3TsApparchPublicItemKind::Function,
            line: 2,
        }],
    };

    let results = crate::run::check(&input);
    g3ts_apparch_source_checks_assertions::run::assert_has_error(&results, "TS-APPARCH-SOURCE-01");
}

#[test]
fn source_checks_flag_exported_interface_in_io() {
    let input = G3TsApparchSourceChecksInput {
        files: vec![G3TsApparchSourceFile {
            rel_path: "src/io/outbound/db.ts".to_owned(),
            layer: G3TsApparchLayer::IoOutbound,
        }],
        public_items: vec![G3TsApparchPublicItem {
            rel_path: "src/io/outbound/db.ts".to_owned(),
            layer: G3TsApparchLayer::IoOutbound,
            item_name: "DbPort".to_owned(),
            kind: G3TsApparchPublicItemKind::Interface,
            line: 1,
        }],
    };

    let results = crate::run::check(&input);
    g3ts_apparch_source_checks_assertions::run::assert_has_error(&results, "TS-APPARCH-SOURCE-02");
}

#[test]
fn source_checks_flag_exported_class_in_types() {
    let input = G3TsApparchSourceChecksInput {
        files: vec![G3TsApparchSourceFile {
            rel_path: "src/types/index.ts".to_owned(),
            layer: G3TsApparchLayer::Types,
        }],
        public_items: vec![G3TsApparchPublicItem {
            rel_path: "src/types/index.ts".to_owned(),
            layer: G3TsApparchLayer::Types,
            item_name: "UserView".to_owned(),
            kind: G3TsApparchPublicItemKind::Class,
            line: 3,
        }],
    };

    let results = crate::run::check(&input);
    g3ts_apparch_source_checks_assertions::run::assert_has_error(&results, "TS-APPARCH-SOURCE-01");
}

#[test]
fn source_checks_flag_exported_interface_in_io_inbound() {
    let input = G3TsApparchSourceChecksInput {
        files: vec![G3TsApparchSourceFile {
            rel_path: "src/io/inbound/http.ts".to_owned(),
            layer: G3TsApparchLayer::IoInbound,
        }],
        public_items: vec![G3TsApparchPublicItem {
            rel_path: "src/io/inbound/http.ts".to_owned(),
            layer: G3TsApparchLayer::IoInbound,
            item_name: "HttpPort".to_owned(),
            kind: G3TsApparchPublicItemKind::Interface,
            line: 1,
        }],
    };

    let results = crate::run::check(&input);
    g3ts_apparch_source_checks_assertions::run::assert_has_error(&results, "TS-APPARCH-SOURCE-02");
}
