use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustfmtRootInput;

const ID: &str = "RS-FMT-06";

pub fn check(input: &RustfmtRootInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(rel) = input.config_rel else {
        return;
    };
    let Some(parsed) = input.parsed else {
        return;
    };
    let Some(workspace_edition) = input.workspace_edition else {
        return;
    };
    let Some(rustfmt_edition) = parsed.get("edition").and_then(toml::Value::as_str) else {
        return;
    };

    if rustfmt_edition != workspace_edition {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "rustfmt edition differs from Cargo edition".to_owned(),
            message: format!(
                "rustfmt edition `{rustfmt_edition}` differs from Cargo edition `{workspace_edition}`."
            ),
            file: Some(rel.to_owned()),
            line: None,
            inventory: false,
        });
    }
}
