use guardrail3_app_rs_family_hooks_shared::hook_shell::{ExecutableLine, parse_script};
use guardrail3_domain_project_tree::ProjectTree;

use crate::facts::InputFailureFacts;

pub(super) struct MutationHookState {
    pub(super) active: bool,
    pub(super) files: Vec<String>,
}

pub(super) fn collect_mutation_hook_state(
    tree: &ProjectTree,
    root_rel_dir: &str,
    hook_root_rels: &[String],
    input_failures: &mut Vec<InputFailureFacts>,
) -> MutationHookState {
    let mut files = Vec::new();
    let mut active = false;

    for hook_root_rel in hook_root_rels {
        for rel_path in [
            super::join_under_root(hook_root_rel, ".githooks/pre-commit"),
            super::join_under_root(hook_root_rel, "hooks/pre-commit"),
        ] {
            match super::read_cached_or_fs(tree, &rel_path) {
                Ok(Some(content)) => {
                    if parse_script(&content)
                        .executable_lines()
                        .iter()
                        .any(executable_line_has_mutation_hook)
                    {
                        active = true;
                        if hook_root_rel == root_rel_dir {
                            files.push(rel_path.to_owned());
                        }
                    }
                }
                Ok(None) => {}
                Err(read_error) => input_failures.push(InputFailureFacts {
                    root_rel_dir: root_rel_dir.to_owned(),
                    rel_path: rel_path.clone(),
                    message: format!(
                        "Failed to read active hook surface for test-family mutation detection: {read_error}"
                    ),
                }),
            }
        }

        let hook_dir_rel = super::join_under_root(hook_root_rel, ".githooks/pre-commit.d");
        if let Some(dir) = tree.dir_contents(&hook_dir_rel) {
            for file_name in dir.files() {
                let rel_path = ProjectTree::join_rel(&hook_dir_rel, file_name);
                match guardrail3_shared_fs::read_file_err(&tree.abs_path(&rel_path)) {
                    Ok(content) => {
                        if parse_script(&content)
                            .executable_lines()
                            .iter()
                            .any(executable_line_has_mutation_hook)
                        {
                            active = true;
                            if hook_root_rel == root_rel_dir {
                                files.push(rel_path);
                            }
                        }
                    }
                    Err(read_error) => input_failures.push(InputFailureFacts {
                        root_rel_dir: root_rel_dir.to_owned(),
                        rel_path: rel_path.clone(),
                        message: format!(
                            "Failed to read active hook surface for test-family mutation detection: {read_error}"
                        ),
                    }),
                }
            }
        }
    }
    files.sort();
    files.dedup();
    MutationHookState { active, files }
}

fn executable_line_has_mutation_hook(line: &ExecutableLine<'_>) -> bool {
    is_cargo_mutants_command(line.command_text())
}

fn is_cargo_mutants_command(command_text: &str) -> bool {
    let tokens = shell_words(command_text);
    let mut parts = tokens.iter().map(String::as_str).peekable();

    while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
        let _ = parts.next();
    }

    let Some(first) = parts.next() else {
        return false;
    };

    let first = normalize_command_token(first);
    if matches!(first, "exec" | "command") {
        while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
            let _ = parts.next();
        }
        let rest = parts.collect::<Vec<_>>();
        return is_cargo_mutants_from_tokens(&rest);
    }
    if matches!(first, "bash" | "sh" | "zsh") {
        while let Some(flag) = parts.next() {
            if matches!(flag, "-c" | "-lc") {
                return parts.next().is_some_and(is_cargo_mutants_command);
            }
        }
        return false;
    }
    if first == "env" {
        while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
            let _ = parts.next();
        }
        while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
            let _ = parts.next();
        }
        let rest = parts.collect::<Vec<_>>();
        return is_cargo_mutants_from_tokens(&rest);
    }

    let rest = parts.collect::<Vec<_>>();
    is_cargo_mutants_from_tokens_with_first(first, &rest)
}

fn is_cargo_mutants_from_tokens(tokens: &[&str]) -> bool {
    let Some((first, rest)) = tokens.split_first() else {
        return false;
    };
    is_cargo_mutants_from_tokens_with_first(normalize_command_token(first), rest)
}

fn is_cargo_mutants_from_tokens_with_first(first: &str, rest: &[&str]) -> bool {
    let mut parts = rest.iter().copied().peekable();
    match first {
        "cargo" => is_cargo_mutants_invocation(&mut parts),
        "cargo-mutants" => !parts.any(is_help_or_version_flag),
        _ => false,
    }
}

fn is_cargo_mutants_invocation<'a, I>(parts: &mut std::iter::Peekable<I>) -> bool
where
    I: Iterator<Item = &'a str>,
{
    if matches!(parts.peek(), Some(token) if token.starts_with('+')) {
        let _ = parts.next();
    }

    while let Some(token) = parts.peek().copied() {
        if !token.starts_with('-') {
            break;
        }

        let flag = parts.next().unwrap_or_default();
        if is_help_or_version_flag(flag) {
            return false;
        }
        if cargo_global_flag_takes_value(flag) {
            let _ = parts.next();
        }
    }

    parts.next() == Some("mutants") && !parts.any(is_help_or_version_flag)
}

fn cargo_global_flag_takes_value(flag: &str) -> bool {
    matches!(
        flag,
        "--config" | "-Z" | "--manifest-path" | "--color" | "--target" | "--target-dir" | "--jobs"
    )
}

fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
}

fn normalize_command_token(token: &str) -> &str {
    token.rsplit('/').next().unwrap_or(token)
}

fn looks_like_env_assignment(token: &str) -> bool {
    let Some((name, _value)) = token.split_once('=') else {
        return false;
    };
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn shell_words(command_text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut chars = command_text.chars().peekable();
    let mut single_quoted = false;
    let mut double_quoted = false;

    while let Some(ch) = chars.next() {
        match ch {
            '\'' if !double_quoted => {
                single_quoted = !single_quoted;
            }
            '"' if !single_quoted => {
                double_quoted = !double_quoted;
            }
            '\\' if double_quoted => {
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            ch if ch.is_whitespace() && !single_quoted && !double_quoted => {
                if !current.is_empty() {
                    words.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        words.push(current);
    }

    words
}
