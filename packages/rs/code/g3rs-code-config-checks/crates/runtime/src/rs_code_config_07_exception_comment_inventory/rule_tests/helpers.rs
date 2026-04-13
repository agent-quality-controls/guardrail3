use g3rs_code_config_checks_types::{
    G3RsCodeConfigChecksInput, G3RsCodeConfigFile, G3RsCodeConfigFileKind,
};
use guardrail3_check_types::G3CheckResult;

pub(super) fn run_check(files: Vec<G3RsCodeConfigFile>) -> Vec<G3CheckResult> {
    crate::run::check(&G3RsCodeConfigChecksInput { files })
}

pub(super) fn text_file(rel_path: &str, content: &str) -> G3RsCodeConfigFile {
    G3RsCodeConfigFile {
        rel_path: rel_path.to_owned(),
        content: content.to_owned(),
        kind: G3RsCodeConfigFileKind::Text,
    }
}
