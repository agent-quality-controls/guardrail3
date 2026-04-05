mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_21_tokio_full_ban_tests/mod.rs"]
mod tests;
