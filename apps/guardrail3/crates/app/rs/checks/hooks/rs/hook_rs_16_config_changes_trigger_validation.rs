use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustHookCommandInput;

const ID: &str = "HOOK-RS-16";

pub fn check(content: &str, input: &RustHookCommandInput<'_>, results: &mut Vec<CheckResult>) {
    let config_needles = [
        "guardrail3.toml",
        "clippy.toml",
        ".clippy.toml",
        "deny.toml",
        ".deny.toml",
        "rustfmt.toml",
        ".rustfmt.toml",
        "rust-toolchain.toml",
    ];

    let found = config_needles.iter().all(|needle| {
        input
            .parsed
            .executable_lines
            .iter()
            .any(|line| mentions_config(line.raw, needle))
            || content
                .lines()
                .filter(|line| {
                    let trimmed = line.trim();
                    !trimmed.starts_with('#') && !trimmed.is_empty()
                })
                .any(|line| {
                    (line.trim_start().starts_with("case ")
                        || line.contains("git diff")
                        || line.contains("STAGED_FILES")
                        || line.contains("changed_files"))
                        && mentions_config(line, needle)
                })
    });

    if found {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "Rust config changes trigger hook validation".to_owned(),
                message: "Hook trigger logic covers Rust guardrail config files.".to_owned(),
                file: Some(input.rel_path.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "Rust config-change trigger coverage incomplete".to_owned(),
            message: "Hook trigger logic does not clearly include all Rust guardrail config files."
                .to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

fn mentions_config(line: &str, needle: &str) -> bool {
    line.contains(needle) || line.replace('\\', "").contains(needle)
}

#[cfg(test)]
#[path = "hook_rs_16_config_changes_trigger_validation_tests.rs"]
mod tests;
