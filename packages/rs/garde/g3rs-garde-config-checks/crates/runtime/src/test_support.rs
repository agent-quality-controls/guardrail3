pub(crate) fn canonical_clippy_toml() -> String {
    let method_entries = crate::support::CORE_METHOD_BANS
        .iter()
        .chain(std::iter::once(&crate::support::REQWEST_JSON_BAN))
        .chain(crate::support::ADDITIONAL_METHOD_BANS.iter())
        .map(|path| format!("{{ path = \"{path}\" }}"))
        .collect::<Vec<_>>()
        .join(",\n    ");
    let type_entries = crate::support::EXTRACTOR_TYPE_BANS
        .iter()
        .map(|path| format!("{{ path = \"{path}\" }}"))
        .collect::<Vec<_>>()
        .join(",\n    ");

    format!(
        "disallowed-methods = [\n    {method_entries}\n]\n\ndisallowed-types = [\n    {type_entries}\n]\n"
    )
}
