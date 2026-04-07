/// Assemble the checks input from selected and parsed data.
use clippy_toml_parser::ClippyToml;
use g3rs_clippy_types::G3RsClippyConfigChecksInput;

/// Build the checks input from the parsed clippy config and its relative path.
pub(crate) fn assemble(clippy_rel_path: String, clippy: ClippyToml) -> G3RsClippyConfigChecksInput {
    G3RsClippyConfigChecksInput {
        clippy_rel_path,
        clippy,
    }
}
