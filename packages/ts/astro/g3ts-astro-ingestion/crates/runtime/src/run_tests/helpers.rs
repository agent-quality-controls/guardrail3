use tempfile::TempDir;

use g3_workspace_crawl::{
    G3RsWorkspaceCrawl as G3WorkspaceCrawl, G3RsWorkspaceEntry as G3WorkspaceEntry,
    G3RsWorkspaceEntryKind as G3WorkspaceEntryKind,
    G3RsWorkspaceIgnoreState as G3WorkspaceIgnoreState, G3RsWorkspacePath as G3WorkspacePath,
};

pub(super) fn fake_astro_workspace() -> TempDir {
    let root = TempDir::new().expect("tempdir should be created");

    std::fs::create_dir_all(root.path().join("src/pages"))
        .expect("pages directory should be created");
    std::fs::create_dir_all(root.path().join("src/lib")).expect("lib directory should be created");
    std::fs::create_dir_all(root.path().join("src/content/posts"))
        .expect("content directory should be created");
    std::fs::create_dir_all(root.path().join("node_modules/eslint"))
        .expect("fake eslint module directory should be created");

    std::fs::write(
        root.path().join("package.json"),
        "{\n  \"devDependencies\": {\n    \"astro\": \"6.1.9\",\n    \"@astrojs/react\": \"5.0.4\",\n    \"@astrojs/mdx\": \"5.0.4\",\n    \"@astrojs/check\": \"0.9.8\",\n    \"@astrojs/sitemap\": \"3.7.2\",\n    \"astro-robots\": \"2.3.1\",\n    \"@nuasite/checks\": \"0.18.0\",\n    \"g3ts-astro-nuasite-checks\": \"0.1.0\",\n    \"schema-dts\": \"2.0.0\",\n    \"react\": \"19.2.5\",\n    \"react-dom\": \"19.2.5\",\n    \"@types/react\": \"19.2.14\",\n    \"@types/react-dom\": \"19.2.3\",\n    \"typescript\": \"5.9.3\",\n    \"eslint-plugin-astro\": \"1.7.0\",\n    \"g3ts-eslint-plugin-astro-pipeline\": \"0.1.5\",\n    \"eslint-plugin-i18next\": \"6.1.4\",\n    \"eslint-plugin-mdx\": \"3.7.0\",\n    \"syncpack\": \"14.0.0\"\n  },\n  \"scripts\": {\n    \"check\": \"astro check && syncpack lint\",\n    \"build\": \"astro build\"\n  }\n}\n",
    )
    .expect("package manifest should be written");
    std::fs::write(
        root.path().join("astro.config.mjs"),
        "import react from '@astrojs/react';\nimport mdx from '@astrojs/mdx';\nimport sitemap from '@astrojs/sitemap';\nimport robots from 'astro-robots';\nimport checks from '@nuasite/checks';\nimport { structuredDataPresentCheck } from 'g3ts-astro-nuasite-checks';\nexport default { site: 'https://example.com', output: 'static', integrations: [react(), mdx(), sitemap(), robots(), checks({ mode: 'full', failOnError: true, failOnWarning: true, reportJson: true, ai: false, customChecks: [structuredDataPresentCheck] })] };\n",
    )
    .expect("astro config should be written");
    std::fs::write(
        root.path().join("src/content.config.ts"),
        "export default {};\n",
    )
    .expect("content config should be written");
    std::fs::write(
        root.path().join("eslint.config.mjs"),
        "export default [];\n",
    )
    .expect("eslint config should be written");
    std::fs::write(
        root.path().join(".syncpackrc"),
        r#"{
  "source": ["package.json"],
  "versionGroups": [
    { "dependencies": ["astro"], "dependencyTypes": ["prod", "dev"], "pinVersion": "6.1.9" },
    { "dependencies": ["@astrojs/react"], "dependencyTypes": ["prod", "dev"], "pinVersion": "5.0.4" },
    { "dependencies": ["@astrojs/mdx"], "dependencyTypes": ["prod", "dev"], "pinVersion": "5.0.4" },
    { "dependencies": ["@astrojs/check"], "dependencyTypes": ["prod", "dev"], "pinVersion": "0.9.8" },
    { "dependencies": ["@astrojs/sitemap"], "dependencyTypes": ["prod", "dev"], "pinVersion": "3.7.2" },
    { "dependencies": ["astro-robots"], "dependencyTypes": ["prod", "dev"], "pinVersion": "2.3.1" },
    { "dependencies": ["@nuasite/checks"], "dependencyTypes": ["prod", "dev"], "pinVersion": "0.18.0" },
    { "dependencies": ["g3ts-astro-nuasite-checks"], "dependencyTypes": ["prod", "dev"], "pinVersion": "0.1.0" },
    { "dependencies": ["schema-dts"], "dependencyTypes": ["prod", "dev"], "pinVersion": "2.0.0" },
    { "dependencies": ["react"], "dependencyTypes": ["prod", "dev"], "pinVersion": "19.2.5" },
    { "dependencies": ["react-dom"], "dependencyTypes": ["prod", "dev"], "pinVersion": "19.2.5" },
    { "dependencies": ["@types/react"], "dependencyTypes": ["prod", "dev"], "pinVersion": "19.2.14" },
    { "dependencies": ["@types/react-dom"], "dependencyTypes": ["prod", "dev"], "pinVersion": "19.2.3" },
    { "dependencies": ["typescript"], "dependencyTypes": ["prod", "dev"], "pinVersion": "5.9.3" },
    { "dependencies": ["eslint-plugin-astro"], "dependencyTypes": ["prod", "dev"], "pinVersion": "1.7.0" },
    { "dependencies": ["g3ts-eslint-plugin-astro-pipeline"], "dependencyTypes": ["prod", "dev"], "pinVersion": "0.1.5" },
    { "dependencies": ["eslint-plugin-i18next"], "dependencyTypes": ["prod", "dev"], "pinVersion": "6.1.4" },
    { "dependencies": ["eslint-plugin-mdx"], "dependencyTypes": ["prod", "dev"], "pinVersion": "3.7.0" },
    { "dependencies": ["next"], "dependencyTypes": ["prod", "dev", "optional", "peer"], "isBanned": true },
    { "dependencies": ["velite"], "dependencyTypes": ["prod", "dev", "optional", "peer"], "isBanned": true },
    { "dependencies": ["@astrojs/node"], "dependencyTypes": ["prod", "dev", "optional", "peer"], "isBanned": true },
    { "dependencies": ["eslint-plugin-astro-pipeline"], "dependencyTypes": ["prod", "dev", "optional", "peer"], "isBanned": true },
    { "dependencies": ["@codemint/astro-meta"], "dependencyTypes": ["prod", "dev", "optional", "peer"], "isBanned": true },
    { "dependencies": ["astro-seo-meta"], "dependencyTypes": ["prod", "dev", "optional", "peer"], "isBanned": true },
    { "dependencies": ["astro-seo-schema"], "dependencyTypes": ["prod", "dev", "optional", "peer"], "isBanned": true }
  ]
}
"#,
    )
    .expect("syncpack config should be written");
    std::fs::write(
        root.path().join("src/pages/index.ts"),
        "export const GET = () => null;\n",
    )
    .expect("route source should be written");
    std::fs::write(
        root.path().join("src/lib/kit-taxonomy.ts"),
        "export const taxonomy = '../../packages/spec/src/data/taxonomy.json';\n",
    )
    .expect("side loader source should be written");
    std::fs::write(root.path().join("src/pages/about.mdx"), "# about\n")
        .expect("route markdown page should be written");
    std::fs::write(
        root.path().join("node_modules/eslint/package.json"),
        "{\n  \"name\": \"eslint\",\n  \"version\": \"0.0.0-test\",\n  \"main\": \"index.js\"\n}\n",
    )
    .expect("fake eslint package manifest should be written");
    std::fs::write(
        root.path().join("node_modules/eslint/index.js"),
        r#"class ESLint {
  constructor(options) {
    this.cwd = options.cwd;
    this.overrideConfigFile = options.overrideConfigFile;
  }

  async isPathIgnored(_filePath) {
    return false;
  }

      async calculateConfigForFile(_filePath) {
    const isTsx = String(_filePath).endsWith('.tsx');
    return {
      plugins: {
        astro: {},
        "astro-pipeline": {},
        "i18next": {},
        mdx: {},
      },
      rules: {
        "astro-pipeline/no-authored-content-fs-read": ["error", { routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"], endpointGlobs: ["src/pages/**/*.{ts,js}"], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-authored-content-glob": ["error", { routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"], endpointGlobs: ["src/pages/**/*.{ts,js}"], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-authored-content-imports": ["error", { routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"], endpointGlobs: ["src/pages/**/*.{ts,js}"], authoredContentGlobs: ["src/content/**"] }],
        "astro-pipeline/no-content-data-modules-in-routes": ["error", { routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"], endpointGlobs: ["src/pages/**/*.{ts,js}"], contentDataModuleGlobs: ["src/**/*.data.{ts,tsx}"] }],
        "astro-pipeline/no-direct-astro-content-in-routes": ["error", { routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"], endpointGlobs: ["src/pages/**/*.{ts,js}"] }],
        "astro-pipeline/no-runtime-mdx-eval": "error",
        "astro-pipeline/require-approved-content-adapter-in-routes": ["error", { routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"], endpointGlobs: ["src/pages/**/*.{ts,js}"], approvedContentAdapterModules: ["src/content/landing-homepage.ts"] }],
        "astro-pipeline/no-side-loader-imports": ["error", { routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"], endpointGlobs: ["src/pages/**/*.{ts,js}"] }],
        "astro-pipeline/no-velite-imports": ["error", { routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"], endpointGlobs: ["src/pages/**/*.{ts,js}"] }],
        "i18next/no-literal-string": ["error", { framework: "react", mode: "all", message: "Inline public copy must live in Astro content entries. Move this text into the content collection, validate it through the collection schema, and pass the typed value into source.", "should-validate-template": true, words: { include: [], exclude: ["[0-9!-/:-@[-`{-~]+", "[A-Z_-]+"] }, "jsx-components": { include: [], exclude: [] }, "jsx-attributes": { include: [], exclude: ["as", "class", "className", "color", "data-.+", "height", "href", "id", "intent", "key", "name", "rel", "role", "size", "slot", "src", "style", "styleName", "target", "tone", "type", "variant", "width", "aria-hidden"] }, callees: { include: [], exclude: ["require", "clsx", "cn", "cx", "cva", "twMerge", "twJoin", "tv", "URL"] }, "object-properties": { include: [], exclude: ["[A-Z_-]+"] }, "class-properties": { include: [], exclude: ["displayName"] } }],
        "mdx/remark": "error",
      },
      languageOptions: { parserOptions: { projectService: true, jsx: isTsx } },
    };
  }
}

module.exports = { ESLint };
"#,
    )
    .expect("fake eslint runtime should be written");

    root
}

pub(super) fn crawl_with_entries(root: &TempDir, rel_paths: &[&str]) -> G3WorkspaceCrawl {
    G3WorkspaceCrawl {
        root_abs_path: root.path().to_path_buf(),
        entries: rel_paths
            .iter()
            .map(|rel_path| entry(root, rel_path, true))
            .collect(),
    }
}

pub(super) fn unreadable_entry(root: &TempDir, rel_path: &str) -> G3WorkspaceEntry {
    entry(root, rel_path, false)
}

pub(super) fn ignored_entry(root: &TempDir, rel_path: &str) -> G3WorkspaceEntry {
    G3WorkspaceEntry {
        path: G3WorkspacePath {
            rel_path: rel_path.to_owned(),
            abs_path: root.path().join(rel_path),
        },
        kind: G3WorkspaceEntryKind::File,
        ignore_state: G3WorkspaceIgnoreState::Ignored,
        readable: true,
    }
}

fn entry(root: &TempDir, rel_path: &str, readable: bool) -> G3WorkspaceEntry {
    G3WorkspaceEntry {
        path: G3WorkspacePath {
            rel_path: rel_path.to_owned(),
            abs_path: root.path().join(rel_path),
        },
        kind: G3WorkspaceEntryKind::File,
        ignore_state: G3WorkspaceIgnoreState::Included,
        readable,
    }
}
