use g3ts_arch_types::G3TsArchFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3ts-arch/structural-split";

const MAX_DEPTH: usize = 3;
const MAX_SIBLING_DIRS: usize = 4;
const MAX_SIBLING_CODE_FILES: usize = 10;

pub(crate) fn check(input: &G3TsArchFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(source_tree) = &input.source_tree else {
        return;
    };

    let mut reasons = Vec::new();
    if source_tree.max_depth > MAX_DEPTH {
        reasons.push(format!(
            "source depth {} (max {})",
            source_tree.max_depth, MAX_DEPTH
        ));
    }
    if source_tree.max_sibling_dir_count > MAX_SIBLING_DIRS {
        reasons.push(format!(
            "{} sibling directories (max {})",
            source_tree.max_sibling_dir_count, MAX_SIBLING_DIRS
        ));
    }
    if source_tree.max_sibling_code_file_count > MAX_SIBLING_CODE_FILES {
        reasons.push(format!(
            "{} sibling source files (max {})",
            source_tree.max_sibling_code_file_count, MAX_SIBLING_CODE_FILES
        ));
    }

    if reasons.is_empty() {
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "source tree too complex, must split".to_owned(),
        format!(
            "The target root exceeds structural complexity thresholds: {}. Extract related source groups into clearer subpackages or module folders.",
            reasons.join(", ")
        ),
        None,
        None,
    ));
}
