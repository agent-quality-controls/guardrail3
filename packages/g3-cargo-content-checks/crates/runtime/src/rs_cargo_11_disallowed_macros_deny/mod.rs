mod rule;
pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_cargo_11_disallowed_macros_deny_tests/mod.rs"]
mod tests;
