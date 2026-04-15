use g3rs_clippy_types::G3RsClippyFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-CLIPPY-FILETREE-01";

pub(crate) fn check(input: &G3RsClippyFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    match input.preferred_root_config_rel_path.as_deref() {
        Some(rel_path) => results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "workspace root covered by clippy config".to_owned(),
                format!("Workspace root is covered by `{rel_path}`."),
                Some(rel_path.to_owned()),
                None,
            )
            .into_inventory(),
        ),
        None => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "workspace root uncovered by clippy config".to_owned(),
            "Add `clippy.toml` or `.clippy.toml` at the workspace root so clippy policy is not left to defaults.".to_owned(),
            Some("clippy.toml".to_owned()),
            None,
        )),
    }
}
