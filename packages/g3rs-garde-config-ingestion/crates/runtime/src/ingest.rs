/// Assemble check inputs from selected and parsed data.
use cargo_toml_parser::CargoToml;
use clippy_toml_parser::ClippyToml;
use g3rs_garde_config_checks::{
    G3RsGardeConfigClippyBanChecksInput, G3RsGardeConfigDependencyCheckInput,
};

/// Build the dependency check input.
pub(crate) fn assemble_dependency(
    cargo_rel_path: String,
    cargo: CargoToml,
) -> G3RsGardeConfigDependencyCheckInput {
    G3RsGardeConfigDependencyCheckInput {
        cargo_rel_path,
        cargo,
    }
}

/// Build the clippy ban checks input.
pub(crate) fn assemble_clippy_bans(
    clippy_rel_path: String,
    clippy: ClippyToml,
) -> G3RsGardeConfigClippyBanChecksInput {
    G3RsGardeConfigClippyBanChecksInput {
        clippy_rel_path,
        clippy,
    }
}
