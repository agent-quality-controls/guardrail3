mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_05_advisories_baseline_tests/mod.rs"]
mod tests;
