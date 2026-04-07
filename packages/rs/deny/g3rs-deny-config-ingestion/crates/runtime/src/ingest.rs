/// Assemble the checks input from selected and parsed data.
use deny_toml_parser::DenyToml;
use g3rs_deny_types::G3RsDenyConfigChecksInput;

/// Build the checks input from the parsed deny config and its relative path.
pub(crate) fn assemble(deny_rel_path: String, deny: DenyToml) -> G3RsDenyConfigChecksInput {
    G3RsDenyConfigChecksInput {
        deny_rel_path,
        deny,
    }
}
