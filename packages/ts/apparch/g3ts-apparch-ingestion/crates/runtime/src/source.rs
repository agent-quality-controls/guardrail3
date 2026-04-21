use std::collections::BTreeSet;
use std::path::{Component, Path};

use g3_workspace_crawl as workspace_crawl;
use g3ts_apparch_types as apparch_types;
use tree_sitter::Parser;

use crate::run::G3TsApparchIngestionError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AppFacts {
    pub files: Vec<apparch_types::G3TsApparchSourceFile>,
    pub internal_edges: Vec<apparch_types::G3TsApparchInternalEdge>,
    pub external_imports: Vec<apparch_types::G3TsApparchExternalImport>,
    pub public_items: Vec<apparch_types::G3TsApparchPublicItem>,
}

pub(crate) fn collect_app_facts(
    crawl: &workspace_crawl::G3WorkspaceCrawl,
) -> Result<AppFacts, G3TsApparchIngestionError> {
    let files = source_files(crawl);
    let mut internal_edges = BTreeSet::new();
    let mut external_imports = BTreeSet::new();
    let mut public_items = Vec::new();

    for file in &files {
        let source = read_source_file(crawl, &file.rel_path)?;
        let parsed = parse_source_file(&file.rel_path, &source)?;

        for import in parsed.imports {
            match resolve_import(crawl, &file.rel_path, &import.specifier) {
                Some(ResolvedImport::Internal {
                    rel_path,
                    layer: to_layer,
                }) => {
                    let _ = internal_edges.insert(apparch_types::G3TsApparchInternalEdge {
                        from_rel_path: file.rel_path.clone(),
                        from_layer: file.layer,
                        to_rel_path: rel_path,
                        to_layer,
                        kind: import.kind,
                    });
                }
                Some(ResolvedImport::External(module_name)) => {
                    let _ = external_imports.insert(apparch_types::G3TsApparchExternalImport {
                        from_rel_path: file.rel_path.clone(),
                        from_layer: file.layer,
                        module_name,
                        kind: import.kind,
                    });
                }
                None => {}
            }
        }

        public_items.extend(parsed.public_items.into_iter().map(|item| {
            apparch_types::G3TsApparchPublicItem {
                rel_path: file.rel_path.clone(),
                layer: file.layer,
                item_name: item.item_name,
                kind: item.kind,
                line: item.line,
            }
        }));
    }

    Ok(AppFacts {
        files,
        internal_edges: internal_edges.into_iter().collect(),
        external_imports: external_imports.into_iter().collect(),
        public_items,
    })
}

fn source_files(
    crawl: &workspace_crawl::G3WorkspaceCrawl,
) -> Vec<apparch_types::G3TsApparchSourceFile> {
    let mut files = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == workspace_crawl::G3WorkspaceEntryKind::File)
        .filter(|entry| entry.ignore_state == workspace_crawl::G3WorkspaceIgnoreState::Included)
        .filter(|entry| entry.readable)
        .filter_map(|entry| {
            classify_layer(&entry.path.rel_path).map(|layer| apparch_types::G3TsApparchSourceFile {
                rel_path: entry.path.rel_path.clone(),
                layer,
            })
        })
        .collect::<Vec<_>>();
    files.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    files
}

fn classify_layer(rel_path: &str) -> Option<apparch_types::G3TsApparchLayer> {
    if is_source_extension(rel_path) {
        if rel_path.starts_with("src/app/") {
            return Some(apparch_types::G3TsApparchLayer::App);
        }
        if rel_path.starts_with("src/types/") {
            return Some(apparch_types::G3TsApparchLayer::Types);
        }
        if rel_path.starts_with("src/logic/") {
            return Some(apparch_types::G3TsApparchLayer::Logic);
        }
        if rel_path.starts_with("src/io/inbound/") {
            return Some(apparch_types::G3TsApparchLayer::IoInbound);
        }
        if rel_path.starts_with("src/io/outbound/") {
            return Some(apparch_types::G3TsApparchLayer::IoOutbound);
        }
    }

    None
}

fn is_source_extension(rel_path: &str) -> bool {
    (rel_path.ends_with(".ts") || rel_path.ends_with(".tsx")) && !rel_path.ends_with(".d.ts")
}

