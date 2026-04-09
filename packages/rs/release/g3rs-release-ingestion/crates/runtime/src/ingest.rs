/// Assemble the checks input from selected and parsed data.
use cargo_toml_parser::CargoToml;
use cliff_toml_parser::CliffToml;
use g3rs_release_types::G3RsReleaseConfigChecksInput;
use release_plz_toml_parser::ReleasePlzToml;

/// Build the checks input from the parsed config files and their relative paths.
pub(crate) fn assemble(
    cargo_rel_path: String,
    cargo: CargoToml,
    release_plz_rel_path: Option<String>,
    release_plz: Option<ReleasePlzToml>,
    cliff_rel_path: Option<String>,
    cliff: Option<CliffToml>,
) -> G3RsReleaseConfigChecksInput {
    G3RsReleaseConfigChecksInput {
        cargo_rel_path,
        cargo,
        release_plz_rel_path,
        release_plz,
        cliff_rel_path,
        cliff,
    }
}
