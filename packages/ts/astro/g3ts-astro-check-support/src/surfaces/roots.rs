pub fn scoped_rel_path(app_root_rel_path: &str, rel_path: &str) -> String {
    if app_root_rel_path == "." {
        rel_path.to_owned()
    } else {
        format!("{app_root_rel_path}/{rel_path}")
    }
}

pub fn app_relative_path(rel_path: &str, app_root_rel_path: &str) -> String {
    if app_root_rel_path == "." {
        rel_path.to_owned()
    } else {
        rel_path
            .strip_prefix(&format!("{app_root_rel_path}/"))
            .unwrap_or(rel_path)
            .to_owned()
    }
}

pub fn is_under_app_root(rel_path: &str, app_root_rel_path: &str) -> bool {
    app_root_rel_path == "."
        || rel_path == app_root_rel_path
        || rel_path.starts_with(&format!("{app_root_rel_path}/"))
}

pub fn nearest_app_root<'a>(rel_path: &str, app_root_rel_paths: &'a [String]) -> Option<&'a str> {
    app_root_rel_paths
        .iter()
        .filter(|root| is_under_app_root(rel_path, root))
        .max_by_key(|root| root.len())
        .map(String::as_str)
}
