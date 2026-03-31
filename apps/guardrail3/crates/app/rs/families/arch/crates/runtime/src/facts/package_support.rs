pub(super) fn package_name(value: &toml::Value) -> Option<String> {
    value
        .get("package")
        .and_then(|package| package.get("name"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
}

pub(super) fn library_rel_path(base: &str, value: &toml::Value) -> Option<String> {
    let lib = value.get("lib")?;
    let path = lib.get("path")?.as_str()?;
    Some(normalize_path(base, path))
}

pub(super) fn normalize_path(base: &str, rel: &str) -> String {
    let mut parts = base
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    for segment in rel.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                let _ = parts.pop();
            }
            value => parts.push(value),
        }
    }
    parts.join("/")
}

pub(super) fn fallback_name(rel_dir: &str) -> String {
    rel_dir.rsplit('/').next().unwrap_or(rel_dir).to_owned()
}
