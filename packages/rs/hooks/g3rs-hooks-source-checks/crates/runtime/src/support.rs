#![expect(
    clippy::arithmetic_side_effects,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::string_slice,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
use hook_shell_parser::command_query::ResolvedCommand;

/// `inline_comment_text` function.
pub(crate) fn inline_comment_text(line: &str) -> Option<&str> {
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut escaped = false;
    let mut prev_was_whitespace = true;

    for (index, ch) in line.char_indices() {
        let escaped_char = escaped;
        if escaped_char {
            escaped = false;
            continue;
        }

        match ch {
            '\\' if !single_quoted => {
                escaped = true;
            }
            '\'' if !double_quoted => {
                single_quoted = !single_quoted;
            }
            '"' if !single_quoted => {
                double_quoted = !double_quoted;
            }
            '#' if !single_quoted && !double_quoted && prev_was_whitespace => {
                if index == 0 && line.starts_with("#!") {
                    return None;
                }
                return Some(&line[index + 1..]);
            }
            _ => {}
        }
        prev_was_whitespace = ch.is_whitespace() && !escaped_char;
    }

    None
}

/// `cargo_subcommand_tail` function.
pub(crate) fn cargo_subcommand_tail<'a>(
    command: &'a ResolvedCommand,
    subcommand: &str,
) -> Option<&'a [String]> {
    if command.command_name() != "cargo" {
        return None;
    }

    let args = command.args();
    let mut index = 0usize;

    if args.get(index).is_some_and(|token| token.starts_with('+')) {
        index += 1;
    }

    while let Some(token) = args.get(index).map(String::as_str) {
        if !token.starts_with('-') {
            break;
        }

        if is_help_or_version_flag(token) {
            return None;
        }
        if let Some((flag_name, _)) = token.split_once('=')
            && cargo_global_flag_takes_value(flag_name)
        {
            index += 1;
            continue;
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

    if args.get(index).map(String::as_str) != Some(subcommand) {
        return None;
    }

    Some(args.get(index + 1..).unwrap_or(&[]))
}

/// `args_have_help_or_version` function.
pub(crate) fn args_have_help_or_version(args: &[String]) -> bool {
    args.iter().any(|arg| is_help_or_version_flag(arg))
}

/// `is_help_or_version_flag` function.
pub(crate) fn is_help_or_version_flag(token: &str) -> bool {
    matches!(token, "-h" | "--help" | "-V" | "--version")
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
