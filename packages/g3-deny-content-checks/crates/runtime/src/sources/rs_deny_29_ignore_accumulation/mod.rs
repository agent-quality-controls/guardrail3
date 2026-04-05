mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_29_ignore_accumulation_tests/mod.rs"]
mod tests;
