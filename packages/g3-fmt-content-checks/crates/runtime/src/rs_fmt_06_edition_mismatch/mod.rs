mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_fmt_06_edition_mismatch_tests/mod.rs"]
mod tests;
