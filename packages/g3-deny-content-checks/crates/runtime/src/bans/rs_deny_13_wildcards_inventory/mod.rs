mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_13_wildcards_inventory_tests/mod.rs"]
mod tests;
