use std::collections::BTreeMap;

use guardrail3_app_rs_family_view::FamilyView as ProjectTree;

use super::crate_tree::CrateTree;

/// Information about a directory that should be a module (has mod declaration + .rs files).
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields collected for rule expansion and diagnostics.
pub(crate) struct ModuleDir {
    /// Repo-relative path to the directory.
    pub dir_rel: String,
    /// The mod declaration that references this directory (e.g., "pub mod foo;").
    pub mod_decl_file: String,
    /// Line number of the mod declaration.
    pub mod_decl_line: usize,
    /// Whether the mod declaration is public.
    pub is_pub: bool,
    /// Whether mod.rs exists in this directory.
    pub has_mod_rs: bool,
    /// Whether a sibling foo.rs exists alongside the foo/ directory (Rust 2018+ convention).
    pub has_sibling_file: bool,
    /// Number of .rs files in this directory.
    pub rs_file_count: usize,
}

pub(crate) type ModuleLayoutMap = BTreeMap<String, ModuleDir>;

pub(super) fn collect(tree: &ProjectTree, crate_tree: &CrateTree) -> ModuleLayoutMap {
    let mut map = BTreeMap::new();

    // Pass 1: Scan mod declarations for directories referenced via `mod foo;`.
    collect_from_mod_declarations(tree, crate_tree, &mut map);

    // Pass 2: Scan all directories with .rs files under crate src/ trees.
    // This catches directories wired via #[path] that Pass 1 misses,
    // because #[path] uses a different module name than the directory name.
    collect_from_directory_scan(tree, crate_tree, &mut map);

    map
}

fn collect_from_mod_declarations(tree: &ProjectTree, crate_tree: &CrateTree, map: &mut ModuleLayoutMap) {
    // Collect .rs files by walking within each crate's directory tree.
    let mut rs_files: Vec<(String, String)> = Vec::new();
    for node in crate_tree.nodes.values() {
        collect_rs_files_in_dir(tree, &node.rel_dir, &mut rs_files);
    }

    for (dir, filename) in &rs_files {
        let rel_path = ProjectTree::join_rel(dir, filename);
        if is_test_or_example_path(&rel_path) {
            continue;
        }

        let content = if let Some(cached) = tree.file_content(&rel_path) {
            cached.to_owned()
        } else {
            let Some(abs) = tree.abs_path(&rel_path) else { continue };
            match guardrail3_shared_fs::read_file_err(&abs) {
                Ok(c) => c,
                Err(_) => continue,
            }
        };

        let Ok(ast) = syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(&content)) else {
            continue;
        };

        for item in &ast.items {
            let syn::Item::Mod(m) = item else {
                continue;
            };
            if m.content.is_some() {
                continue;
            }

            let mod_name = m.ident.to_string();
            let mod_dir = ProjectTree::join_rel(dir, &mod_name);

            if !tree.dir_exists(&mod_dir) {
                continue;
            }
            let Some(mod_entry) = tree.dir_contents(&mod_dir) else {
                continue;
            };
            let rs_count = mod_entry
                .files()
                .iter()
                .filter(|f| f.ends_with(".rs"))
                .count();
            if rs_count == 0 {
                continue;
            }

            let has_mod_rs = mod_entry.files().iter().any(|f| f == "mod.rs");
            let sibling_file = format!("{mod_name}.rs");
            let has_sibling = tree
                .dir_contents(dir)
                .is_some_and(|e| e.files().iter().any(|f| f == &sibling_file));
            let is_pub = matches!(m.vis, syn::Visibility::Public(_));

            let _ = map.insert(
                mod_dir.clone(),
                ModuleDir {
                    dir_rel: mod_dir,
                    mod_decl_file: rel_path.clone(),
                    mod_decl_line: m.ident.span().start().line,
                    is_pub,
                    has_mod_rs,
                    has_sibling_file: has_sibling,
                    rs_file_count: rs_count,
                },
            );
        }
    }
}

/// Scan directories under crate src/ trees that have .rs files but no mod.rs.
/// These are directories being wired via #[path] or other non-standard mechanisms.
fn collect_from_directory_scan(
    tree: &ProjectTree,
    crate_tree: &CrateTree,
    map: &mut ModuleLayoutMap,
) {
    // Walk within each crate's src/ tree.
    let mut src_dirs = Vec::new();
    for node in crate_tree.nodes.values() {
        let src = ProjectTree::join_rel(&node.rel_dir, "src");
        if tree.dir_exists(&src) {
            collect_dirs_recursive(tree, &src, &mut src_dirs);
        }
    }

    for dir in &src_dirs {
        // Skip if already found by pass 1.
        if map.contains_key(dir) {
            continue;
        }
        if is_test_or_example_path(dir) {
            continue;
        }

        let Some(entry) = tree.dir_contents(dir) else {
            continue;
        };
        let rs_files: Vec<&String> = entry
            .files()
            .iter()
            .filter(|f| f.ends_with(".rs"))
            .collect();
        if rs_files.is_empty() {
            continue;
        }

        let has_mod_rs = rs_files.iter().any(|f| *f == "mod.rs");

        // Only flag directories that are under a crate's src/ tree.
        if !is_under_crate_src(dir, crate_tree) {
            continue;
        }

        // Skip src/ directories themselves — they contain lib.rs/main.rs, not mod.rs.
        if rs_files.iter().any(|f| *f == "lib.rs" || *f == "main.rs") {
            continue;
        }

        // This directory has .rs files, is under a crate src/ tree, and wasn't
        // found by mod-declaration scanning. It's likely wired via #[path].
        let _ = map.insert(
            dir.clone(),
            ModuleDir {
                dir_rel: dir.clone(),
                mod_decl_file: String::new(), // No direct mod declaration found.
                mod_decl_line: 0,
                is_pub: false,
                has_mod_rs,
                has_sibling_file: false,
                rs_file_count: rs_files.len(),
            },
        );
    }
}

fn is_under_crate_src(dir: &str, crate_tree: &CrateTree) -> bool {
    // Walk up to find the nearest crate root, then check if dir is under its src/.
    for node in crate_tree.nodes.values() {
        let src_prefix = if node.rel_dir.is_empty() {
            "src".to_owned()
        } else {
            format!("{}/src", node.rel_dir)
        };
        if dir.starts_with(&src_prefix)
            && (dir.len() == src_prefix.len()
                || dir.as_bytes().get(src_prefix.len()) == Some(&b'/'))
        {
            return true;
        }
    }
    false
}

fn is_test_or_example_path(rel_path: &str) -> bool {
    let segments: Vec<&str> = rel_path.split('/').collect();
    segments.iter().any(|s| {
        *s == "tests" || *s == "examples" || *s == "benches" || *s == "target"
    })
}

fn collect_rs_files_in_dir(tree: &ProjectTree, dir: &str, rs_files: &mut Vec<(String, String)>) {
    let Some(entry) = tree.dir_contents(dir) else {
        return;
    };
    for file in entry.files() {
        if file.ends_with(".rs") {
            rs_files.push((dir.to_owned(), file.clone()));
        }
    }
    for subdir in entry.dirs() {
        let child = ProjectTree::join_rel(dir, subdir);
        collect_rs_files_in_dir(tree, &child, rs_files);
    }
}

fn collect_dirs_recursive(tree: &ProjectTree, dir: &str, result: &mut Vec<String>) {
    result.push(dir.to_owned());
    let Some(entry) = tree.dir_contents(dir) else {
        return;
    };
    for subdir in entry.dirs() {
        let child = ProjectTree::join_rel(dir, subdir);
        collect_dirs_recursive(tree, &child, result);
    }
}
