mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_10_multiple_versions_floor_tests/mod.rs"]
mod tests;
