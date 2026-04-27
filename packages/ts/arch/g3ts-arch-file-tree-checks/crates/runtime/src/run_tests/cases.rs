use g3ts_arch_types::{
    G3TsArchDeclaredEntryPoint, G3TsArchEntryPointSource, G3TsArchFileTreeChecksInput,
    G3TsArchManifestSnapshot, G3TsArchManifestState, G3TsArchSourceTree,
};

#[test]
fn filetree_checks_flag_structural_split() {
    let input = G3TsArchFileTreeChecksInput {
        existing_entrypoints: Vec::new(),
        manifest: G3TsArchManifestState::Missing,
        source_tree: Some(G3TsArchSourceTree {
            max_depth: 5,
            max_sibling_dir_count: 2,
            max_sibling_code_file_count: 3,
        }),
    };

    let results = crate::run::check(&input);
    g3ts_arch_file_tree_checks_assertions::run::assert_has_error(
        &results,
        "g3ts-arch/structural-split",
    );
}

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
        source_tree: None,
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

#[test]
fn filetree_checks_stay_quiet_at_exact_structural_thresholds() {
    let input = G3TsArchFileTreeChecksInput {
        existing_entrypoints: Vec::new(),
        manifest: G3TsArchManifestState::Missing,
        source_tree: Some(G3TsArchSourceTree {
            max_depth: 3,
            max_sibling_dir_count: 4,
            max_sibling_code_file_count: 10,
        }),
    };

    let results = crate::run::check(&input);
    g3ts_arch_file_tree_checks_assertions::run::assert_missing(
        &results,
        "g3ts-arch/structural-split",
    );
}
