use syncpack_config_parser_types::document::{
    SyncpackDependencyDeclarationRef, SyncpackVersionGroup,
};

pub fn first_matching_group_pins_dependency(
    groups: &[SyncpackVersionGroup],
    package_name: Option<&str>,
    declarations: &[SyncpackDependencyDeclarationRef<'_>],
    dependency: &str,
) -> bool {
    let Some(group) = groups
        .iter()
        .find(|group| group_matches_dependency(group, package_name, declarations, dependency))
    else {
        return false;
    };
    group.is_ignored != Some(true) && group.is_banned != Some(true) && group.pin_version.is_some()
}

fn group_matches_dependency(
    group: &SyncpackVersionGroup,
    package_name: Option<&str>,
    declarations: &[SyncpackDependencyDeclarationRef<'_>],
    dependency: &str,
) -> bool {
    group.packages.as_ref().is_none_or(|packages| {
        package_name.is_some_and(|name| pattern_list_matches(packages, name))
    }) && pattern_list_matches(&group.dependencies, dependency)
        && declarations.iter().any(|declaration| {
            declaration.name == dependency
                && (group.dependency_types.is_empty()
                    || pattern_list_matches(&group.dependency_types, declaration.lane))
                && group
                    .specifier_types
                    .as_ref()
                    .is_none_or(|specifier_types| {
                        pattern_list_matches(specifier_types, declaration.specifier_type)
                    })
        })
}

pub fn pattern_list_matches(patterns: &[String], value: &str) -> bool {
    let mut has_positive = false;
    let mut positive_match = false;
    for pattern in patterns {
        if let Some(negative_pattern) = pattern.strip_prefix('!') {
            if pattern_matches(negative_pattern, value) {
                return false;
            }
        } else {
            has_positive = true;
            if pattern_matches(pattern, value) {
                positive_match = true;
            }
        }
    }
    !has_positive || positive_match
}

fn pattern_matches(pattern: &str, value: &str) -> bool {
    pattern == value
        || globset::Glob::new(pattern)
            .map(|glob| glob.compile_matcher().is_match(value))
            .unwrap_or(false)
}
