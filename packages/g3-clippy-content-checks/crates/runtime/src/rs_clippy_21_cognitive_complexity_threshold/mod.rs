mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_clippy_21_cognitive_complexity_threshold_tests/mod.rs"]
mod tests;
