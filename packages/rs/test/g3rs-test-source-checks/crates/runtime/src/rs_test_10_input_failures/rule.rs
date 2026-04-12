use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-TEST-SOURCE-10";

pub(crate) fn check(
    _root_rel_dir: &str,
    rel_path: &str,
    message: &str,
    results: &mut Vec<G3CheckResult>,
) {
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "failed to read test input".to_owned(),
        message.to_owned(),
        Some(rel_path.to_owned()),
        None,
    ));
}
