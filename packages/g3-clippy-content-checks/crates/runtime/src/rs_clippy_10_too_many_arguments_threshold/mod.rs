mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_clippy_10_too_many_arguments_threshold_tests/mod.rs"]
mod tests;
