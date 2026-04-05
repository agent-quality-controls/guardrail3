mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_clippy_22_type_complexity_threshold_tests/mod.rs"]
mod tests;
