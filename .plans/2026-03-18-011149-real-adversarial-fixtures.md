# Real adversarial fixture projects — complex, messy, realistic

**Date:** 2026-03-18 01:11
**Task:** Build fixture projects that represent the worst real-world monorepo structures

## Philosophy

Each fixture is a complete project directory tree with dozens of files. Not "create one Cargo.toml and check one thing" — create the whole fucked-up monorepo and then run EVERY command against it (init, generate, diff, validate) and check EVERYTHING.

## Fixture 1: nightmare-monorepo

A polyglot monorepo with every edge case stacked on top of each other.

```
nightmare-monorepo/
├── Cargo.toml                    # Virtual workspace: members=["packages/*"], exclude=["apps/*"]
├── package.json                  # pnpm workspace: "apps/*", "packages/*"
├── pnpm-workspace.yaml           # workspace config
├── guardrail3.toml               # Pre-existing config (partially wrong)
│
├── apps/
│   ├── api/                      # Nested workspace (own [workspace] with crates/*)
│   │   ├── Cargo.toml            # [workspace] members=["crates/*"]
│   │   ├── crates/
│   │   │   ├── domain/
│   │   │   │   └── Cargo.toml    # [package] name="api-domain"
│   │   │   ├── app/
│   │   │   │   └── Cargo.toml    # [package] name="api-app"
│   │   │   └── adapters/
│   │   │       └── Cargo.toml    # [package] name="api-adapters"
│   │   ├── clippy.toml           # PRE-EXISTING with 15 custom method bans
│   │   ├── deny.toml             # PRE-EXISTING with custom anyhow wrapper + extra crate ban
│   │   └── rustfmt.toml          # PRE-EXISTING (matches generated — no changes needed)
│   │
│   ├── my-api/                   # Single crate (NO workspace) — suffix of "api"
│   │   └── Cargo.toml            # [package] name="my-api"
│   │
│   ├── worker/                   # Single crate, no pre-existing configs
│   │   └── Cargo.toml            # [package] name="worker"
│   │
│   ├── landing/                  # TS content app
│   │   ├── package.json          # velite in devDependencies
│   │   ├── content/              # content/ directory (strong signal)
│   │   │   └── blog/
│   │   │       └── hello.mdx
│   │   └── src/
│   │       └── app/
│   │           └── page.tsx
│   │
│   ├── admin/                    # TS service app
│   │   ├── package.json          # express in dependencies
│   │   └── src/
│   │       └── modules/
│   │           └── domain/       # hex arch signal
│   │               └── index.ts
│   │
│   └── legacy/                   # EXCLUDED from root workspace, but in guardrail3.toml
│       └── Cargo.toml            # [package] name="legacy"
│
├── packages/
│   ├── shared-types/
│   │   └── Cargo.toml            # [package] name="shared-types", publish=false
│   ├── utils/
│   │   └── Cargo.toml            # [package] name="utils"
│   └── ts-ui/
│       └── package.json          # TS library package
│
├── .guardrail3/
│   └── overrides/
│       ├── clippy-methods.toml   # Valid: 3 extra method bans
│       ├── clippy-types.toml     # Empty file (0 bytes)
│       ├── deny-bans.toml        # Has valid entry + injected [[bans.features]] header
│       ├── deny-skip.toml        # Valid skip entries
│       └── eslint-rules.toml     # UNRECOGNIZED filename (should warn)
│
├── eslint.config.mjs             # PRE-EXISTING with 40+ custom rules, test relaxations
├── .stylelintrc.mjs              # PRE-EXISTING with custom CSS notation rules
├── cspell.json                   # PRE-EXISTING with 20 custom words
├── .npmrc                        # PRE-EXISTING (matches generated)
├── tsconfig.base.json            # PRE-EXISTING (outdated — missing strict flags)
├── .jscpd.json                   # PRE-EXISTING with threshold=10 (should be 0)
└── .githooks/
    └── pre-commit                # PRE-EXISTING (outdated)
```

guardrail3.toml (intentionally has issues):
```toml
version = "0.1"

[profile]
name = "service"

[rust]
workspace_root = "."

[rust.apps.api]
type = "service"

[rust.apps.api.checks]
architecture = true
garde = true
tests = true
release = true

[rust.apps.my-api]
type = "service"

[rust.apps.legacy]
type = "service"

[rust.apps.worker]
type = "service"

[rust.packages]
type = "library"

[rust.packages.checks]
architecture = false
garde = false
tests = true
release = false

[typescript]

[typescript.apps.landing]
type = "content"

[typescript.apps.landing.checks]
architecture = false
content = true
tests = true

[typescript.apps.admin]
type = "service"

[typescript.apps.admin.checks]
architecture = true
content = false
tests = true
```

### Tests against this fixture

**RS generate --dry-run assertions:**
1. `apps/api/clippy.toml` — would update (custom entries detected: 15 method bans)
2. `apps/api/deny.toml` — would update (custom entry: anyhow wrapper)
3. `apps/api/rustfmt.toml` — no changes needed
4. `apps/my-api/clippy.toml` — would create (NOT apps/api/ — suffix bug test)
5. `apps/worker/clippy.toml` — would create (NOT worker/ at root)
6. `legacy/clippy.toml` — would create (excluded from workspace, fallback path)
7. Root `clippy.toml` — would create (for packages, library profile)
8. Root `deny.toml` — would create (for packages)
9. `rust-toolchain.toml` — would create
10. `release-plz.toml` — would create
11. `cliff.toml` — would create
12. Override warning for deny-bans.toml `[[bans.features]]` injection
13. No warning for clippy-types.toml (empty = no overrides, not an error)

