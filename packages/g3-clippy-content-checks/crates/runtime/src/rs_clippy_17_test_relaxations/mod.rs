mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_clippy_17_test_relaxations_tests/mod.rs"]
mod tests;
