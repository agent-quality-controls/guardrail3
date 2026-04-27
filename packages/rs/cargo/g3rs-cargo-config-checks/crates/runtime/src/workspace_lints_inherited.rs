use cargo_toml_parser::types::CargoBoolFieldState;
use g3rs_cargo_types::G3RsCargoWorkspaceMember;
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3rs-cargo/workspace-lints-inherited";

pub(crate) fn check(member: &G3RsCargoWorkspaceMember, results: &mut Vec<G3CheckResult>) {
    match crate::support::member_lints_workspace_state(member) {
        CargoBoolFieldState::WrongType(_) => results.push(crate::support::error(
            ID,
            "workspace lint inheritance invalid",
            format!(
                "{}: `[lints].workspace` must be `true` or `false` in member Cargo.toml",
                member.member_rel
            ),
            &member.cargo_rel_path,
        )),
        CargoBoolFieldState::Value(true) => {
            let package_name = crate::support::member_package_name(member).unwrap_or("unknown");
            results.push(crate::support::info(
                ID,
                "workspace lints inherited",
                format!(
                    "{package_name}: `[lints] workspace = true` inherits workspace lint policy"
                ),
                &member.cargo_rel_path,
            ));
        }
        CargoBoolFieldState::Missing | CargoBoolFieldState::Value(false) => {
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
}

#[cfg(test)]
#[path = "workspace_lints_inherited_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod workspace_lints_inherited_tests;
