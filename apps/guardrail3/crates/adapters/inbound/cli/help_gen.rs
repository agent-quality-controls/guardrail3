//! Runtime help injection for the CLI.

use clap::Command;
use guardrail3_app_commands::messages::{
    RS_HELP, RS_INIT_HELP, RS_VALIDATE_HELP, TOP_LEVEL_HELP, TS_HELP, TS_INIT_HELP,
    TS_VALIDATE_HELP,
};

/// Inject comprehensive help text into every relevant subcommand.
pub fn inject_help(cmd: Command) -> Command {
    let cmd = cmd
        .after_help(TOP_LEVEL_HELP)
        .after_long_help(TOP_LEVEL_HELP);
    inject_ts_help(inject_rs_help(cmd))
}

fn inject_rs_help(cmd: Command) -> Command {
    cmd.mut_subcommand("rs", |rs| {
        rs.after_help(RS_HELP)
            .after_long_help(RS_HELP)
            .mut_subcommand("validate", |v| {
                v.after_help(RS_VALIDATE_HELP)
                    .after_long_help(RS_VALIDATE_HELP)
            })
            .mut_subcommand("init", |i| {
                i.after_help(RS_INIT_HELP).after_long_help(RS_INIT_HELP)
            })
    })
}

fn inject_ts_help(cmd: Command) -> Command {
    cmd.mut_subcommand("ts", |ts| {
        ts.after_help(TS_HELP)
            .after_long_help(TS_HELP)
            .mut_subcommand("validate", |v| {
                v.after_help(TS_VALIDATE_HELP)
                    .after_long_help(TS_VALIDATE_HELP)
            })
            .mut_subcommand("init", |i| {
                i.after_help(TS_INIT_HELP).after_long_help(TS_INIT_HELP)
            })
    })
}

#[cfg(test)]
#[path = "help_gen_tests.rs"]
mod tests;
