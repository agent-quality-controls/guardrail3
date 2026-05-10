#![allow(
    clippy::missing_panics_doc,
    clippy::indexing_slicing,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses; fixture-controlled indexed access panics on the same line that proves the missing value"
)]

use hook_shell_parser_runtime::command_query::any_resolved_command;
use hook_shell_parser_runtime::parse_script;

pub fn assert_script_has_resolved_command(
    script: &str,
    expected_name: &str,
    expected_text_fragment: &str,
) {
    let parsed = parse_script(script);
    assert!(
        any_resolved_command(&parsed, |command| {
            command.command_name() == expected_name
                && command.command_text().contains(expected_text_fragment)
        }),
        "expected resolved command {expected_name:?} containing {expected_text_fragment:?}",
    );
}
