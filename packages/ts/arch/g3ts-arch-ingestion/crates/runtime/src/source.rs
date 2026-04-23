use g3_workspace_crawl::{G3RsWorkspaceCrawl as G3WorkspaceCrawl, entry};
use g3ts_arch_types::{
    G3TsArchFacadeFileState, G3TsArchFacadeItem, G3TsArchFacadeReexport, G3TsArchFacadeSurface,
    G3TsArchManifestState,
};
use tree_sitter::Parser;

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

fn build_surface_state(
    crawl: &G3WorkspaceCrawl,
    rel_path: &str,
) -> Option<G3TsArchFacadeFileState> {
    let entry = entry(crawl, rel_path)?;
    if !entry.readable {
        return Some(G3TsArchFacadeFileState::Unreadable {
            rel_path: rel_path.to_owned(),
            reason: "workspace crawl marked the facade unreadable".to_owned(),
        });
    }

    let source = match crate::fs::read_to_string(&entry.path.abs_path) {
        Ok(source) => source,
        Err(error) => {
            return Some(G3TsArchFacadeFileState::Unreadable {
                rel_path: rel_path.to_owned(),
                reason: error.to_string(),
            });
        }
    };
    let mut parser = Parser::new();
    let language = if rel_path.ends_with(".tsx") {
        tree_sitter_typescript::LANGUAGE_TSX
    } else {
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT
    };
    if let Err(error) = parser.set_language(&language.into()) {
        return Some(G3TsArchFacadeFileState::ParseError {
            rel_path: rel_path.to_owned(),
            reason: error.to_string(),
        });
    }

    let Some(tree) = parser.parse(&source, None) else {
        return Some(G3TsArchFacadeFileState::ParseError {
            rel_path: rel_path.to_owned(),
            reason: "tree-sitter returned no parse tree".to_owned(),
        });
    };

    if tree.root_node().has_error() {
        return Some(G3TsArchFacadeFileState::ParseError {
            rel_path: rel_path.to_owned(),
            reason: "TypeScript parser reported syntax errors".to_owned(),
        });
    }

    let mut body_items = Vec::new();
    let mut broad_reexports = Vec::new();
    let root = tree.root_node();
    let mut cursor = root.walk();

    for child in root.children(&mut cursor) {
        let child_kind = child.kind();
        if is_ignored_root_kind(child_kind) {
            continue;
        }

        if child_kind == "export_statement" {
            let text = child.utf8_text(source.as_bytes()).unwrap_or_default();
            if text.trim_start().starts_with("export *") {
                broad_reexports.push(G3TsArchFacadeReexport {
                    line: child.start_position().row + 1,
                    source: text.trim().to_owned(),
                });
            }

            let mut export_cursor = child.walk();
            for export_child in child.children(&mut export_cursor) {
                let export_kind = export_child.kind();
                if is_ignored_export_kind(export_kind) {
                    continue;
                }
                if !is_named_export_kind(export_kind) {
                    continue;
                }

                body_items.push(G3TsArchFacadeItem {
                    line: export_child.start_position().row + 1,
                    kind: export_kind,
                    name: export_child
                        .utf8_text(source.as_bytes())
                        .unwrap_or_default()
                        .lines()
                        .next()
                        .unwrap_or_default()
                        .trim()
                        .to_owned(),
                });
            }
            continue;
        }

        body_items.push(G3TsArchFacadeItem {
            line: child.start_position().row + 1,
            kind: child_kind,
            name: child
                .utf8_text(source.as_bytes())
                .unwrap_or_default()
                .lines()
                .next()
                .unwrap_or_default()
                .trim()
                .to_owned(),
        });
    }

    Some(G3TsArchFacadeFileState::Parsed {
        surface: G3TsArchFacadeSurface {
            rel_path: rel_path.to_owned(),
            body_items,
            broad_reexports,
        },
    })
}

fn is_ignored_root_kind(kind: &str) -> bool {
    kind == "import_statement" || kind == "empty_statement" || kind == "comment"
}

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
