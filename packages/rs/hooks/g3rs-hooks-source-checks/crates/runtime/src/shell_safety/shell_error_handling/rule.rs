#![expect(
    clippy::arithmetic_side_effects,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
#![expect(
    clippy::type_complexity,
    reason = "shell script parser requires byte indexing and arithmetic for tokenization"
)]
use crate::compat::{G3CheckResult, G3Severity};
use hook_shell_parser::command_query::{ResolvedCommand, any_resolved_command};

use crate::inputs::ExecutableCommandContextInput;

/// `ID` constant.
const ID: &str = "g3rs-hooks/shell-error-handling";

/// `check` function.
pub(crate) fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<G3CheckResult>) {
    let has_shell_error_handling =
        any_resolved_command(input.parsed, has_shell_error_handling_command);

    if has_shell_error_handling {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Info,
                "`.githooks/pre-commit` enables fail-closed shell options".to_owned(),
                "`.githooks/pre-commit` enables `set -e`-style shell error handling before running checks.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
    } else {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            // Reason: without `set -e`, any fail-closed claim is a lie; missing must gate.
            G3Severity::Error,
            "missing fail-closed shell options in `.githooks/pre-commit`".to_owned(),
            "Add `set -euo pipefail` near the top of `.githooks/pre-commit`, before any real checks run. Without `-e`, a failing command can be ignored and the hook can continue past a broken check.".to_owned(),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    }
}

/// `has_shell_error_handling_command` function.
fn has_shell_error_handling_command(command: &ResolvedCommand) -> bool {
    if command.command_name() != "set" {
        return false;
    }

    set_command_enables_errexit(command.args())
}

#[expect(
    clippy::excessive_nesting,
    reason = "shell script parser is inherently iterative with byte-walking loops"
)]
/// `set_command_enables_errexit` function.
fn set_command_enables_errexit(args: &[String]) -> bool {
    let mut errexit_enabled = false;
    let mut index = 0usize;

    while let Some(arg) = args.get(index).map(String::as_str) {
        if arg == "--" {
            break;
        }

        let Some((enable, short_flags)) = short_option_cluster(arg) else {
            break;
        };

        let mut chars = short_flags.chars().peekable();
        while let Some(flag) = chars.next() {
            match flag {
                'e' => errexit_enabled = enable,
                'o' => {
                    if chars.peek().is_some() {
                        return false;
                    }

                    let Some(option_name) = args.get(index + 1).map(String::as_str) else {
                        return false;
                    };

                    if option_name == "errexit" {
                        errexit_enabled = enable;
                    }

                    index += 1;
                }
                _ => {}
            }
        }

        index += 1;
    }

    errexit_enabled
}

/// `short_option_cluster` function.
fn short_option_cluster(token: &str) -> Option<(bool, &str)> {
    let (prefix, rest) = token.split_at(1);
    match prefix {
        "-" if !rest.is_empty() && rest != "-" => Some((true, rest)),
        "+" if !rest.is_empty() => Some((false, rest)),
        _ => None,
    }
}
