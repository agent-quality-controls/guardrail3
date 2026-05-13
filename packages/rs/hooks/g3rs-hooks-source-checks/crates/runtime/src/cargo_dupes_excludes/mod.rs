/// `rule` module.
mod rule;

pub(crate) use rule::check;
pub(crate) use rule::script_contains_cargo_dupes_command;
pub(crate) use rule::script_contains_cargo_dupes_with_exclude_tests;
