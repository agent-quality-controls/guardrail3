mod rule;
pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_toolchain_03_msrv_consistency_tests/mod.rs"]
mod tests;
