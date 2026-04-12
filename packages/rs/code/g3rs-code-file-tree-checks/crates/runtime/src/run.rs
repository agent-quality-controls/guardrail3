use g3rs_code_file_tree_checks_types::G3RsCodeFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsCodeFileTreeChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    for root in &input.roots {
        crate::rs_code_filetree_35_root_structural_cap::check(root, &mut results);
    }
    results
}
