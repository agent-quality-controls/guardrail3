pub(super) fn fixture(
    boundary_fields: Vec<crate::support::BoundaryFieldSite>,
) -> super::super::Fixture {
    super::super::fixture(boundary_fields)
}

pub(super) fn field(
    rel_path: &str,
    line: usize,
    boundary_name: &str,
    field_name: &str,
) -> crate::support::BoundaryFieldSite {
    crate::support::BoundaryFieldSite {
        rel_path: rel_path.to_owned(),
        line,
        boundary_name: boundary_name.to_owned(),
        field_name: field_name.to_owned(),
        field_type: "String".to_owned(),
        requires_field_validation: true,
        nested_validated: false,
        has_garde_skip: false,
        has_garde_dive: false,
        has_meaningful_garde_rule: true,
        uses_context: true,
        boundary_has_context: false,
    }
}
