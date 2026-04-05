mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_fmt_02_settings_tests/mod.rs"]
mod tests;
