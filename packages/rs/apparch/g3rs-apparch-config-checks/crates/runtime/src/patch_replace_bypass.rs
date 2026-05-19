use g3rs_apparch_types::{G3RsApparchPatchBypassChecksInput, G3RsApparchRustPolicyState};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-apparch/patch-replace-bypass";

fn push_policy_unavailable(
    patch: &g3rs_apparch_types::G3RsApparchPatchBypass,
    rel_path: &str,
    reason: &str,
    kind_phrase: &str,
    results: &mut Vec<G3CheckResult>,
) {
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        format!(
            "cannot validate {} bypass `{}`",
            patch.kind.label(),
            patch.key
        ),
        format!(
            "Internal {} entry `{}` resolves to `{}` inside apparch, but `{}` {}: {}.",
            patch.kind.label(),
            patch.key,
            patch.target_rel_dir,
            rel_path,
            kind_phrase,
            reason
        ),
        Some(patch.cargo_rel_path.clone()),
        None,
    ));
}

pub(crate) fn check(input: &G3RsApparchPatchBypassChecksInput, results: &mut Vec<G3CheckResult>) {
    let patch = &input.patch;
    let selector = patch.key.as_str();
    match &input.rust_policy {
        G3RsApparchRustPolicyState::Parsed { .. } | G3RsApparchRustPolicyState::Missing => {}
        G3RsApparchRustPolicyState::Unreadable { rel_path, reason } => {
            push_policy_unavailable(patch, rel_path, reason, "is unreadable", results);
            return;
        }
        G3RsApparchRustPolicyState::ParseError { rel_path, reason } => {
            push_policy_unavailable(patch, rel_path, reason, "could not be parsed", results);
            return;
        }
    }
    results.push(
        G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            format!("{} bypass `{}` missing waiver", patch.kind.label(), patch.key),
            format!(
                "Internal {} entry `{}` resolves to `{}` and bypasses normal apparch dependency analysis. Add a waiver in guardrail3-rs.toml with rule = \"{}\", subject = \"{}\", selector = \"{}\", and a specific reason.",
                patch.kind.label(),
                patch.key,
                patch.target_rel_dir,
                ID,
                patch.cargo_rel_path,
                selector
            ),
            Some(patch.cargo_rel_path.clone()),
            None,
        )
        .with_selector(selector.to_owned()),
    );
}
