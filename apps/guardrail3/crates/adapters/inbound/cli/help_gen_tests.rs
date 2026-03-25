use clap::CommandFactory;

use crate::cli::Cli;

use super::inject_help;

#[test]
#[allow(clippy::expect_used)] // reason: test assertions
fn inject_help_does_not_panic() {
    let cmd = Cli::command();
    let cmd = inject_help(cmd);
    let after = cmd.get_after_help().expect("after_help set").to_string();
    assert!(
        after.contains("COMMAND REFERENCE"),
        "missing COMMAND REFERENCE"
    );
    assert!(after.contains("SETUP GUIDE"), "missing SETUP GUIDE in help");
}

#[test]
#[allow(clippy::expect_used)] // reason: test assertions
fn rs_validate_help_contains_check_ids() {
    let cmd = inject_help(Cli::command());
    let rs = cmd
        .get_subcommands()
        .find(|c| c.get_name() == "rs")
        .expect("rs subcommand");
    let validate = rs
        .get_subcommands()
        .find(|c| c.get_name() == "validate")
        .expect("validate subcommand");
    let after = validate
        .get_after_help()
        .expect("after_help set")
        .to_string();
    assert!(after.contains("RUST VALIDATION FAMILIES"));
    assert!(after.contains("hooks-rs"));
    assert!(after.contains("RS-*"));
    assert!(after.contains("HOOK-*"));
}

#[test]
#[allow(clippy::expect_used)] // reason: test assertions
fn ts_validate_help_contains_check_ids() {
    let cmd = inject_help(Cli::command());
    let ts = cmd
        .get_subcommands()
        .find(|c| c.get_name() == "ts")
        .expect("ts subcommand");
    let validate = ts
        .get_subcommands()
        .find(|c| c.get_name() == "validate")
        .expect("validate subcommand");
    let after = validate
        .get_after_help()
        .expect("after_help set")
        .to_string();
    assert!(after.contains("TYPESCRIPT VALIDATION"));
    assert!(after.contains("--staged"));
}

#[test]
#[allow(clippy::expect_used)] // reason: test assertions
fn rs_init_help_contains_profiles() {
    let cmd = inject_help(Cli::command());
    let rs = cmd
        .get_subcommands()
        .find(|c| c.get_name() == "rs")
        .expect("rs subcommand");
    let init = rs
        .get_subcommands()
        .find(|c| c.get_name() == "init")
        .expect("init subcommand");
    let after = init.get_after_help().expect("after_help set").to_string();
    assert!(after.contains("PROFILES"));
    assert!(after.contains("guardrail3 rs generate"));
}
