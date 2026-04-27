use g3ts_arch_types::{
    G3TsArchConfigChecksInput, G3TsArchDeclaredEntryPoint, G3TsArchEntryPointSource,
    G3TsArchManifestSnapshot, G3TsArchManifestState,
};

#[test]
fn config_checks_flag_noncanonical_entrypoint() {
    let input = G3TsArchConfigChecksInput {
        manifest: G3TsArchManifestState::Parsed {
            snapshot: G3TsArchManifestSnapshot {
                rel_path: "package.json".to_owned(),
                declared_entrypoints: vec![G3TsArchDeclaredEntryPoint {
                    source: G3TsArchEntryPointSource::ExportsDot,
                    rel_path: "src/public/api.ts".to_owned(),
                }],
            },
        },
    };

    let results = crate::run::check(&input);
    g3ts_arch_config_checks_assertions::run::assert_has_error(
        &results,
        "g3ts-arch/declared-entrypoints-canonical",
    );
}
