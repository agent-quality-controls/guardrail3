/// Parser helpers for the family's structured inputs.
mod parse;
/// Family entry point that runs all rules.
mod run;

pub(crate) use run::analyze_source_files;
