mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_22_extra_feature_bans_inventory_tests/mod.rs"]
mod tests;
