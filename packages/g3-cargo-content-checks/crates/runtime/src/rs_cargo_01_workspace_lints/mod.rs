mod rule;
pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_cargo_01_workspace_lints_tests/mod.rs"]
mod tests;
