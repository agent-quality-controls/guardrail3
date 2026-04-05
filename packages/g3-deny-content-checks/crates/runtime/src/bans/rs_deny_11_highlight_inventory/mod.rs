mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_11_highlight_inventory_tests/mod.rs"]
mod tests;
