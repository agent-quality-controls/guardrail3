use std::collections::BTreeSet;
use std::path::Path;

use crate::ports::outbound::FileSystem;

/// Load `.gitignore` patterns from the project root.
///
/// Returns a set of directory/file names that should be excluded from scanning.
/// Only handles simple patterns (directory names, file globs). Does not handle
/// negation (`!`), nested gitignores, or complex glob patterns.
pub fn load_gitignore_dirs(fs: &dyn FileSystem, root: &Path) -> BTreeSet<String> {
    let mut ignored = BTreeSet::new();

    let gitignore_path = root.join(".gitignore");
    let Some(content) = fs.read_file(&gitignore_path) else {
        return ignored;
    };

    for line in content.lines() {
        let trimmed = line.trim();
        // Skip comments and empty lines
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('!') {
            continue;
        }
        // Strip trailing slash (directory marker)
        let name = trimmed.trim_end_matches('/');
        // Add simple names (no path separators, no complex globs)
        // e.g., "node_modules", "legacy", "dist", "build", ".next", "coverage"
        if !name.contains('/') && !name.contains('*') {
            let _ = ignored.insert(name.to_owned());
        }
    }

    ignored
}

/// Check if a walkdir entry's directory name is in the gitignore set.
pub fn is_gitignored(entry: &walkdir::DirEntry, ignored: &BTreeSet<String>) -> bool {
    if !entry.file_type().is_dir() {
        return false;
    }
    let name = entry.file_name().to_string_lossy();
    ignored.contains(name.as_ref())
}
