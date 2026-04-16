use g3rs_arch_types::types::G3RsArchSourceFile;
use guardrail3_check_types::G3CheckResult;

pub(super) fn source_file(rel_path: &str, content: &str) -> G3RsArchSourceFile {
    G3RsArchSourceFile {
        rel_path: rel_path.to_owned(),
        content: content.to_owned(),
    }
}

pub(super) fn run_rule(file: &G3RsArchSourceFile) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::rs_arch_09_no_path_attr::check_file(file, &mut results);
    results
}
