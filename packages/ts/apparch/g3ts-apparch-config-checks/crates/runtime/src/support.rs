use g3ts_apparch_types::{G3TsApparchConfigChecksInput, G3TsApparchInternalEdge, G3TsApparchLayer};
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub(crate) fn layer_label(layer: G3TsApparchLayer) -> &'static str {
    match layer {
        G3TsApparchLayer::App => "app",
        G3TsApparchLayer::Types => "types",
        G3TsApparchLayer::Logic => "logic",
        G3TsApparchLayer::IoInbound => "io/inbound",
        G3TsApparchLayer::IoOutbound => "io/outbound",
    }
}

pub(crate) fn has_layer_files(
    input: &G3TsApparchConfigChecksInput,
    layer: G3TsApparchLayer,
) -> bool {
    input.files.iter().any(|file| file.layer == layer)
}

pub(crate) fn violating_edges<'a>(
    input: &'a G3TsApparchConfigChecksInput,
    from_layer: G3TsApparchLayer,
    forbidden_targets: &[G3TsApparchLayer],
) -> Vec<&'a G3TsApparchInternalEdge> {
    input
        .internal_edges
        .iter()
        .filter(|edge| edge.from_layer == from_layer)
        .filter(|edge| forbidden_targets.contains(&edge.to_layer))
        .collect()
}

pub(crate) fn inventory(id: &str, title: String, message: String) -> G3CheckResult {
    G3CheckResult::new(id.to_owned(), G3Severity::Info, title, message, None, None).into_inventory()
}

pub(crate) fn edge_error(
    id: &str,
    title: String,
    message: String,
    edge: &G3TsApparchInternalEdge,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title,
        message,
        Some(edge.from_rel_path.clone()),
        None,
    )
}
