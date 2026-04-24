use std::fs;

use g3_workspace_crawl::crawl;

#[test]
fn ingest_collects_internal_edge_for_alias_import() {
    let tempdir =
        tempfile::tempdir().expect("create temporary workspace for apparch alias fixture");
    fs::create_dir_all(tempdir.path().join("src/types"))
        .expect("create apparch types fixture directory");
    fs::create_dir_all(tempdir.path().join("src/logic"))
        .expect("create apparch logic fixture directory");
    fs::write(
        tempdir.path().join("src/types/user.ts"),
        "export interface User {}\n",
    )
    .expect("write types fixture file");
    fs::write(
        tempdir.path().join("src/logic/get_user.ts"),
        "import type { User } from \"@/types/user\";\nexport async function getUser(): Promise<User | null> { return null; }\n",
    )
    .expect("write logic fixture file");

    let crawl = crawl(tempdir.path()).expect("crawl apparch alias fixture");
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingest apparch config facts for alias fixture");

    g3ts_apparch_ingestion_assertions::run::assert_has_internal_edge(
        &input,
        "src/logic/get_user.ts",
        "src/types/user.ts",
    );
}

#[test]
fn ingest_collects_internal_edge_for_dynamic_import() {
    let tempdir =
        tempfile::tempdir().expect("create temporary workspace for apparch dynamic-import fixture");
    fs::create_dir_all(tempdir.path().join("src/types"))
        .expect("create apparch dynamic-import types fixture directory");
    fs::create_dir_all(tempdir.path().join("src/app"))
        .expect("create apparch dynamic-import app fixture directory");
    fs::write(
        tempdir.path().join("src/types/user.ts"),
        "export interface User {}\n",
    )
    .expect("write dynamic-import types fixture file");
    fs::write(
        tempdir.path().join("src/app/page.tsx"),
        "export default async function Page() { const module = await import(\"@/types/user\"); return module.User ? null : null; }\n",
    )
    .expect("write dynamic-import app fixture file");

    let crawl = crawl(tempdir.path()).expect("crawl apparch dynamic-import fixture");
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingest apparch config facts for dynamic-import fixture");

    assert!(
        input.internal_edges.iter().any(|edge| {
            edge.from_rel_path == "src/app/page.tsx"
                && edge.to_rel_path == "src/types/user.ts"
                && edge.kind == g3ts_apparch_types::G3TsApparchImportKind::DynamicImport
        }),
        "expected dynamic internal edge to types fixture, got {:?}",
        input.internal_edges
    );
}

#[test]
fn ingest_accepts_valid_tsx_with_ampersand_in_jsx_text() {
    let tempdir =
        tempfile::tempdir().expect("create temporary workspace for apparch tsx-text fixture");
    fs::create_dir_all(tempdir.path().join("src/app"))
        .expect("create apparch tsx-text app fixture directory");
    fs::write(
        tempdir.path().join("src/app/hub-hero.tsx"),
        "import type { ReactElement } from \"react\";\nexport function HubHero(): ReactElement {\n  return (\n    <section>\n      <h1>\n        Technical SEO &\n        <br />\n        <span>AI-search</span> playbook.\n      </h1>\n    </section>\n  );\n}\n",
    )
    .expect("write tsx-text fixture file");

    let crawl = crawl(tempdir.path()).expect("crawl apparch tsx-text fixture");
    let input = crate::run::ingest_for_config_checks(&crawl);

    assert!(
        input.is_ok(),
        "expected valid TSX with JSX text ampersand to ingest cleanly, got {:?}",
        input
    );
}

#[test]
fn ingest_accepts_valid_tsx_with_ampersand_phrase_in_jsx_text() {
    let tempdir =
        tempfile::tempdir().expect("create temporary workspace for apparch tsx-phrase fixture");
    fs::create_dir_all(tempdir.path().join("src/app"))
        .expect("create apparch tsx-phrase app fixture directory");
    fs::write(
        tempdir.path().join("src/app/styleguide.tsx"),
        "export function Styleguide() {\n  return (\n    <p>AI Search & GEO dashboards</p>\n  );\n}\n",
    )
    .expect("write tsx-phrase fixture file");

    let crawl = crawl(tempdir.path()).expect("crawl apparch tsx-phrase fixture");
    let input = crate::run::ingest_for_config_checks(&crawl);

    assert!(
        input.is_ok(),
        "expected valid TSX with JSX text ampersand phrase to ingest cleanly, got {:?}",
        input
    );
}

