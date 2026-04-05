mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_16_copyleft_allowlist_tests/mod.rs"]
mod tests;
