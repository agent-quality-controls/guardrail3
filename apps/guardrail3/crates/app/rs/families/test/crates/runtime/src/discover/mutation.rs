use guardrail3_app_rs_family_hooks_shared::hook_shell::{ExecutableLine, parse_script};
use guardrail3_domain_project_tree::ProjectTree;

pub(super) fn collect_mutation_hook_files(tree: &ProjectTree, root_rel_dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    for rel_path in [
        super::join_under_root(root_rel_dir, ".githooks/pre-commit"),
        super::join_under_root(root_rel_dir, "hooks/pre-commit"),
    ] {
        if let Some(content) = tree.file_content(&rel_path) {
            if parse_script(content)
                .executable_lines()
                .iter()
                .any(executable_line_has_mutation_hook)
            {
                files.push(rel_path.to_owned());
            }
        }
    }
    let hook_dir_rel = super::join_under_root(root_rel_dir, ".githooks/pre-commit.d");
    if let Some(dir) = tree.dir_contents(&hook_dir_rel) {
        for file_name in dir.files() {
            let rel_path = ProjectTree::join_rel(&hook_dir_rel, file_name);
            let Ok(content) = guardrail3_shared_fs::read_file_err(&tree.abs_path(&rel_path)) else {
                continue;
            };
            if parse_script(&content)
                .executable_lines()
                .iter()
                .any(executable_line_has_mutation_hook)
            {
                files.push(rel_path);
            }
        }
    }
    files.sort();
    files
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
    if first == "env" {
        while matches!(parts.peek(), Some(token) if token.starts_with('-')) {
            let _ = parts.next();
        }
        while matches!(parts.peek(), Some(token) if looks_like_env_assignment(token)) {
            let _ = parts.next();
        }
        let Some(next) = parts.next() else {
            return false;
        };
        return match normalize_command_token(next) {
            "cargo" => is_cargo_mutants_invocation(&mut parts),
            "cargo-mutants" => !parts.any(is_help_or_version_flag),
            _ => false,
        };
    }

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
