use g3rs_apparch_types::{G3RsApparchPatchBypassChecksInput, G3RsApparchRustPolicyState};
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_reason_policy::validate_reason_text;

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
    let reason = match &input.rust_policy {
        G3RsApparchRustPolicyState::Parsed { waivers, .. } => waivers
            .iter()
            .find(|waiver| {
                waiver.rule == ID
                    && waiver.file == patch.cargo_rel_path
                    && waiver.selector == selector
            })
            .map(|waiver| waiver.reason.as_str()),
        G3RsApparchRustPolicyState::Missing => None,
        G3RsApparchRustPolicyState::Unreadable { rel_path, reason } => {
            push_policy_unavailable(patch, rel_path, reason, "is unreadable", results);
            return;
        }
        G3RsApparchRustPolicyState::ParseError { rel_path, reason } => {
            push_policy_unavailable(patch, rel_path, reason, "could not be parsed", results);
            return;
        }
    };

    match reason {
        None => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            format!("{} bypass `{}` missing waiver", patch.kind.label(), patch.key),
            format!(
                "Internal {} entry `{}` resolves to `{}` and bypasses normal apparch dependency analysis. Add a waiver in guardrail3-rs.toml with rule = \"{}\", file = \"{}\", selector = \"{}\", and a specific reason.",
                patch.kind.label(),
                patch.key,
                patch.target_rel_dir,
                ID,
                patch.cargo_rel_path,
                selector
            ),
            Some(patch.cargo_rel_path.clone()),
            None,
        )),
        Some(reason) => match validate_reason_text(reason) {
            Ok(()) => results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Warn,
                format!("{} bypass `{}` is documented", patch.kind.label(), patch.key),
                format!(
                    "Internal {} entry `{}` resolves to `{}` with documented reason `{}`.",
                    patch.kind.label(),
                    patch.key,
                    patch.target_rel_dir,
                    reason
                ),
                Some(patch.cargo_rel_path.clone()),
                None,
            )),
            Err(issue) => results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                format!("{} bypass `{}` reason too weak", patch.kind.label(), patch.key),
                format!(
                    "Internal {} entry `{}` resolves to `{}` but the waiver reason is too weak: {}.",
                    patch.kind.label(),
                    patch.key,
                    patch.target_rel_dir,
                    issue.message()
                ),
                Some(patch.cargo_rel_path.clone()),
                None,
            )),
        },
    }
}
