pub(super) fn fixture(
    query_as_macros: Vec<crate::support::QueryAsMacroSite>,
) -> super::super::Fixture {
    super::super::fixture(query_as_macros)
}

pub(super) fn macro_use(
    rel_path: &str,
    line: usize,
    macro_name: &str,
    policy_resolved: bool,
    waiver_reason: Option<&str>,
) -> crate::support::QueryAsMacroSite {
    crate::support::QueryAsMacroSite {
        rel_path: rel_path.to_owned(),
        line,
        macro_name: macro_name.to_owned(),
        policy_resolved,
        waiver_reason: waiver_reason.map(str::to_owned),
    }
}
