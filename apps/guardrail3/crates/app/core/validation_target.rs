use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationTarget {
    project_root: PathBuf,
    requested_path: PathBuf,
    scope_rel: Option<String>,
}

impl ValidationTarget {
    #[must_use]
    pub fn project_root(&self) -> &Path {
        &self.project_root
    }

    #[must_use]
    pub fn requested_path(&self) -> &Path {
        &self.requested_path
    }

    #[must_use]
    pub fn scope_rel(&self) -> Option<&str> {
        self.scope_rel.as_deref()
    }
}

#[derive(Debug)]
pub struct ResolveValidationTargetError {
    path: PathBuf,
    source: std::io::Error,
}

impl std::fmt::Display for ResolveValidationTargetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: cannot resolve path '{}': {}",
            self.path.display(),
            self.source
        )
    }
}

impl std::error::Error for ResolveValidationTargetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

pub fn resolve_validation_target(path: &Path) -> Result<ValidationTarget, ResolveValidationTargetError> {
    let requested_path = path
        .canonicalize()
        .map_err(|source| ResolveValidationTargetError {
            path: path.to_path_buf(),
            source,
        })?;
    let anchor = requested_anchor(&requested_path);
    let project_root = detect_project_root(anchor).unwrap_or_else(|| anchor.to_path_buf());
    let scope_rel = requested_path
        .strip_prefix(&project_root)
        .ok()
        .and_then(normalize_scope_rel);

    Ok(ValidationTarget {
        project_root,
        requested_path,
        scope_rel,
    })
}

fn requested_anchor(requested_path: &Path) -> &Path {
    if requested_path.is_dir() {
        requested_path
    } else {
        requested_path.parent().unwrap_or(requested_path)
    }
}

fn detect_project_root(anchor: &Path) -> Option<PathBuf> {
    let ancestors = anchor.ancestors().collect::<Vec<_>>();

    ancestors
        .iter()
        .copied()
        .find(|candidate| candidate.join(".git").exists())
        .or_else(|| {
            ancestors
                .iter()
                .copied()
                .find(|candidate| candidate.join("guardrail3.toml").exists())
        })
        .or_else(|| {
            ancestors.iter().rev().copied().find(|candidate| {
                candidate.join("Cargo.toml").exists() || candidate.join("package.json").exists()
            })
        })
        .map(Path::to_path_buf)
}

fn normalize_scope_rel(rel: &Path) -> Option<String> {
    let normalized = rel.to_string_lossy().replace('\\', "/");
    let normalized = normalized.trim_matches('/').to_owned();
    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
}

#[cfg(test)]
mod tests {
    use super::resolve_validation_target;

    #[test]
    fn resolves_repo_root_and_subdir_scope() {
        let temp = tempfile::tempdir().expect("create temp dir for validation-target test");
        let root = temp.path();
        let expected_root = root
            .canonicalize()
            .expect("canonicalize temp dir for validation-target test");
        std::fs::create_dir_all(root.join(".git")).expect("create git dir");
        std::fs::create_dir_all(root.join("apps/backend/src")).expect("create source dir");
        std::fs::write(root.join("apps/backend/src/lib.rs"), "pub fn ok() {}\n")
            .expect("write test Rust source file");

        let resolved = resolve_validation_target(&root.join("apps/backend/src"))
            .expect("resolve directory validation target");

        assert_eq!(resolved.project_root(), expected_root.as_path());
        assert_eq!(resolved.scope_rel(), Some("apps/backend/src"));
    }

    #[test]
    fn uses_requested_parent_for_file_targets() {
        let temp = tempfile::tempdir().expect("create temp dir for file-target test");
        let root = temp.path();
        let expected_root = root
            .canonicalize()
            .expect("canonicalize temp dir for file-target test");
        std::fs::create_dir_all(root.join(".git")).expect("create git dir");
        std::fs::create_dir_all(root.join("apps/backend/src")).expect("create source dir");
        std::fs::write(root.join("apps/backend/src/lib.rs"), "pub fn ok() {}\n")
            .expect("write test Rust source file");

        let resolved = resolve_validation_target(&root.join("apps/backend/src/lib.rs"))
            .expect("resolve file validation target");

        assert_eq!(resolved.project_root(), expected_root.as_path());
        assert_eq!(resolved.scope_rel(), Some("apps/backend/src/lib.rs"));
    }
}
