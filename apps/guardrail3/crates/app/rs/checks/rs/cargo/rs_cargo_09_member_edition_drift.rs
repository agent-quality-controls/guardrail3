use crate::domain::report::{CheckResult, Severity};

use super::inputs::WorkspaceMemberInput;

const ID: &str = "RS-CARGO-09";

pub fn check(input: &WorkspaceMemberInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(workspace_edition) = input.workspace.workspace_edition.as_deref() else {
        return;
    };
    let Some(member_edition) = input.member.edition.as_deref() else {
        return;
    };

    if edition_rank(member_edition) < edition_rank(workspace_edition) {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "member edition older than workspace".to_owned(),
            message: format!(
                "{} sets edition `{member_edition}` while workspace uses `{workspace_edition}`.",
                input.member.member_rel
            ),
            file: Some(input.member.rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
}

fn edition_rank(edition: &str) -> usize {
    match edition {
        "2015" => 0,
        "2018" => 1,
        "2021" => 2,
        "2024" => 3,
        _ => 0,
    }
}
