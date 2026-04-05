mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_18_unknown_sources_policy_tests/mod.rs"]
mod tests;
