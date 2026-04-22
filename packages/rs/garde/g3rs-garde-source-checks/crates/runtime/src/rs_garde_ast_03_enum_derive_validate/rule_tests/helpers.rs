pub(super) fn fixture(
    targets: Vec<crate::support::DerivedBoundaryTypeSite>,
) -> super::super::Fixture {
    super::super::fixture(targets)
}

pub(super) fn target(
    rel_path: &str,
    line: usize,
    name: &str,
    boundary_macros: &[&str],
    has_validate: bool,
) -> crate::support::DerivedBoundaryTypeSite {
    crate::support::DerivedBoundaryTypeSite {
        rel_path: rel_path.to_owned(),
        line,
        name: name.to_owned(),
        boundary_kind: g3rs_garde_types::G3RsGardeBoundaryKind::Enum,
        boundary_macros: boundary_macros
            .iter()
            .map(|item| (*item).to_owned())
            .collect(),
        has_validate,
    }
}
