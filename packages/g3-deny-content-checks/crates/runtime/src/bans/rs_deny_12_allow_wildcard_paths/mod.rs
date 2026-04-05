mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_12_allow_wildcard_paths_tests/mod.rs"]
mod tests;
