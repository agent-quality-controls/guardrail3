/// Assemble the checks input from selected and parsed data.
use cargo_toml_parser::CargoToml;
use g3rs_garde_types::{G3RsGardeClippyInput, G3RsGardeConfigChecksInput};

/// Build the checks input from parsed config files.
pub(crate) fn assemble(
    cargo_rel_path: String,
    cargo: CargoToml,
    clippy_input: G3RsGardeClippyInput,
) -> G3RsGardeConfigChecksInput {
    G3RsGardeConfigChecksInput {
        cargo_rel_path,
        cargo,
        clippy_input,
    }
}
