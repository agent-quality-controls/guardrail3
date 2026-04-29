use g3ts_arch_types::{
    G3TsArchDeclaredEntryPoint, G3TsArchEntryPointSource, G3TsArchFileTreeChecksInput,
    G3TsArchManifestSnapshot, G3TsArchManifestState,
};

#[test]
fn filetree_checks_require_each_declared_entrypoint_to_exist() {
    let input = G3TsArchFileTreeChecksInput {
        existing_entrypoints: vec!["src/index.ts".to_owned()],
        manifest: G3TsArchManifestState::Parsed {
            snapshot: G3TsArchManifestSnapshot {
                rel_path: "package.json".to_owned(),
                declared_entrypoints: vec![
                    G3TsArchDeclaredEntryPoint {
                        source: G3TsArchEntryPointSource::ExportsDot,
                        rel_path: "src/index.ts".to_owned(),
                    },
                    G3TsArchDeclaredEntryPoint {
                        source: G3TsArchEntryPointSource::Types,
                        rel_path: "src/public.ts".to_owned(),
                    },
                ],
            },
        },
    };

    let results = crate::run::check(&input);
    g3ts_arch_file_tree_checks_assertions::run::assert_has_inventory(
        &results,
        "g3ts-arch/declared-entrypoint-exists",
    );
    g3ts_arch_file_tree_checks_assertions::run::assert_has_error(
        &results,
        "g3ts-arch/declared-entrypoint-exists",
    );
}
