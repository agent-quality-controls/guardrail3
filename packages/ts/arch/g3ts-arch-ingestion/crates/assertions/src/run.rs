use g3ts_arch_types::G3TsArchManifestState;

/// Assert the manifest in `input` is in the parsed state.
///
/// # Panics
///
/// Panics if the manifest is not `G3TsArchManifestState::Parsed`.
pub fn assert_manifest_parsed(input: &g3ts_arch_types::G3TsArchConfigChecksInput) {
    assert!(
        matches!(input.manifest, G3TsArchManifestState::Parsed { .. }),
        "expected parsed manifest, got {:?}",
        input.manifest
    );
}
