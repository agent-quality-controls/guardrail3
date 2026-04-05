mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_19_allow_registry_baseline_tests/mod.rs"]
mod tests;
