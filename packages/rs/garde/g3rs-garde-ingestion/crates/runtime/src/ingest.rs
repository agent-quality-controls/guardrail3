/// Assemble the checks input from selected and parsed data.
use cargo_toml_parser::types::CargoToml;
use g3rs_garde_types::{G3RsGardeApplicability, G3RsGardeClippyInput, G3RsGardeConfigChecksInput};

/// Build the checks input from parsed config files.
pub(crate) fn assemble(
    applicability: G3RsGardeApplicability,
    cargo_rel_path: String,
    cargo: CargoToml,
    clippy_input: G3RsGardeClippyInput,
) -> G3RsGardeConfigChecksInput {
    G3RsGardeConfigChecksInput {
        applicability,
        cargo_rel_path,
        cargo,
        clippy_input,
    }
}
