use std::collections::BTreeMap;

use guardrail3_app_rs_family_mapper::RsProjectSurface as ProjectTree;

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

pub(super) fn collect(tree: &ProjectTree, _crate_tree: &CrateTree) -> ModuleLayoutMap {
    let mut map = BTreeMap::new();

    // Scan all .rs files for mod declarations.
    let all_dirs = tree.all_dir_rels();
    let mut rs_files: Vec<(String, String)> = Vec::new(); // (dir, filename)

    for dir in &all_dirs {
        let Some(entry) = tree.dir_contents(dir) else {
            continue;
        };
        for file in entry.files() {
            if file.ends_with(".rs") {
                rs_files.push((dir.clone(), file.clone()));
            }
        }
    }
    // Also check root directory.
    if let Some(entry) = tree.dir_contents("") {
        for file in entry.files() {
            if file.ends_with(".rs") {
                rs_files.push((String::new(), file.clone()));
            }
        }
    }

    for (dir, filename) in &rs_files {
        let rel_path = ProjectTree::join_rel(dir, filename);
        let content = if let Some(cached) = tree.file_content(&rel_path) {
            cached.to_owned()
        } else {
            let abs = tree.abs_path(&rel_path);
            match guardrail3_shared_fs::read_file_err(&abs) {
                Ok(c) => c,
                Err(_) => continue,
            }
        };

        // Skip test/example directories.
        if is_test_or_example_path(&rel_path) {
            continue;
        }

        let Ok(ast) = syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(&content)) else {
            continue;
        };

        for item in &ast.items {
            let syn::Item::Mod(m) = item else {
                continue;
            };
            // Only care about module declarations without body (mod foo;).
            if m.content.is_some() {
                continue;
            }

            let mod_name = m.ident.to_string();
            let mod_dir = ProjectTree::join_rel(dir, &mod_name);

            // Check if the directory exists and has .rs files.
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

            // Check for sibling foo.rs alongside foo/ (Rust 2018+ convention).
            let sibling_file = format!("{mod_name}.rs");
            let has_sibling = if let Some(parent_entry) = tree.dir_contents(dir) {
                parent_entry.files().iter().any(|f| f == &sibling_file)
            } else {
                false
            };

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

    map
}

fn is_test_or_example_path(rel_path: &str) -> bool {
    let segments: Vec<&str> = rel_path.split('/').collect();
    segments.iter().any(|s| {
        *s == "tests" || *s == "examples" || *s == "benches" || *s == "target"
    })
}
