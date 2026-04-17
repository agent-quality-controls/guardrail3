use g3rs_release_types::G3RsReleaseInputFailure;
use guardrail3_check_types::G3CheckResult;

pub(super) fn input(rel_path: &str, message: &str) -> G3RsReleaseInputFailure {
    G3RsReleaseInputFailure {
        rel_path: rel_path.to_owned(),
        message: message.to_owned(),
    }
}

pub(super) fn check(rel_path: &str, message: &str) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    super::super::check(&input(rel_path, message), &mut results);
    results
}
