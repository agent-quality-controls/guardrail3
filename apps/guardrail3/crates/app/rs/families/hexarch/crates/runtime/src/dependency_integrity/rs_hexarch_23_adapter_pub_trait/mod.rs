mod rule;
pub use rule::{check};

    pub_trait_count: usize,
    source_error_rel_path: Option<&str>,
    source_error_message: Option<&str>,
) -> Vec<CheckResult> {
    let source = SourceCrateFacts {
        crate_name: crate_name.to_owned(),
        rel_dir: rel_dir.to_owned(),
        layer: Some(crate::dependency_facts::Layer::Adapters),
        pub_trait_count,
        public_free_fn_count: 0,
        public_inherent_method_count: 0,
        source_error_rel_path: source_error_rel_path.map(|value| value.to_owned()),
        source_error_message: source_error_message.map(|value| value.to_owned()),
    };
    let mut results = Vec::new();
    check(&crate::inputs::SourceCrateHexarchInput::new(&source), &mut results);
    results
}

#[cfg(test)]
mod tests;
