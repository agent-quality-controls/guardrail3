use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ExecutableCommandContextInput;

const ID: &str = "HOOK-SHARED-11";

pub fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<CheckResult>) {
    match input.parsed.shebang {
        Some("#!/bin/bash" | "#!/usr/bin/env bash" | "#!/bin/sh") => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "valid hook shebang present".to_owned(),
                message: "Hook script starts with a supported shell shebang.".to_owned(),
                file: Some(input.rel_path.to_owned()),
                line: Some(1),
                inventory: false,
            }
            .as_inventory(),
        ),
        Some(shebang) => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "unsupported hook shebang".to_owned(),
            message: format!("Hook script uses unsupported shebang `{shebang}`."),
            file: Some(input.rel_path.to_owned()),
            line: Some(1),
            inventory: false,
        }),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "hook shebang missing".to_owned(),
            message: "Hook script does not start with a supported shell shebang.".to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: Some(1),
            inventory: false,
        }),
    }
}

#[cfg(test)]
pub(crate) fn run_case(content: &str) -> Vec<CheckResult> {
    let parsed = crate::hook_shell::parse_script(content);
    let input = ExecutableCommandContextInput {
        rel_path: ".githooks/pre-commit",
        kind: super::facts::HookScriptKind::PreCommit,
        content,
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "hook_shared_11_valid_shebang_tests/mod.rs"]
mod hook_shared_11_valid_shebang_tests;
