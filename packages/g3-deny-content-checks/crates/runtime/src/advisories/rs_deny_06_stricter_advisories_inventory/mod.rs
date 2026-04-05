mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_06_stricter_advisories_inventory_tests/mod.rs"]
mod tests;
