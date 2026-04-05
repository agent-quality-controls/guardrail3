mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_15_confidence_threshold_tests/mod.rs"]
mod tests;
