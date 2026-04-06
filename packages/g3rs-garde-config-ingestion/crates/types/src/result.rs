use g3rs_garde_config_checks::{
    G3RsGardeConfigClippyBanChecksInput, G3RsGardeConfigDependencyCheckInput,
};

/// Ingestion result for garde config checks.
///
/// The dependency check input is always produced (Cargo.toml is required).
/// The clippy ban checks input is only produced when a clippy config file exists.
#[derive(Debug)]
pub struct G3RsGardeConfigIngestionResult {
    /// Input for the dependency-present check (always available).
    pub dependency: G3RsGardeConfigDependencyCheckInput,
    /// Input for the clippy ban checks (only when clippy config exists).
    pub clippy_bans: Option<G3RsGardeConfigClippyBanChecksInput>,
}
