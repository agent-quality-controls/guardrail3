use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_reason_policy::validate_reason_text;

use crate::inputs::WorkspaceMemberCargoInput;
use crate::lint_support::{
    allow_selector, escape_hatch_reason, explicit_allow_entries, has_valid_lint_level, member_lints,
};

const ID: &str = "RS-CARGO-13";

pub fn check(input: &WorkspaceMemberCargoInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.member.lint_workspace_true || input.member.parse_error.is_some() {
        return;
    }
    let Some(parsed) = input.member.parsed.as_ref() else {
        return;
    };
    let workspace_policy_complete = crate::lint_support::policy_lints(input.workspace, "rust")
        .is_some()
        && crate::lint_support::policy_lints(input.workspace, "clippy").is_some();
    let member_override_shapes_valid =
        [member_lints(parsed, "rust"), member_lints(parsed, "clippy")]
            .into_iter()
            .all(lints_are_well_formed);

    let mut documented_count = 0usize;
    let mut missing_reason_count = 0usize;
    let mut weak_reason_count = 0usize;
    for (family, lints) in [
        ("rust", member_lints(parsed, "rust")),
        ("clippy", member_lints(parsed, "clippy")),
    ] {
        for lint_name in explicit_allow_entries(lints) {
            let selector = allow_selector(family, &lint_name);
            match escape_hatch_reason(
                &input.workspace.escape_hatches,
                "cargo",
                &input.member.cargo_rel_path,
                "lint_allow",
                &selector,
            ) {
                None => {
                    missing_reason_count += 1;
                    results.push(CheckResult::from_parts(
                        ID.to_owned(),
                        Severity::Error,
                        "member-local allow entry missing reason".to_owned(),
                        format!(
                            "`{}` uses `[lints] workspace = true` but still sets `{lint_name}` to `allow` in `{family}` without a matching escape-hatch reason.",
                            input.member.cargo_rel_path
                        ),
                        Some(input.member.cargo_rel_path.clone()),
                        None,
                        false,
                    ));
                }
                Some(reason) => match validate_reason_text(reason) {
                    Ok(()) => {
                        documented_count += 1;
                        results.push(CheckResult::from_parts(
                            ID.to_owned(),
                            Severity::Error,
                            "member-local allow entry forbidden".to_owned(),
                            format!(
                                "`{}` uses `[lints] workspace = true` but still sets `{lint_name}` to `allow` in `{family}`. The entry is documented but still forbidden.",
                                input.member.cargo_rel_path
                            ),
                            Some(input.member.cargo_rel_path.clone()),
                            None,
                            false,
                        ));
                    }
                    Err(issue) => {
                        weak_reason_count += 1;
                        results.push(CheckResult::from_parts(
                            ID.to_owned(),
                            Severity::Error,
                            "member-local allow entry reason too weak".to_owned(),
                            format!(
                                "`{}` sets `{lint_name}` to `allow` in `{family}` with a weak reason: {}.",
                                input.member.cargo_rel_path,
                                issue.message()
                            ),
                            Some(input.member.cargo_rel_path.clone()),
                            None,
                            false,
                        ));
                    }
                },
            }
        }
    }

    let total = documented_count + missing_reason_count + weak_reason_count;
    if total == 0 && workspace_policy_complete && member_override_shapes_valid {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "no member-local allow entries".to_owned(),
                format!(
                    "`{}` does not add member-local allow entries on top of inherited policy.",
                    input.member.cargo_rel_path
                ),
                Some(input.member.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    } else if total > 0 {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "member-local allow count".to_owned(),
            format!(
                "`{}` has {total} member-local manifest allow entries ({documented_count} documented, {missing_reason_count} missing reasons, {weak_reason_count} weak reasons).",
                input.member.cargo_rel_path
            ),
            None,
            None,
            false,
        ));
    }
}


fn lints_are_well_formed(lints: Option<&toml::Value>) -> bool {
    let Some(lints) = lints else {
        return true;
    };
    let Some(table) = lints.as_table() else {
        return false;
    };

    table.values().all(has_valid_lint_level)
}