fn read_source_file(
    crawl: &workspace_crawl::G3WorkspaceCrawl,
    rel_path: &str,
) -> Result<String, G3TsApparchIngestionError> {
    let Some(workspace_entry) = workspace_crawl::entry(crawl, rel_path) else {
        return Err(G3TsApparchIngestionError {
            message: format!("source path `{rel_path}` missing from crawl"),
        });
    };

    crate::fs::read_to_string(&workspace_entry.path.abs_path).map_err(|error| {
        G3TsApparchIngestionError {
            message: format!("could not read `{rel_path}`: {error}"),
        }
    })
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ParsedImport {
    kind: apparch_types::G3TsApparchImportKind,
    specifier: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedPublicItem {
    item_name: String,
    kind: apparch_types::G3TsApparchPublicItemKind,
    line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedSourceFile {
    imports: Vec<ParsedImport>,
    public_items: Vec<ParsedPublicItem>,
}

fn parse_source_file(
    rel_path: &str,
    source: &str,
) -> Result<ParsedSourceFile, G3TsApparchIngestionError> {
    let mut parser = Parser::new();
    let language = if rel_path.ends_with(".tsx") {
        tree_sitter_typescript::LANGUAGE_TSX
    } else {
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT
    };
    parser
        .set_language(&language.into())
        .map_err(|error| G3TsApparchIngestionError {
            message: format!("could not initialize parser for `{rel_path}`: {error}"),
        })?;

    let Some(tree) = parser.parse(source, None) else {
        return Err(G3TsApparchIngestionError {
            message: format!("tree-sitter returned no parse tree for `{rel_path}`"),
        });
    };

    if tree.root_node().has_error() {
        return Err(G3TsApparchIngestionError {
            message: format!("TypeScript parser reported syntax errors in `{rel_path}`"),
        });
    }

    let mut imports = BTreeSet::new();
    let mut public_items = Vec::new();
    let root = tree.root_node();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        let kind = child.kind();
        if kind == "import_statement" {
            if let Some(specifier) = module_specifier_text(child, source) {
                let _ = imports.insert(ParsedImport {
                    kind: apparch_types::G3TsApparchImportKind::Import,
                    specifier,
                });
            }
            continue;
        }
        if kind == "export_statement" {
            if let Some(specifier) = module_specifier_text(child, source) {
                let _ = imports.insert(ParsedImport {
                    kind: apparch_types::G3TsApparchImportKind::Reexport,
                    specifier,
                });
            }
            collect_exported_items(child, source, &mut public_items);
        }
        collect_dynamic_imports(child, source, &mut imports);
    }

    Ok(ParsedSourceFile {
        imports: imports.into_iter().collect(),
        public_items,
    })
}

fn module_specifier_text(node: tree_sitter::Node<'_>, source: &str) -> Option<String> {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .find(|child| child.kind() == "string")
        .and_then(|child| child.utf8_text(source.as_bytes()).ok())
        .and_then(trim_string_literal)
        .map(str::to_owned)
}

fn trim_string_literal(raw: &str) -> Option<&str> {
    raw.strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .or_else(|| {
            raw.strip_prefix('\'')
                .and_then(|value| value.strip_suffix('\''))
        })
}

fn collect_exported_items(
    node: tree_sitter::Node<'_>,
    source: &str,
    public_items: &mut Vec<ParsedPublicItem>,
) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = match child.kind() {
            "interface_declaration" => Some(apparch_types::G3TsApparchPublicItemKind::Interface),
            "function_declaration" => Some(apparch_types::G3TsApparchPublicItemKind::Function),
            "class_declaration" => Some(apparch_types::G3TsApparchPublicItemKind::Class),
            _ => None,
        };
        let Some(kind) = kind else {
            if child.kind() == "lexical_declaration" {
                collect_exported_lexical_items(child, source, public_items);
            }
            continue;
        };
        public_items.push(ParsedPublicItem {
            item_name: exported_item_name(child, source),
            kind,
            line: child.start_position().row + 1,
        });
    }
}

fn collect_exported_lexical_items(
    node: tree_sitter::Node<'_>,
    source: &str,
    public_items: &mut Vec<ParsedPublicItem>,
) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() != "variable_declarator" {
            continue;
        }

        let Some(kind) = declarator_public_item_kind(child) else {
            continue;
        };
        public_items.push(ParsedPublicItem {
            item_name: exported_item_name(child, source),
            kind,
            line: child.start_position().row + 1,
        });
    }
}

