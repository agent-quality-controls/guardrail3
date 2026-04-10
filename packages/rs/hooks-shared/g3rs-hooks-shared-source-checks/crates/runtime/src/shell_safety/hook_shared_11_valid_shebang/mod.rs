use crate::compat::{G3CheckResult, G3Severity};

use crate::inputs::ExecutableCommandContextInput;

const ID: &str = "HOOK-SHARED-11";

pub(crate) fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<G3CheckResult>) {
    match input.parsed.shebang {
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
        Some(shebang) => results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Warn,
            "unsupported hook shebang".to_owned(),
            format!("Hook script uses unsupported shebang `{shebang}`."),
            Some(input.rel_path.to_owned()),
            Some(1),
            false,
        )),
        None => results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Warn,
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
        content,
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]

mod tests;
