use g3_workspace_crawl::{G3RsWorkspaceCrawl as G3WorkspaceCrawl, entry};
use g3ts_arch_types::{
    G3TsArchFacadeFileState, G3TsArchFacadeItem, G3TsArchFacadeReexport, G3TsArchFacadeSurface,
    G3TsArchManifestState,
};
use std::path::Path;
use tree_sitter::{Node, Parser};

/// Build per-facade source-surface states from `manifest`.
pub(crate) fn facade_states(
    crawl: &G3WorkspaceCrawl,
    manifest: &G3TsArchManifestState,
) -> Vec<G3TsArchFacadeFileState> {
    let G3TsArchManifestState::Parsed { snapshot } = manifest else {
        return Vec::new();
    };

    snapshot
        .declared_entrypoints
        .iter()
        .filter_map(|entrypoint| build_surface_state(crawl, &entrypoint.rel_path))
        .collect()
}

/// Build the per-rel-path facade surface state.
fn build_surface_state(
    crawl: &G3WorkspaceCrawl,
    rel_path: &str,
) -> Option<G3TsArchFacadeFileState> {
    let entry = entry(crawl, rel_path)?;
    if !entry.readable {
        return Some(unreadable(
            rel_path,
            "workspace crawl marked the facade unreadable",
        ));
    }

    let source = match crate::fs::read_to_string(&entry.path.abs_path) {
        Ok(source) => source,
        Err(error) => return Some(unreadable(rel_path, &error.to_string())),
    };

    let mut parser = Parser::new();
    let language = if has_tsx_extension(rel_path) {
        tree_sitter_typescript::LANGUAGE_TSX
    } else {
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT
    };
    if let Err(error) = parser.set_language(&language.into()) {
        return Some(parse_error(rel_path, &error.to_string()));
    }

    let Some(tree) = parser.parse(&source, None) else {
        return Some(parse_error(rel_path, "tree-sitter returned no parse tree"));
    };

    if tree.root_node().has_error() {
        return Some(parse_error(
            rel_path,
            "TypeScript parser reported syntax errors",
        ));
    }

    let (body_items, broad_reexports) = collect_surface(tree.root_node(), &source);
    Some(G3TsArchFacadeFileState::Parsed {
        surface: G3TsArchFacadeSurface {
            rel_path: rel_path.to_owned(),
            body_items,
            broad_reexports,
        },
    })
}

/// Build an `Unreadable` facade state with a reason string.
fn unreadable(rel_path: &str, reason: &str) -> G3TsArchFacadeFileState {
    G3TsArchFacadeFileState::Unreadable {
        rel_path: rel_path.to_owned(),
        reason: reason.to_owned(),
    }
}

/// Build a `ParseError` facade state with a reason string.
fn parse_error(rel_path: &str, reason: &str) -> G3TsArchFacadeFileState {
    G3TsArchFacadeFileState::ParseError {
        rel_path: rel_path.to_owned(),
        reason: reason.to_owned(),
    }
}

/// Whether `rel_path` ends in `.tsx`, case-insensitive.
fn has_tsx_extension(rel_path: &str) -> bool {
    Path::new(rel_path)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("tsx"))
}

/// One-based row line number derived from `node`'s start position.
fn line_one_based(node: Node<'_>) -> usize {
    node.start_position().row.saturating_add(1)
}

/// Pair of (body items, broad reexports) collected from a facade root node.
type FacadeSurfaceParts = (Vec<G3TsArchFacadeItem>, Vec<G3TsArchFacadeReexport>);

/// Walk `root` and produce body items and broad reexports.
fn collect_surface(root: Node<'_>, source: &str) -> FacadeSurfaceParts {
    let mut body_items = Vec::new();
    let mut broad_reexports = Vec::new();
    let mut cursor = root.walk();

    for child in root.children(&mut cursor) {
        let child_kind = child.kind();
        if is_ignored_root_kind(child_kind) {
            continue;
        }

        if child_kind == "export_statement" {
            handle_export_statement(child, source, &mut body_items, &mut broad_reexports);
            continue;
        }

        body_items.push(item_from_node(child, source));
    }
    (body_items, broad_reexports)
}

/// Process one `export_statement` child node.
fn handle_export_statement(
    child: Node<'_>,
    source: &str,
    body_items: &mut Vec<G3TsArchFacadeItem>,
    broad_reexports: &mut Vec<G3TsArchFacadeReexport>,
) {
    let text = child.utf8_text(source.as_bytes()).unwrap_or_default();
    if text.trim_start().starts_with("export *") {
        broad_reexports.push(G3TsArchFacadeReexport {
            line: line_one_based(child),
            source: text.trim().to_owned(),
        });
    }

    let mut export_cursor = child.walk();
    for export_child in child.children(&mut export_cursor) {
        let export_kind = export_child.kind();
        if is_ignored_export_kind(export_kind) || !is_named_export_kind(export_kind) {
            continue;
        }
        body_items.push(item_from_node(export_child, source));
    }
}

/// Build a `G3TsArchFacadeItem` from `node`.
fn item_from_node(node: Node<'_>, source: &str) -> G3TsArchFacadeItem {
    G3TsArchFacadeItem {
        line: line_one_based(node),
        kind: node.kind(),
        name: node
            .utf8_text(source.as_bytes())
            .unwrap_or_default()
            .lines()
            .next()
            .unwrap_or_default()
            .trim()
            .to_owned(),
    }
}

/// Whether the node kind should be skipped at the root scan level.
fn is_ignored_root_kind(kind: &str) -> bool {
    kind == "import_statement" || kind == "empty_statement" || kind == "comment"
}

/// Whether the node kind under an `export_statement` should be skipped.
fn is_ignored_export_kind(kind: &str) -> bool {
    [
        "export_clause",
        "namespace_export",
        "string",
        "{",
        "}",
        "*",
        "from",
        "type",
        "default",
    ]
    .contains(&kind)
}

/// Whether the node kind under an `export_statement` represents a named declaration.
fn is_named_export_kind(kind: &str) -> bool {
    [
        "interface_declaration",
        "type_alias_declaration",
        "function_declaration",
        "generator_function_declaration",
        "class_declaration",
        "lexical_declaration",
        "variable_declaration",
        "enum_declaration",
    ]
    .contains(&kind)
}
