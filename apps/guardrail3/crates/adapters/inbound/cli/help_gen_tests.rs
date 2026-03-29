use clap::CommandFactory;

use crate::cli::Cli;

use super::inject_help;

fn require_subcommand<'a>(command: &'a clap::Command, name: &str) -> &'a clap::Command {
    let Some(subcommand) = command.get_subcommands().find(|c| c.get_name() == name) else {
        panic!("missing `{name}` subcommand");
    };
    subcommand
}

fn require_after_help(command: &clap::Command) -> String {
    let Some(after_help) = command.get_after_help() else {
        panic!("after_help should be set");
    };
    after_help.to_string()
}

#[test]
fn inject_help_does_not_panic() {
    let cmd = Cli::command();
    let cmd = inject_help(cmd);
    let after = require_after_help(&cmd);
    assert!(
        after.contains("COMMAND REFERENCE"),
        "missing COMMAND REFERENCE"
    );
    assert!(after.contains("SETUP GUIDE"), "missing SETUP GUIDE in help");
}

#[test]
fn rs_validate_help_contains_check_ids() {
    let cmd = inject_help(Cli::command());
    let rs = require_subcommand(&cmd, "rs");
    let validate = require_subcommand(rs, "validate");
    let after = require_after_help(validate);
    assert!(after.contains("RUST VALIDATION FAMILIES"));
    assert!(after.contains("hooks-rs"));
    assert!(after.contains("RS-*"));
    assert!(after.contains("HOOK-*"));
}

#[test]
fn ts_validate_help_contains_check_ids() {
    let cmd = inject_help(Cli::command());
    let ts = require_subcommand(&cmd, "ts");
    let validate = require_subcommand(ts, "validate");
    let after = require_after_help(validate);
    assert!(after.contains("TYPESCRIPT VALIDATION"));
    assert!(after.contains("--staged"));
}

#[test]
fn rs_init_help_contains_profiles() {
    let cmd = inject_help(Cli::command());
    let rs = require_subcommand(&cmd, "rs");
    let init = require_subcommand(rs, "init");
    let after = require_after_help(init);
    assert!(after.contains("PROFILES"));
    assert!(after.contains("guardrail3 rs generate"));
}
