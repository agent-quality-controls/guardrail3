mod rule;

pub(crate) use rule::check;

#[cfg(test)]
#[path = "../rs_deny_07_graph_all_features_tests/mod.rs"]
mod tests;
