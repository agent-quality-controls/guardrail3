use crate::compat::{G3CheckResult, G3Severity};

use crate::inputs::ExecutableCommandContextInput;

/// `ID` constant.
const ID: &str = "g3rs-hooks/valid-shebang";

/// `check` function.
pub(crate) fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<G3CheckResult>) {
    match input.parsed.shebang.as_deref() {
        Some("#!/bin/bash" | "#!/usr/bin/env bash" | "#!/bin/sh") => results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "valid hook shebang present".to_owned(),
                "Hook script starts with a supported shell shebang.".to_owned(),
                Some(input.rel_path.to_owned()),
                Some(1),
                false,
            )
            .into_inventory(),
        ),
        // Reason: a missing or unsupported shebang fails the hook silently or runs it under
        // the wrong interpreter; the gate must reject this case.
        Some(shebang) => results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Error,
            "unsupported hook shebang".to_owned(),
            format!("Hook script uses unsupported shebang `{shebang}`."),
            Some(input.rel_path.to_owned()),
            Some(1),
            false,
        )),
        None => results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Error,
            "hook shebang missing".to_owned(),
            "Hook script does not start with a supported shell shebang.".to_owned(),
            Some(input.rel_path.to_owned()),
            Some(1),
            false,
        )),
    }
}

#[cfg(test)]
pub(crate) fn run_case(content: &str) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = ExecutableCommandContextInput {
        rel_path: ".githooks/pre-commit",
        kind: crate::facts::HookScriptKind::PreCommit,
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
