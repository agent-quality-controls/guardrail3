mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_28_unknown_keys_tests/mod.rs"]
mod tests;
