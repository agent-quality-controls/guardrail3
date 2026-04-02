mod rule;
pub use rule::{check};

#[cfg(test)]
use crate::source_facts::SourceCrateFacts;
#[cfg(test)]
pub(crate) use rule::SourceCrateLayerForTest;
#[cfg(test)]
pub(crate) fn run_source_case(
    layer: SourceCrateLayerForTest,
    crate_name: &str,
    rel_dir: &str,
    pub_trait_count: usize,
    public_free_fn_count: usize,
    public_inherent_method_count: usize,
    source_error_rel_path: Option<&str>,
    source_error_message: Option<&str>,
) -> Vec<CheckResult> {
    let source = SourceCrateFacts {
        crate_name: crate_name.to_owned(),
        rel_dir: rel_dir.to_owned(),
        layer: Some(match layer {
            SourceCrateLayerForTest::Ports => crate::dependency_facts::Layer::Ports,
            SourceCrateLayerForTest::Adapters => crate::dependency_facts::Layer::Adapters,
        }),
        pub_trait_count,
        public_free_fn_count,
        public_inherent_method_count,
        source_error_rel_path: source_error_rel_path.map(|value| value.to_owned()),
        source_error_message: source_error_message.map(|value| value.to_owned()),
    };
    let mut results = Vec::new();
    check(&crate::inputs::SourceCrateHexarchInput::new(&source), &mut results);
    results
}
#[cfg(test)]
pub(crate) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}

#[cfg(test)]
mod tests;
