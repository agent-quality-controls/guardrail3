use std::collections::BTreeSet;
use std::path::{Component, Path};

use g3_workspace_crawl as workspace_crawl;
use g3ts_apparch_types as apparch_types;
use tree_sitter::{Node, Parser};

use crate::run::G3TsApparchIngestionError;

/// Aggregated facts about the app's source tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AppFacts {
    /// All discovered TS/TSX source files with layer assignments.
    pub files: Vec<apparch_types::G3TsApparchSourceFile>,
    /// Internal-to-internal import edges between layers.
    pub internal_edges: Vec<apparch_types::G3TsApparchInternalEdge>,
    /// External (npm) imports keyed by module name.
    pub external_imports: Vec<apparch_types::G3TsApparchExternalImport>,
    /// Public top-level exported items keyed by file/line.
    pub public_items: Vec<apparch_types::G3TsApparchPublicItem>,
}

/// Walk the workspace crawl and produce `AppFacts`.
pub(crate) fn collect_app_facts(
    crawl: &workspace_crawl::G3RsWorkspaceCrawl,
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

/// Collect the readable, classified source files from `crawl`, sorted by rel-path.
fn source_files(
    crawl: &workspace_crawl::G3RsWorkspaceCrawl,
) -> Vec<apparch_types::G3TsApparchSourceFile> {
    let mut files = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == workspace_crawl::G3RsWorkspaceEntryKind::File)
        .filter(|entry| entry.ignore_state == workspace_crawl::G3RsWorkspaceIgnoreState::Included)
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

/// Map a rel-path under `src/...` to a layer, or `None` if it does not belong to any known layer.
fn classify_layer(rel_path: &str) -> Option<apparch_types::G3TsApparchLayer> {
    if !is_source_extension(rel_path) {
        return None;
    }
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
    None
}

/// Whether `rel_path` has a TypeScript source extension (`.ts`/`.tsx`, excluding `.d.ts`).
fn is_source_extension(rel_path: &str) -> bool {
    let path = Path::new(rel_path);
    let ext_matches = path
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("ts") || ext.eq_ignore_ascii_case("tsx"));
    if !ext_matches {
        return false;
    }
    !rel_path.rsplit_once('.').is_some_and(|(stem, _)| {
        Path::new(stem)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("d"))
    })
}

/// One-based line number for `node`'s starting row.
fn line_one_based(node: Node<'_>) -> usize {
    node.start_position().row.saturating_add(1)
}

/// Read the source bytes of `rel_path` from the crawl, returning a structured ingestion error on failure.
fn read_source_file(
    crawl: &workspace_crawl::G3RsWorkspaceCrawl,
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

/// One module-specifier import discovered in a source file.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ParsedImport {
    /// Kind of import (static, reexport, dynamic).
    kind: apparch_types::G3TsApparchImportKind,
    /// Raw module specifier text (without quotes).
    specifier: String,
}

/// One top-level public item discovered by the parser.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedPublicItem {
    /// Exported name.
    item_name: String,
    /// Kind of public item (function/class/interface/etc.).
    kind: apparch_types::G3TsApparchPublicItemKind,
    /// One-based line number of the declaration.
    line: usize,
}

/// Parsed surface of one source file.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedSourceFile {
    /// Imports discovered in the file.
    imports: Vec<ParsedImport>,
    /// Public top-level items discovered in the file.
    public_items: Vec<ParsedPublicItem>,
}

