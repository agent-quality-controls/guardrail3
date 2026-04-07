/// Assemble the checks input from parsed data.
use cargo_toml_parser::CargoToml;
use g3rs_toolchain_config_checks::G3RsToolchainConfigChecksInput;
use rust_toolchain_toml_parser::RustToolchainToml;

/// Build the checks input from parsed config files.
pub(crate) fn assemble(
    toolchain_rel_path: String,
    toolchain_toml: RustToolchainToml,
    cargo_rel_path: Option<String>,
    cargo_toml: Option<CargoToml>,
) -> G3RsToolchainConfigChecksInput {
    G3RsToolchainConfigChecksInput {
        toolchain_rel_path,
        toolchain_toml,
        cargo_rel_path,
        cargo_toml,
    }
}