**TS generate --dry-run assertions:**
14. `eslint.config.mjs` — would update (massive diff, custom rules detected? no — not entry-based)
15. `.stylelintrc.mjs` — would update (has content app)
16. `cspell.json` — would update (diff from pre-existing)
17. `.npmrc` — no changes needed
18. `tsconfig.base.json` — would update (missing strict flags)
19. `.jscpd.json` — would update (threshold 10→0)
20. `.githooks/pre-commit` — would update

**TS init --dry-run assertions:**
21. landing detected as "content" (content/ dir + velite in devDeps)
22. admin detected as "service" (hex arch structure)

**Generated eslint.config.mjs content assertions (after actual generate):**
23. Contains `jsx-a11y` (content app exists)
24. Contains `boundaries` (service app exists)
25. Contains `unicorn` plugin
26. Contains `sonarjs` plugin
27. Contains all `**/` prefixed ignore patterns
28. Contains `max-lines` with 400
29. Contains test relaxation block
30. Contains `naming-convention` with `selector`

## Fixture 2: broken-configs

A project with intentionally corrupted/partial/weird config files.

```
broken-configs/
├── Cargo.toml                    # [workspace] members=["crates/*"]
├── crates/
│   └── app/
│       └── Cargo.toml
├── guardrail3.toml               # type = "frontend" (unknown type)
├── clippy.toml                   # CRLF line endings throughout
├── deny.toml                     # UTF-8 BOM + valid content
├── rustfmt.toml                  # Empty file (0 bytes)
├── .guardrail3/
│   └── overrides/
│       ├── clippy-methods.toml   # BOM + valid entries
│       ├── deny-bans.toml        # Mix of valid/invalid lines + null bytes
│       └── mystery-file.toml     # Unrecognized override name
└── eslint.config.mjs             # Contains `}` in string literals
```

guardrail3.toml:
```toml
version = "0.1"
[profile]
name = "service"
[rust]
workspace_root = "."
[rust.apps.app]
type = "frontend"
```

### Tests:
31. Unknown type "frontend" falls back to service without crash
32. CRLF clippy.toml diff detection (false "would update"?)
33. BOM deny.toml stripped correctly
34. Empty rustfmt.toml shows "would update"
35. BOM in override file handled (stripped before validation)
36. Invalid lines in override file skipped with warning
37. Null bytes in override file handled

## Fixture 3: ts-type-confusion

TS project with ambiguous type signals.

```
ts-type-confusion/
├── package.json                  # Root workspace
├── guardrail3.toml               # Only [typescript], no apps configured
├── apps/
│   ├── hybrid/                   # Has BOTH content/ dir AND hex arch
│   │   ├── package.json          # velite in devDeps + express in deps
│   │   ├── content/
│   │   │   └── post.mdx
│   │   └── src/
│   │       └── modules/
│   │           └── domain/
│   │               └── index.ts
│   │
│   ├── bare/                     # No signals at all
│   │   └── package.json          # Basic next.js app, no content deps
│   │
│   ├── devdep-only/              # Content deps ONLY in devDependencies
│   │   └── package.json          # velite + contentlayer in devDeps
│   │
│   └── no-package-json/          # Directory exists but no package.json
│       └── src/
│           └── index.ts
│
└── packages/
    └── ui/
        └── package.json          # Library package
```

### Tests:
38. ts init --dry-run: hybrid detected as content (content/ dir wins over hex arch)
39. ts init --dry-run: bare defaults to service
40. ts init --dry-run: devdep-only detected as content (velite in devDeps)
41. ts init --dry-run: no-package-json still discovered but defaults
42. After ts init + ts generate: only one .stylelintrc.mjs if any app is content
43. ESLint config has jsx-a11y if any app is content

## Fixture 4: deep-nesting

Deeply nested Rust workspaces to stress path resolution.

```
deep-nesting/
├── Cargo.toml                    # [workspace] members=["packages/*"] exclude=["apps/*"]
├── guardrail3.toml
├── packages/
│   └── types/
│       └── Cargo.toml
├── apps/
│   ├── platform/                 # Nested workspace
│   │   ├── Cargo.toml            # [workspace] members=["crates/*"]
│   │   └── crates/
│   │       ├── core/
│   │       │   └── Cargo.toml
│   │       ├── web/
│   │       │   └── Cargo.toml
│   │       └── cli/
│   │           └── Cargo.toml
│   │
│   └── tools/                    # Another nested workspace
│       ├── Cargo.toml            # [workspace] members=["crates/*"]
│       └── crates/
│           └── migrator/
│               └── Cargo.toml
```

guardrail3.toml:
```toml
[profile]
name = "service"
[rust]
workspace_root = "."
[rust.apps.platform]
type = "service"
[rust.apps.tools]
type = "service"
[rust.packages]
type = "library"
```

### Tests:
44. platform resolves to apps/platform/ (not platform/ at root)
45. tools resolves to apps/tools/
46. Root clippy.toml uses library profile (for packages)
47. apps/platform/clippy.toml uses service profile
48. apps/tools/clippy.toml uses service profile
49. All 3 deny.toml files generated (root + 2 apps)
50. rust-toolchain.toml at root only (not per-app)

## Implementation

One test file per fixture: `adversarial_nightmare_monorepo.rs`, `adversarial_broken_configs.rs`, `adversarial_ts_type_confusion.rs`, `adversarial_deep_nesting.rs`.

Each file has a `setup_fixture(dir: &Path)` function that creates the ENTIRE directory tree with all files. Then individual `#[test]` functions run specific commands and make assertions.

Total: 50 adversarial assertions across 4 complex fixtures.
