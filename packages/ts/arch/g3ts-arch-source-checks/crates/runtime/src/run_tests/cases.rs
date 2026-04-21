use g3ts_arch_types::{
    G3TsArchFacadeFileState, G3TsArchFacadeItem, G3TsArchFacadeReexport, G3TsArchFacadeSurface,
    G3TsArchSourceChecksInput,
};

#[test]
fn source_checks_flag_body_items_and_broad_reexports() {
    let input = G3TsArchSourceChecksInput {
        facades: vec![G3TsArchFacadeFileState::Parsed {
            surface: G3TsArchFacadeSurface {
                rel_path: "src/index.ts".to_owned(),
                body_items: vec![G3TsArchFacadeItem {
                    line: 3,
                    kind: "function_declaration",
                    name: "export function makeThing() {}".to_owned(),
                }],
                broad_reexports: vec![G3TsArchFacadeReexport {
                    line: 1,
                    source: "export * from \"./thing\";".to_owned(),
                }],
            },
        }],
    };

    let results = crate::run::check(&input);
    g3ts_arch_source_checks_assertions::run::assert_has_error(&results, "TS-ARCH-SOURCE-02");
    g3ts_arch_source_checks_assertions::run::assert_has_error(&results, "TS-ARCH-SOURCE-03");
}

#[test]
fn source_checks_flag_unreadable_facade() {
    let input = G3TsArchSourceChecksInput {
        facades: vec![G3TsArchFacadeFileState::Unreadable {
            rel_path: "src/index.ts".to_owned(),
            reason: "permission denied".to_owned(),
        }],
    };

    let results = crate::run::check(&input);
    g3ts_arch_source_checks_assertions::run::assert_has_finding(
        &results,
        "TS-ARCH-SOURCE-01",
        false,
        "facade file unreadable",
        "Facade file `src/index.ts` is unreadable: permission denied.",
        Some("src/index.ts"),
        None,
    );
}

#[test]
fn source_checks_flag_parse_error_facade() {
    let input = G3TsArchSourceChecksInput {
        facades: vec![G3TsArchFacadeFileState::ParseError {
            rel_path: "src/index.tsx".to_owned(),
            reason: "syntax error".to_owned(),
        }],
    };

    let results = crate::run::check(&input);
    g3ts_arch_source_checks_assertions::run::assert_has_finding(
        &results,
        "TS-ARCH-SOURCE-01",
        false,
        "facade file parse failed",
        "Facade file `src/index.tsx` could not be parsed: syntax error.",
        Some("src/index.tsx"),
        None,
    );
}

#[test]
fn source_checks_accept_clean_tsx_facade() {
    let input = G3TsArchSourceChecksInput {
        facades: vec![G3TsArchFacadeFileState::Parsed {
            surface: G3TsArchFacadeSurface {
                rel_path: "src/index.tsx".to_owned(),
                body_items: Vec::new(),
                broad_reexports: Vec::new(),
            },
        }],
    };

    let results = crate::run::check(&input);
    g3ts_arch_source_checks_assertions::run::assert_has_finding(
        &results,
        "TS-ARCH-SOURCE-01",
        true,
        "facade file parseable",
        "Facade file `src/index.tsx` parsed successfully.",
        Some("src/index.tsx"),
        None,
    );
    g3ts_arch_source_checks_assertions::run::assert_has_inventory(&results, "TS-ARCH-SOURCE-02");
    g3ts_arch_source_checks_assertions::run::assert_has_inventory(&results, "TS-ARCH-SOURCE-03");
}
