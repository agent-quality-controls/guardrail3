use g3rs_deny_types::{G3RsDenyFileTreeChecksInput, G3RsDenyInputFailure};

/// One `(rel_path, message)` pair describing a synthesized deny input failure.
type InputFailureSpec<'a> = (&'a str, &'a str);

/// Builds a [`G3RsDenyFileTreeChecksInput`] from compact test-friendly inputs.
pub fn input(
    selected_deny_rel_path: Option<&str>,
    candidate_deny_rel_paths: Vec<&str>,
    input_failures: Vec<InputFailureSpec<'_>>,
) -> G3RsDenyFileTreeChecksInput {
    G3RsDenyFileTreeChecksInput {
        selected_deny_rel_path: selected_deny_rel_path.map(str::to_owned),
        candidate_deny_rel_paths: candidate_deny_rel_paths
            .into_iter()
            .map(str::to_owned)
            .collect(),
        input_failures: input_failures
            .into_iter()
            .map(|(rel_path, message)| G3RsDenyInputFailure {
                title: if rel_path == "guardrail3-rs.toml" {
                    "deny rust policy is not parseable".to_owned()
                } else {
                    "deny input failure".to_owned()
                },
                rel_path: rel_path.to_owned(),
                message: message.to_owned(),
            })
            .collect(),
    }
}
