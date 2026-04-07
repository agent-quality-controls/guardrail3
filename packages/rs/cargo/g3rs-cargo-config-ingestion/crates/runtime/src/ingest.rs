/// Assemble the checks input from selected and parsed data.
use cargo_toml_parser::CargoToml;
use g3rs_cargo_types::G3RsCargoConfigChecksInput;

/// Build the checks input from the parsed manifest and its relative path.
pub(crate) fn assemble(cargo_rel_path: String, cargo: CargoToml) -> G3RsCargoConfigChecksInput {
    G3RsCargoConfigChecksInput {
        cargo_rel_path,
        cargo,
    }
}
