/// Assemble the checks input from selected and parsed data.
use cargo_toml_parser::CargoToml;
use clippy_toml_parser::ClippyToml;
use g3rs_garde_config_checks::G3RsGardeConfigChecksInput;

/// Build the checks input from parsed config files.
pub(crate) fn assemble(
    cargo_rel_path: String,
    cargo: CargoToml,
    clippy_rel_path: Option<String>,
    clippy: Option<ClippyToml>,
) -> G3RsGardeConfigChecksInput {
    G3RsGardeConfigChecksInput {
        cargo_rel_path,
        cargo,
        clippy_rel_path,
        clippy,
    }
}
