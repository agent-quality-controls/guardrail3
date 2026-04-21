pub fn assert_has_internal_edge(
    input: &g3ts_apparch_types::G3TsApparchConfigChecksInput,
    from_rel_path: &str,
    to_rel_path: &str,
) {
    assert!(
        input
            .internal_edges
            .iter()
            .any(|edge| edge.from_rel_path == from_rel_path && edge.to_rel_path == to_rel_path),
        "expected internal edge `{from_rel_path}` -> `{to_rel_path}`, got {:?}",
        input.internal_edges
    );
}
