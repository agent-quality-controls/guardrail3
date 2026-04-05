mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_24_ignore_hygiene_tests/mod.rs"]
mod tests;
