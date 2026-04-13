use g3rs_cargo_types::{G3RsCargoMissingMember, G3RsCargoPolicyRootKind};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-CARGO-FILETREE-10";

pub(crate) fn check(input: &G3RsCargoMissingMember, results: &mut Vec<G3CheckResult>) {
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Warn,
        "declared workspace member missing Cargo.toml".to_owned(),
        format!(
            "`{}` is declared in `[workspace].members` but no `Cargo.toml` was discovered there. Remove it from `[workspace].members` or create a `Cargo.toml` at that path.",
            input.member_rel
        ),
        Some(input.workspace_cargo_rel_path.clone()),
        None,
    ));
}

pub(crate) fn check_inventory(
    kind: Option<G3RsCargoPolicyRootKind>,
    cargo_rel_path: &str,
    members_parse_error: bool,
    no_missing_members: bool,
    results: &mut Vec<G3CheckResult>,
) {
    if kind != Some(G3RsCargoPolicyRootKind::WorkspaceRoot) {
        return;
    }
    if !no_missing_members || members_parse_error {
        return;
    }

    results.push(
        G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Info,
            "all declared workspace members have Cargo.toml".to_owned(),
            "workspace root declares only member directories that contain Cargo.toml.".to_owned(),
            Some(cargo_rel_path.to_owned()),
            None,
        )
        .into_inventory(),
    );
}