#[test]
fn ingest_still_rejects_real_tsx_syntax_errors() {
    let tempdir =
        tempfile::tempdir().expect("create temporary workspace for apparch invalid-tsx fixture");
    fs::create_dir_all(tempdir.path().join("src/app"))
        .expect("create apparch invalid-tsx app fixture directory");
    fs::write(
        tempdir.path().join("src/app/broken.tsx"),
        "export function Broken() {\n  return <section>{foo(}</section>;\n}\n",
    )
    .expect("write invalid-tsx fixture file");

    let crawl = crawl(tempdir.path()).expect("crawl apparch invalid-tsx fixture");
    let input = crate::run::ingest_for_config_checks(&crawl);

    assert!(input.is_err(), "expected invalid TSX to stay rejected");
}

#[test]
fn ingest_collects_external_imports() {
    let tempdir =
        tempfile::tempdir().expect("create temporary workspace for apparch external fixture");
    fs::create_dir_all(tempdir.path().join("src/io/outbound"))
        .expect("create outbound fixture directory");
    fs::write(
        tempdir.path().join("src/io/outbound/db.ts"),
        "import { drizzle } from \"drizzle-orm/node-postgres\";\nexport class DbAdapter {}\n",
    )
    .expect("write outbound fixture file");

    let crawl = crawl(tempdir.path()).expect("crawl apparch external fixture");
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingest apparch config facts for external fixture");

    assert!(
        input.external_imports.iter().any(|import| {
            import.from_rel_path == "src/io/outbound/db.ts"
                && import.module_name == "drizzle-orm/node-postgres"
        }),
        "expected external import from drizzle fixture, got {:?}",
        input.external_imports
    );
}

#[test]
fn ingest_collects_exported_function_binding_for_source_checks() {
    let tempdir = tempfile::tempdir()
        .expect("create temporary workspace for apparch exported-binding fixture");
    fs::create_dir_all(tempdir.path().join("src/types"))
        .expect("create exported-binding types fixture directory");
    fs::write(
        tempdir.path().join("src/types/index.ts"),
        "export const helper = (): number => 1;\n",
    )
    .expect("write exported-binding types fixture file");

    let crawl = crawl(tempdir.path()).expect("crawl apparch exported-binding fixture");
    let input = crate::run::ingest_for_source_checks(&crawl)
        .expect("ingest apparch source facts for exported-binding fixture");

    assert!(
        input.public_items.iter().any(|item| {
            item.rel_path == "src/types/index.ts"
                && item.item_name == "helper"
                && item.kind == g3ts_apparch_types::G3TsApparchPublicItemKind::Function
        }),
        "expected exported function binding in source facts, got {:?}",
        input.public_items
    );
}

#[test]
fn ingest_collects_exported_items_for_source_checks() {
    let tempdir =
        tempfile::tempdir().expect("create temporary workspace for apparch public-surface fixture");
    fs::create_dir_all(tempdir.path().join("src/types"))
        .expect("create public-surface types fixture directory");
    fs::create_dir_all(tempdir.path().join("src/io/outbound"))
        .expect("create public-surface outbound fixture directory");
    fs::write(
        tempdir.path().join("src/types/index.ts"),
        "export interface User {}\nexport function helper(): number { return 1; }\n",
    )
    .expect("write types surface fixture file");
    fs::write(
        tempdir.path().join("src/io/outbound/db.ts"),
        "export interface DbPort {}\n",
    )
    .expect("write outbound surface fixture file");

    let crawl = crawl(tempdir.path()).expect("crawl apparch public-surface fixture");
    let input = crate::run::ingest_for_source_checks(&crawl)
        .expect("ingest apparch source facts for public-surface fixture");

    assert!(
        input.public_items.iter().any(|item| {
            item.rel_path == "src/types/index.ts"
                && item.item_name == "helper"
                && item.kind == g3ts_apparch_types::G3TsApparchPublicItemKind::Function
        }),
        "expected exported helper function in source facts, got {:?}",
        input.public_items
    );
    assert!(
        input.public_items.iter().any(|item| {
            item.rel_path == "src/io/outbound/db.ts"
                && item.item_name == "DbPort"
                && item.kind == g3ts_apparch_types::G3TsApparchPublicItemKind::Interface
        }),
        "expected exported interface in source facts, got {:?}",
        input.public_items
    );
}
