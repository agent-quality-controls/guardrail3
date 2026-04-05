mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_08_graph_no_default_features_tests/mod.rs"]
mod tests;
