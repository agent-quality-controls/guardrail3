/// Assemble the checks input from selected and parsed data.
use cargo_toml_parser::CargoToml;
use g3rs_fmt_config_checks::G3RsFmtConfigChecksInput;
use rust_toolchain_toml_parser::RustToolchainToml;
use rustfmt_toml_parser::RustfmtToml;

/// Build the checks input from the three parsed config files and their relative paths.
pub(crate) fn assemble(
    rustfmt_rel_path: String,
    rustfmt: RustfmtToml,
    cargo_rel_path: String,
    cargo: CargoToml,
    toolchain_rel_path: String,
    toolchain: RustToolchainToml,
) -> G3RsFmtConfigChecksInput {
    G3RsFmtConfigChecksInput {
        rustfmt_rel_path,
        rustfmt,
        cargo_rel_path,
        cargo,
        toolchain_rel_path,
        toolchain,
    }
}
