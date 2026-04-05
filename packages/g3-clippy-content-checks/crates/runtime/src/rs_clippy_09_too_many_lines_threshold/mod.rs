mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_clippy_09_too_many_lines_threshold_tests/mod.rs"]
mod tests;
