mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_clippy_02_max_struct_bools_tests/mod.rs"]
mod tests;
