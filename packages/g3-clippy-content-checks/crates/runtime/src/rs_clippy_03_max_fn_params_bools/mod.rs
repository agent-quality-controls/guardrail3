mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_clippy_03_max_fn_params_bools_tests/mod.rs"]
mod tests;