fn declarator_public_item_kind(
    node: tree_sitter::Node<'_>,
) -> Option<apparch_types::G3TsApparchPublicItemKind> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "arrow_function" | "function" | "function_expression" => {
                return Some(apparch_types::G3TsApparchPublicItemKind::Function);
            }
            "class" | "class_declaration" => {
                return Some(apparch_types::G3TsApparchPublicItemKind::Class);
            }
            _ => {}
        }
    }
    None
}

fn exported_item_name(node: tree_sitter::Node<'_>, source: &str) -> String {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .find(|child| child.kind() == "identifier" || child.kind() == "type_identifier")
        .and_then(|child| child.utf8_text(source.as_bytes()).ok())
        .map(str::to_owned)
        .unwrap_or_else(|| node.kind().to_owned())
}

fn collect_dynamic_imports(
    node: tree_sitter::Node<'_>,
    source: &str,
    imports: &mut BTreeSet<ParsedImport>,
) {
    if is_dynamic_import_call(node) {
        if let Some(specifier) = first_string_descendant_text(node, source) {
            let _ = imports.insert(ParsedImport {
                kind: apparch_types::G3TsApparchImportKind::DynamicImport,
                specifier,
            });
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_dynamic_imports(child, source, imports);
    }
}

fn is_dynamic_import_call(node: tree_sitter::Node<'_>) -> bool {
    if node.kind() != "call_expression" {
        return false;
    }

    let mut cursor = node.walk();
    node.children(&mut cursor)
        .any(|child| child.kind() == "import")
}

fn first_string_descendant_text(node: tree_sitter::Node<'_>, source: &str) -> Option<String> {
    if node.kind() == "string" {
        return node
            .utf8_text(source.as_bytes())
            .ok()
            .and_then(trim_string_literal)
            .map(str::to_owned);
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if let Some(specifier) = first_string_descendant_text(child, source) {
            return Some(specifier);
        }
    }

    None
}

enum ResolvedImport {
    Internal {
        rel_path: String,
        layer: apparch_types::G3TsApparchLayer,
    },
    External(String),
}

fn resolve_import(
    crawl: &workspace_crawl::G3WorkspaceCrawl,
    from_rel_path: &str,
    specifier: &str,
) -> Option<ResolvedImport> {
    if let Some(stripped) = specifier.strip_prefix("@/") {
        return resolve_internal_target(crawl, &format!("src/{stripped}"));
    }
    if specifier.starts_with("./") || specifier.starts_with("../") {
        let base_dir = Path::new(from_rel_path).parent()?;
        let joined = base_dir.join(specifier);
        let normalized = normalize_relative_path(&joined)?;
        return resolve_internal_target(crawl, &normalized);
    }
    Some(ResolvedImport::External(specifier.to_owned()))
}

fn resolve_internal_target(
    crawl: &workspace_crawl::G3WorkspaceCrawl,
    base_rel_path: &str,
) -> Option<ResolvedImport> {
    for candidate in candidate_source_paths(base_rel_path) {
        let Some(workspace_entry) = workspace_crawl::entry(crawl, &candidate) else {
            continue;
        };
        if !workspace_entry.readable {
            continue;
        }
        let Some(layer) = classify_layer(&candidate) else {
            continue;
        };
        return Some(ResolvedImport::Internal {
            rel_path: candidate,
            layer,
        });
    }
    None
}

fn candidate_source_paths(base_rel_path: &str) -> Vec<String> {
    let mut candidates = Vec::new();
    if is_source_extension(base_rel_path) {
        candidates.push(base_rel_path.to_owned());
    }
    candidates.push(format!("{base_rel_path}.ts"));
    candidates.push(format!("{base_rel_path}.tsx"));
    candidates.push(format!("{base_rel_path}/index.ts"));
    candidates.push(format!("{base_rel_path}/index.tsx"));
    candidates
}

fn normalize_relative_path(path: &Path) -> Option<String> {
    let mut segments = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => segments.push(part.to_str()?.to_owned()),
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = segments.pop()?;
            }
            Component::Prefix(_) | Component::RootDir => return None,
        }
    }
    (!segments.is_empty()).then(|| segments.join("/"))
}
