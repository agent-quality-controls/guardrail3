use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::WorkspaceMemberCargoInput;

const ID: &str = "RS-CARGO-04";

pub fn check(input: &WorkspaceMemberCargoInput<'_>, results: &mut Vec<CheckResult>) {
    if input.member.parse_error.is_some() {
        return;
    }

    if input.member.lint_workspace_true {
        let package_name = input.member.package_name.as_deref().unwrap_or("unknown");
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "workspace lints inherited".to_owned(),
                format!(
                    "{package_name}: `[lints] workspace = true` inherits workspace lint policy"
                ),
                Some(input.member.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "workspace lints not inherited".to_owned(),
            format!(
                "{}: missing `[lints] workspace = true` in member Cargo.toml",
                input.member.member_rel
            ),
            Some(input.member.cargo_rel_path.clone()),
            None,
            false,
        ));
    }
}

#[cfg(test)]

mod tests;
