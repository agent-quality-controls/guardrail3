mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_23_skip_hygiene_tests/mod.rs"]
mod tests;