/// Parse a source file's text into imports and public items.
fn parse_source_file(
    rel_path: &str,
    source: &str,
) -> Result<ParsedSourceFile, G3TsApparchIngestionError> {
    let mut parser = Parser::new();
    let language = if has_tsx_extension(rel_path) {
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

    let root = tree.root_node();
    if let Some(reason) = first_intolerable_parse_error(root, source, has_tsx_extension(rel_path)) {
        return Err(G3TsApparchIngestionError {
            message: format!("TypeScript parser reported syntax errors in `{rel_path}`: {reason}"),
        });
    }

    let mut imports = BTreeSet::new();
    let mut public_items = Vec::new();
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

/// Whether `rel_path` ends in `.tsx` (case-insensitive).
fn has_tsx_extension(rel_path: &str) -> bool {
    Path::new(rel_path)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("tsx"))
}

/// Find the first parser error worth reporting, recursing into children.
fn first_intolerable_parse_error(node: Node<'_>, source: &str, is_tsx: bool) -> Option<String> {
    if node.is_missing() {
        return Some(format!(
            "missing `{}` token at line {}",
            node.kind(),
            line_one_based(node)
        ));
    }

    if node.is_error() && !is_tolerable_jsx_text_ampersand_error(node, source, is_tsx) {
        return Some(format!(
            "unexpected token {:?} at line {}",
            node.utf8_text(source.as_bytes())
                .unwrap_or("<invalid utf8>"),
            line_one_based(node)
        ));
    }

    if !node.has_error() {
        return None;
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if let Some(reason) = first_intolerable_parse_error(child, source, is_tsx) {
            return Some(reason);
        }
    }

    None
}

/// Whether `node` is a tolerable bare-`&` JSX text error (tree-sitter quirk).
fn is_tolerable_jsx_text_ampersand_error(node: Node<'_>, source: &str, is_tsx: bool) -> bool {
    if !is_tsx || !node.is_error() || node.kind() != "ERROR" {
        return false;
    }

    let Ok(text) = node.utf8_text(source.as_bytes()) else {
        return false;
    };

    if !text.contains('&')
        || text
            .chars()
            .any(|ch| matches!(ch, '<' | '>' | '{' | '}' | '(' | ')'))
    {
        return false;
    }

    let mut current = node.parent();
    while let Some(parent) = current {
        if matches!(parent.kind(), "jsx_element" | "jsx_fragment" | "jsx_text") {
            return true;
        }
        current = parent.parent();
    }

    false
}

/// Extract the module-specifier string under an import/export statement.
fn module_specifier_text(node: Node<'_>, source: &str) -> Option<String> {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .find(|child| child.kind() == "string")
        .and_then(|child| child.utf8_text(source.as_bytes()).ok())
        .and_then(trim_string_literal)
        .map(str::to_owned)
}

/// Strip surrounding `"`/`'` quotes from a string literal token.
fn trim_string_literal(raw: &str) -> Option<&str> {
    raw.strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .or_else(|| {
            raw.strip_prefix('\'')
                .and_then(|value| value.strip_suffix('\''))
        })
}

/// Walk an `export_statement` and append top-level exported items to `public_items`.
fn collect_exported_items(node: Node<'_>, source: &str, public_items: &mut Vec<ParsedPublicItem>) {
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
            line: line_one_based(child),
        });
    }
}

/// Walk an exported `lexical_declaration` and append per-declarator public items.
fn collect_exported_lexical_items(
    node: Node<'_>,
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
            line: line_one_based(child),
        });
    }
}

/// Map a `variable_declarator` initializer kind to a public-item kind.
fn declarator_public_item_kind(node: Node<'_>) -> Option<apparch_types::G3TsApparchPublicItemKind> {
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

/// Extract the exported item's identifier name, falling back to the node kind.
fn exported_item_name(node: Node<'_>, source: &str) -> String {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .find(|child| child.kind() == "identifier" || child.kind() == "type_identifier")
        .and_then(|child| child.utf8_text(source.as_bytes()).ok())
        .map_or_else(|| node.kind().to_owned(), str::to_owned)
}

/// Recursively look for `import("...")` calls and add them to `imports`.
fn collect_dynamic_imports(node: Node<'_>, source: &str, imports: &mut BTreeSet<ParsedImport>) {
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

/// Whether `node` is a `call_expression` of `import(...)`.
fn is_dynamic_import_call(node: Node<'_>) -> bool {
    if node.kind() != "call_expression" {
        return false;
    }

    let mut cursor = node.walk();
    node.children(&mut cursor)
        .any(|child| child.kind() == "import")
}

/// First descendant string-literal text under `node`, if any.
fn first_string_descendant_text(node: Node<'_>, source: &str) -> Option<String> {
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

/// Resolution outcome for one import specifier.
enum ResolvedImport {
    /// Resolves to a known internal source file with a layer assignment.
    Internal {
        /// Internal target rel-path.
        rel_path: String,
        /// Target layer.
        layer: apparch_types::G3TsApparchLayer,
    },
    /// External (npm) module specifier.
    External(String),
}

/// Resolve `specifier` from `from_rel_path` to either an internal or external import.
fn resolve_import(
    crawl: &workspace_crawl::G3RsWorkspaceCrawl,
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

/// Resolve `base_rel_path` to an existing readable internal source file.
fn resolve_internal_target(
    crawl: &workspace_crawl::G3RsWorkspaceCrawl,
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

/// Build candidate rel-paths to try when resolving an internal import target.
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

/// Normalize a relative path with `.`/`..` components into a `/`-joined string.
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
