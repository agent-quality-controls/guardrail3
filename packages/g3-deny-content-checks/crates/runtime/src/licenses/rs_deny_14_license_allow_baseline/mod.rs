mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_14_license_allow_baseline_tests/mod.rs"]
mod tests;
