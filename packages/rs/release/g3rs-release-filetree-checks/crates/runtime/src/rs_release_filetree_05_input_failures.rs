use g3rs_release_filetree_checks_types::G3RsReleaseInputFailure;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-RELEASE-FILETREE-05";

pub(crate) fn check(failure: &G3RsReleaseInputFailure, results: &mut Vec<G3CheckResult>) {
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "failed to read release filetree input".to_owned(),
        failure.message.clone(),
        Some(failure.rel_path.clone()),
        None,
    ));
}

#[cfg(test)]
mod tests {
    use super::check;
    use crate::test_support::failure;

    #[test]
    fn reports_input_failure() {
        let mut results = Vec::new();

        check(&failure("README.md", "Failed to read README"), &mut results);

        assert_eq!(results[0].id(), "RS-RELEASE-FILETREE-05");
        assert_eq!(results[0].title(), "failed to read release filetree input");
    }
}
