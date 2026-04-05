mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_27_duplicate_entries_tests/mod.rs"]
mod tests;
