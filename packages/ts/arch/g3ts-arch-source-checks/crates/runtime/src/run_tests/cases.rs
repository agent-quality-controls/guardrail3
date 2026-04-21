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
