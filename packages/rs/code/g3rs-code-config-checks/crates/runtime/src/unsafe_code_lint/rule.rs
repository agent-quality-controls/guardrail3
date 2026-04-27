use cargo_toml_parser::types::LintValue;
use g3rs_code_types::{G3RsCodeConfigFile, G3RsCodeConfigFileKind};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-code/unsafe-code-lint";

pub(crate) fn check(file: &G3RsCodeConfigFile, results: &mut Vec<G3CheckResult>) {
    let G3RsCodeConfigFileKind::CargoToml { cargo } = &file.kind else {
        return;
    };

    let Some(workspace) = cargo.workspace.as_ref() else {
        return;
    };

    let lint_level = workspace
        .lints
        .as_ref()
        .and_then(|lints| lints.tools.get("rust").cloned())
        .and_then(|tool| tool.get("unsafe_code").cloned())
        .and_then(|value| match value {
            LintValue::Level(level) => Some(level),
            LintValue::Detailed(detail) => Some(detail.level),
        });

    match lint_level.as_deref() {
        Some("forbid") => results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "unsafe_code = forbid".to_owned(),
                "unsafe_code is set to forbid in workspace lints.".to_owned(),
                Some(file.rel_path.clone()),
                None,
            )
            .into_inventory(),
        ),
        Some("deny") => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "unsafe_code should be forbid".to_owned(),
            "unsafe_code = deny can be overridden; use forbid in workspace lints.".to_owned(),
            Some(file.rel_path.clone()),
            None,
        )),
        _ => {}
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
