use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ExecutableCommandContextInput;

const ID: &str = "HOOK-SHARED-11";

pub fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<CheckResult>) {
    match input.parsed.shebang {
        Some("#!/bin/bash" | "#!/usr/bin/env bash" | "#!/bin/sh") => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Warn,
                "valid hook shebang present".to_owned(),
                "Hook script starts with a supported shell shebang.".to_owned(),
                Some(input.rel_path.to_owned()),
                Some(1),
                false,
            )
            .as_inventory(),
        ),
        Some(shebang) => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "unsupported hook shebang".to_owned(),
            format!("Hook script uses unsupported shebang `{shebang}`."),
            Some(input.rel_path.to_owned()),
            Some(1),
            false,
        )),
        None => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "hook shebang missing".to_owned(),
            "Hook script does not start with a supported shell shebang.".to_owned(),
            Some(input.rel_path.to_owned()),
            Some(1),
            false,
        )),
    }
}

#[cfg(test)]
pub(crate) fn run_case(content: &str) -> Vec<CheckResult> {
    let parsed = crate::hook_shell::parse_script(content);
    let input = ExecutableCommandContextInput {
        rel_path: ".githooks/pre-commit",
        kind: crate::facts::HookScriptKind::PreCommit,
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
