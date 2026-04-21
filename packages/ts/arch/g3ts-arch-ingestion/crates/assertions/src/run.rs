use g3ts_arch_types::G3TsArchManifestState;

pub fn assert_manifest_parsed(input: &g3ts_arch_types::G3TsArchConfigChecksInput) {
    assert!(
        matches!(input.manifest, G3TsArchManifestState::Parsed { .. }),
        "expected parsed manifest, got {:?}",
        input.manifest
    );
}
