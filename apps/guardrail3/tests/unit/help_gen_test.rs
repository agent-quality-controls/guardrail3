use clap::CommandFactory;

use guardrail3::cli::Cli;
use guardrail3::help_gen::inject_help;

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
    assert!(after.contains("PROFILES"), "missing PROFILES in help");
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
    assert!(after.contains("R1"));
    assert!(after.contains("R58"));
    assert!(after.contains("R-DEPS-01"));
    assert!(after.contains("R-TEST-"));
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
    assert!(after.contains("T1"));
    assert!(after.contains("T-TEST-"));
}
