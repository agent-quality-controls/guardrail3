mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_clippy_11_excessive_nesting_threshold_tests/mod.rs"]
mod tests;
