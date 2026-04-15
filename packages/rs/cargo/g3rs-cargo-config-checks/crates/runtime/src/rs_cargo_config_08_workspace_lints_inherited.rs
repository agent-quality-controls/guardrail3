use g3rs_cargo_types::G3RsCargoWorkspaceMember;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "RS-CARGO-CONFIG-08";

pub(crate) fn check(member: &G3RsCargoWorkspaceMember, results: &mut Vec<G3CheckResult>) {
    if member.lint_workspace_invalid {
        results.push(crate::support::error(
            ID,
            "workspace lint inheritance invalid",
            format!(
                "{}: `[lints].workspace` must be `true` or `false` in member Cargo.toml",
                member.member_rel
            ),
            &member.cargo_rel_path,
        ));
        return;
    }
    if member.lint_workspace_true {
        let package_name = member.package_name.as_deref().unwrap_or("unknown");
        results.push(
            crate::support::info(
                ID,
                "workspace lints inherited",
                format!(
                    "{package_name}: `[lints] workspace = true` inherits workspace lint policy"
                ),
                &member.cargo_rel_path,
            ),
        );
    } else {
        results.push(crate::support::error(
            ID,
            "workspace lints not inherited",
            format!(
                "{}: missing `[lints] workspace = true` in member Cargo.toml",
                member.member_rel
            ),
            &member.cargo_rel_path,
        ));
    }
}
