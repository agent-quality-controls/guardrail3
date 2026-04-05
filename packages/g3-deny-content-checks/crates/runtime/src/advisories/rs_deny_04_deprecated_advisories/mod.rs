mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_04_deprecated_advisories_tests/mod.rs"]
mod tests;
