use crate::view::CrawlView;

use super::{is_inside, is_test_or_example_path, should_stop_at_nested_crate};

pub(super) fn collect_structure_root_dirs(
    view: &CrawlView<'_>,
    crate_dir: &str,
    lib_rs_rel: Option<&str>,
    has_main_rs: bool,
) -> Vec<String> {
    let mut roots = Vec::new();

    if let Some(lib_rs_rel) = lib_rs_rel {
        roots.push(parent_dir_of_file(lib_rs_rel));
    }
    if has_main_rs {
        roots.push(CrawlView::join_rel(crate_dir, "src"));
    }
    if roots.is_empty() {
        let src_dir = CrawlView::join_rel(crate_dir, "src");
        if view.dir_contents(&src_dir).is_some() {
            roots.push(src_dir);
        } else {
            roots.push(crate_dir.to_owned());
        }
    }

    roots.sort();
    roots.dedup();
    roots
}

pub(super) fn measure_max_sibling_counts(
    view: &CrawlView<'_>,
    dir: &str,
    crate_dir: &str,
    crate_dirs: &[&str],
    structure_roots: &[String],
) -> (usize, usize) {
    let Some(entry) = view.dir_contents(dir) else {
        return (0, 0);
    };
    let sibling_rs_file_count = entry
        .files()
        .iter()
        .filter(|file| file.ends_with(".rs"))
        .count();
    let sibling_dir_count = entry
        .dirs()
        .iter()
        .filter(|subdir| {
            let child_dir = CrawlView::join_rel(dir, subdir);
            !should_skip_structure_dir(view, crate_dir, &child_dir, crate_dirs, structure_roots)
        })
        .count();

    entry.dirs().iter().fold(
        (sibling_rs_file_count, sibling_dir_count),
        |(max_rs, max_dirs), subdir| {
            let child_dir = CrawlView::join_rel(dir, subdir);
            if should_skip_structure_dir(view, crate_dir, &child_dir, crate_dirs, structure_roots) {
                return (max_rs, max_dirs);
            }

            let (child_max_rs, child_max_dirs) = measure_max_sibling_counts(
                view,
                &child_dir,
                crate_dir,
                crate_dirs,
                structure_roots,
            );
            (max_rs.max(child_max_rs), max_dirs.max(child_max_dirs))
        },
    )
}

pub(super) fn measure_module_depth(
    view: &CrawlView<'_>,
    crate_dir: &str,
    base_dir: &str,
    crate_dirs: &[&str],
    structure_roots: &[String],
) -> usize {
    measure_depth_recursive(view, crate_dir, base_dir, crate_dirs, structure_roots, 0)
}

fn measure_depth_recursive(
    view: &CrawlView<'_>,
    crate_dir: &str,
    dir: &str,
    crate_dirs: &[&str],
    structure_roots: &[String],
    depth: usize,
) -> usize {
    let Some(entry) = view.dir_contents(dir) else {
        return depth;
    };
    let has_rs = entry.files().iter().any(|file| file.ends_with(".rs"));
    let current = if has_rs { depth } else { 0 };
    let max_child = entry
        .dirs()
        .iter()
        .map(|subdir| {
            let child_dir = CrawlView::join_rel(dir, subdir);
            if should_skip_structure_dir(view, crate_dir, &child_dir, crate_dirs, structure_roots) {
                0
            } else {
                measure_depth_recursive(
                    view,
                    crate_dir,
                    &child_dir,
                    crate_dirs,
                    structure_roots,
                    depth + 1,
                )
            }
        })
        .max()
        .unwrap_or(0);
    current.max(max_child)
}

fn parent_dir_of_file(rel_path: &str) -> String {
    rel_path
        .rsplit_once('/')
        .map_or(String::new(), |(prefix, _)| prefix.to_owned())
}

fn should_skip_structure_dir(
    view: &CrawlView<'_>,
    crate_dir: &str,
    child_dir: &str,
    crate_dirs: &[&str],
    structure_roots: &[String],
) -> bool {
    should_stop_at_nested_crate(view, crate_dir, child_dir, crate_dirs)
        || is_test_or_example_path(child_dir)
        || is_under_inactive_src_root(crate_dir, child_dir, structure_roots)
}

fn is_under_inactive_src_root(crate_dir: &str, rel_path: &str, structure_roots: &[String]) -> bool {
    let src_dir = CrawlView::join_rel(crate_dir, "src");
    structure_roots.iter().all(|root| root != &src_dir)
        && (rel_path == src_dir || is_inside(rel_path, &src_dir))
}
