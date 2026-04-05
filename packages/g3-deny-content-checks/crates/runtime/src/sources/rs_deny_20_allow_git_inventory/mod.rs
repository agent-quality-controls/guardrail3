mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_20_allow_git_inventory_tests/mod.rs"]
mod tests;
