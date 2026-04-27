pub(super) fn fixture(
    targets: Vec<crate::support::ManualDeserializeImplSite>,
) -> super::super::Fixture {
    super::super::fixture(targets)
}

pub(super) fn target(
    rel_path: &str,
    line: usize,
    type_name: &str,
    needs_validate: bool,
    has_validate: bool,
) -> crate::support::ManualDeserializeImplSite {
    crate::support::ManualDeserializeImplSite {
        rel_path: rel_path.to_owned(),
        line,
        type_name: type_name.to_owned(),
        needs_validate,
        has_validate,
    }
}
