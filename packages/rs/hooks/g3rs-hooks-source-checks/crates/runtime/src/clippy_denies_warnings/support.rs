#![expect(
    clippy::arithmetic_side_effects,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::if_same_then_else,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::indexing_slicing,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::unused_peekable,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
use hook_shell_parser::command_query::ShellEnvState;

/// `EnvState` struct.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct EnvState {
    /// `rustflags` item.
    pub(super) rustflags: Option<String>,
}

impl ShellEnvState for EnvState {
    fn apply_assignment(&mut self, name: &str, value: &str) {
        if name == "RUSTFLAGS" {
            self.rustflags = Some(value.to_owned());
        }
    }

    fn unset(&mut self, name: &str) {
        if name == "RUSTFLAGS" {
            self.rustflags = None;
        }
    }

    fn clear(&mut self) {
        self.rustflags = None;
    }
}

/// `LintEffect` struct.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct LintEffect {
    /// `denied` item.
    pub(super) denied: bool,
    /// `softened` item.
    pub(super) softened: bool,
}

/// `cargo_clippy_denies_warnings` function.
pub(super) fn cargo_clippy_denies_warnings(args: &[String], env_state: &EnvState) -> bool {
    let mut index = 0usize;

    if args.get(index).is_some_and(|token| token.starts_with('+')) {
        index += 1;
    }

    while let Some(token) = args.get(index).map(String::as_str) {
        if !token.starts_with('-') {
            break;
        }

        if is_help_or_version_flag(token) {
            return false;
        }
        match token.split_once('=') {
            Some((flag_name, _)) if cargo_global_flag_takes_value(flag_name) => {
                index += 1;
                continue;
            }
            _ => {}
        }
        if matches!(token.strip_prefix("-j"), Some(value) if !value.is_empty()) {
            index += 1;
            continue;
        }
        if cargo_global_flag_takes_value(token) {
            index += 2;
            continue;
        }

        index += 1;
    }

    if args.get(index).map(String::as_str) != Some("clippy") {
        return false;
    }
    index += 1;

    let mut combined_tokens = env_state
        .rustflags
        .as_deref()
        .map(rustflags_tokens)
        .unwrap_or_default();

    while let Some(token) = args.get(index).map(String::as_str) {
        if is_help_or_version_flag(token) {
            return false;
        }
        if token == "--" {
            combined_tokens.extend(args.get(index + 1..).unwrap_or(&[]).iter().cloned());
            break;
        }
        index += 1;
    }

    let token_refs = combined_tokens
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let effect = lint_effect_from_tokens(&token_refs);
    effect.denied && !effect.softened
}

/// `rustflags_tokens` function.
fn rustflags_tokens(value: &str) -> Vec<String> {
    shell_words(value)
}

/// `lint_effect_from_tokens` function.
fn lint_effect_from_tokens(tokens: &[&str]) -> LintEffect {
    let mut effect = LintEffect::default();
    let mut warnings_level = None;
    let mut i = 0usize;

    while i < tokens.len() {
        let token = tokens[i];
        if let Some(level) = split_warning_level(token, tokens.get(i + 1).copied()) {
            warnings_level = Some(level);
            i += 1;
        } else if split_force_warn(token, tokens.get(i + 1).copied()) {
            effect.softened = true;
            i += 1;
        } else if soften_from_split_cap_lints(token, tokens.get(i + 1).copied()) {
            effect.softened = true;
            i += 1;
        } else if let Some(level) = inline_warning_level(token) {
            warnings_level = Some(level);
        } else if inline_force_warn(token) || soften_from_inline_cap_lints(token) {
            effect.softened = true;
        }
        i += 1;
    }

    match warnings_level {
        Some("deny" | "forbid") => effect.denied = true,
        Some("allow" | "warn") => effect.softened = true,
        _ => {}
    }

    effect
}

/// `split_warning_level` function.
fn split_warning_level(token: &str, next: Option<&str>) -> Option<&'static str> {
    let level = match token {
        "-D" | "--deny" => "deny",
        "-A" | "--allow" => "allow",
        "-W" | "--warn" => "warn",
        "-F" | "--forbid" => "forbid",
        _ => return None,
    };
    (next == Some("warnings")).then_some(level)
}

/// `split_force_warn` function.
fn split_force_warn(token: &str, next: Option<&str>) -> bool {
    token == "--force-warn" && next == Some("warnings")
}

/// `soften_from_split_cap_lints` function.
fn soften_from_split_cap_lints(token: &str, next: Option<&str>) -> bool {
    token == "--cap-lints" && next.is_some_and(|value| !matches!(value, "deny" | "forbid"))
}

/// `inline_warning_level` function.
fn inline_warning_level(token: &str) -> Option<&'static str> {
    match token {
        "-Dwarnings" | "--deny=warnings" => Some("deny"),
        "-Awarnings" | "--allow=warnings" => Some("allow"),
        "-Wwarnings" | "--warn=warnings" => Some("warn"),
        "-Fwarnings" | "--forbid=warnings" => Some("forbid"),
        _ => None,
    }
}

/// `inline_force_warn` function.
fn inline_force_warn(token: &str) -> bool {
    token == "--force-warn=warnings"
}

/// `soften_from_inline_cap_lints` function.
fn soften_from_inline_cap_lints(token: &str) -> bool {
    token
        .strip_prefix("--cap-lints=")
        .is_some_and(|value| !matches!(value, "deny" | "forbid"))
}

/// `cargo_global_flag_takes_value` function.
fn cargo_global_flag_takes_value(flag: &str) -> bool {
    matches!(
        flag,
        "--config"
            | "-Z"
            | "--manifest-path"
            | "--color"
            | "--target"
            | "--target-dir"
            | "--jobs"
            | "-j"
            | "-C"
    )
}

/// `is_help_or_version_flag` function.
fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
}

/// `shell_words` function.
fn shell_words(command_text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let chars = command_text.chars().peekable();
    let mut current = String::new();
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut escaped = false;

    for ch in chars {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }
        match ch {
            '\\' if !single_quoted => escaped = true,
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
            c if c.is_whitespace() && !single_quoted && !double_quoted => {
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
