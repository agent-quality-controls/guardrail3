# eslint.config.mjs

## Location

**How ESLint flat config works:** Looks for `eslint.config.{js,mjs,cjs,ts,mts,cts}` in CWD. ONE config file loaded. No cascade, no merging, no walk-up (stable behavior). With experimental `unstable_config_lookup_from_file` flag, walks up from each file being linted to find nearest config.

**In steady-parent:**
- `eslint.config.mjs` at root (487 lines) — THE active config when `pnpm lint` runs from root
- `apps/landing/eslint.config.mjs` (34 lines) — separate config for IDE/per-app linting. Uses eslint-config-next, NOT the root config. Completely independent.

**Scoping:**
- Root config: active for `pnpm lint` / `eslint` from repo root
- Per-app configs: active only when ESLint runs from app directory (IDE, per-app scripts)
- ESLint does NOT cascade — per-app config REPLACES root entirely, no inheritance

## What's in the root config (487 lines, verified)

### Imports (10 plugins)
`@eslint/js`, `typescript-eslint`, `globals`, `@next/eslint-plugin-next`, `eslint-plugin-react`, `eslint-plugin-react-hooks`, `eslint-plugin-boundaries`, `eslint-plugin-import-x`, `eslint-plugin-tailwind-ban`, `eslint-plugin-jsx-a11y`

### Sections (in order)
1. **Global ignores** (58 patterns) — .next, node_modules, legacy, tools, scripts, content, pipeline outputs, plans/worklogs. ~44 lines. PROJECT-SPECIFIC.
2. **Base JS config** — `js.configs.recommended`. 1 line. GUARDRAIL.
3. **TypeScript strict** — `tseslint.configs.strictTypeChecked` + `stylisticTypeChecked`. 2 lines. GUARDRAIL.
4. **Language options** — globals (browser+node), projectService, tsconfigRootDir. 10 lines. MIXED (projectService is guardrail, globals are project).
5. **Next.js plugin** — recommended + core-web-vitals rules. 10 lines. PROJECT-SPECIFIC (not all projects use Next.js).
6. **React plugin** — recommended + jsx-runtime + hooks + jsx-no-leaked-render. 20 lines. GUARDRAIL (we enforce React rules).
7. **jsx-a11y strict** — all a11y rules + control-has-associated-label. 12 lines. GUARDRAIL.
8. **TS strict rules** — 30+ rules (no-explicit-any, strict-boolean-expressions, etc.). 80 lines. GUARDRAIL.
9. **Structural health** — max-lines 400, import-x/max-dependencies 15, import-x/no-cycle depth 5. 12 lines. GUARDRAIL.
10. **Test relaxation** — 15 rules relaxed for test files. 23 lines. GUARDRAIL.
11. **Config file relaxation** — return types off for config files. 7 lines. GUARDRAIL.
12. **Hex arch (admin)** — boundaries plugin with domain/application/adapters layers. 63 lines. GUARDRAIL PATTERN, project names.
13. **Landing module boundaries** — no-restricted-imports blocking workspace packages. 25 lines. GUARDRAIL PATTERN, project names.
14. **CdnImage enforcement** — no-restricted-syntax for `<img>`. 13 lines. PROJECT-SPECIFIC.
15. **Metadata boundaries** — no-restricted-imports for leaf package. 13 lines. GUARDRAIL PATTERN, project names.
16. **Pipeline boundaries** — no-restricted-imports for pipeline steps. 20 lines. PROJECT-SPECIFIC (depends on project's pipeline architecture).
17. **Design token bans** — tailwind-ban with 53 denied classes. 65 lines. PROJECT-SPECIFIC (depends on project's design system).

### Split by line count
- GUARDRAIL (sections 2-3, 6-11): ~170 lines
- GUARDRAIL PATTERN with project names (sections 12-13, 15): ~100 lines
- PROJECT-SPECIFIC (sections 1, 5, 14, 16-17): ~155 lines
- MIXED (section 4): ~10 lines

## What's in the per-app config (landing, 34 lines)

Uses `eslint-config-next` (core-web-vitals + typescript). Adds module boundary: bans all `@steady-parent/*` except `content-constraints`. That's it.

**Critical observation:** This config has ZERO guardrail rules. No TS strict, no a11y, no structural health. When ESLint runs from the landing directory, ALL guardrails are lost.

## What guardrail3 currently generates

The `eslint::build_eslint_config(has_content_app, has_service_app)` function generates ~300 lines with:
- Imports: js, tseslint, unicorn, regexp, sonarjs, react, reactHooks (+ jsx-a11y, tailwind-ban if content; + boundaries if service)
- Base configs: recommended + strictTypeChecked
- Plugins: unicorn flat/recommended with 8 disabled + 4 extra, regexp flat/recommended with 6 extra, sonarjs with 24 rules, react with 10 extra
- Core rules: max-lines 400, complexity 25, no-console, eqeqeq, no-restricted-imports (15 banned packages), no-restricted-globals (process)
- TS strict: 30+ rules
- Test relaxation: 5 rules
- Content: jsx-a11y strict, tailwind-ban with generic denyList
- Service: boundaries with generic hex arch config

**Plugins guardrail3 has that steady-parent root doesn't:** unicorn, regexp, sonarjs
**Plugins steady-parent root has that guardrail3 doesn't:** globals, @next/eslint-plugin-next, import-x

## Category: Shadow-imported

guardrail3 generates an ENGINE file at `.guardrail3/generated/eslint-engine.mjs`. The user's `eslint.config.mjs` imports and spreads it.

### Why shadow-import (not merge, not full-overwrite)

1. **Can't merge JS programmatically:** ESLint config is JavaScript, not data. It has import statements, function calls, spread operators. You can't reliably parse and merge two JS files.
2. **Can't overwrite:** 487 lines of project-specific content would be destroyed.
3. **Import composability:** ESLint flat config IS an array. `[...engine, ...project]` works. Later entries override earlier ones for the same rule key. Project-specific rules layer on top of engine rules naturally.

### How it works

**Root eslint.config.mjs (user-owned, scaffolded once):**
```js
import engine from './.guardrail3/generated/eslint-engine.mjs';
import tseslint from 'typescript-eslint';
// ... project imports ...

export default tseslint.config(
  ...engine,           // guardrail3 engine (always first)
  // ... project sections (ignores, Next.js, boundaries, tokens, etc.)
);
```

**Per-app eslint.config.mjs (user-owned, scaffolded once):**
```js
import engine from '../../.guardrail3/generated/eslint-engine.mjs';
import nextVitals from 'eslint-config-next/core-web-vitals';

export default [...engine, ...nextVitals, { /* app rules */ }];
```

**`.guardrail3/generated/eslint-engine.mjs` (guardrail3-owned, regenerated freely):**
Contains all guardrail rules. Updated on every `ts generate`. User never edits.

### What the engine exports

An array of ESLint flat config objects:
1. Base: `js.configs.recommended`, `...tseslint.configs.strictTypeChecked`
2. Language options: `projectService: true`
3. Default ignores: `**/node_modules/**`, `**/.next/**`, `**/dist/**`, `**/target/**`
4. Unicorn: recommended + disabled + extras
5. Regexp: recommended + extras
6. Sonarjs: 24 rules
7. React: recommended + extras
8. jsx-a11y: strict (if content app)
9. TS strict rules: 30+ rules
10. Structural health: max-lines 400, complexity 25
11. Test relaxation
12. Config relaxation
13. Hex arch boundary TEMPLATE (if service app) — uses app path from guardrail3.toml
14. Content isolation TEMPLATE (if content app) — blocks workspace packages

### What the user adds on top
- Project-specific ignores (legacy/**, content/**, tools/**)
- Framework plugins (Next.js, Vue, Svelte)
- CdnImage enforcement, pipeline boundaries, design token bans
- Overrides to guardrail rules (e.g., `no-console: "off"`)

## Algorithm

### On `ts generate`:
```
1. ALWAYS regenerate .guardrail3/generated/eslint-engine.mjs
   - Build from modules based on app types (content/service)
   - Include hex arch template for each service app (using paths from guardrail3.toml)
   - Include content isolation for each content app

2. If root eslint.config.mjs DOES NOT EXIST:
   - Scaffold a starter that imports the engine + has placeholder project sections

3. If root eslint.config.mjs EXISTS:
   - Do NOT touch it
   - validate checks that it imports the engine (grep for .guardrail3/generated/eslint-engine)

4. For each app that has its own eslint.config.mjs:
   - If it doesn't import the engine: validate WARNS (enforcement gap)
   - Do NOT touch the file
```

### On `ts generate --dry-run`:
```
- Show engine file status (would create/would update)
- Show root config status:
  - If missing: "would scaffold root eslint.config.mjs"
  - If exists without engine import: "WARNING: root config does not import guardrail3 engine — guardrail rules may not be active"
  - If exists with engine import: "no changes needed"
- For each per-app config:
  - If exists without engine import: "WARNING: {app}/eslint.config.mjs does not import engine"
```

## Override mechanism

No override files needed. The user's eslint.config.mjs IS the override mechanism — entries after `...engine` override engine rules. If the user wants `no-console: "off"`, they add it in their config section and it overrides the engine's `no-console: "error"`.

## Known bugs in current engine (eslint.rs) — must fix before shipping

**BUG 1: Missing `files` scoping on TS rules.** The engine's TS strict rules section, core rules section, sonarjs section have NO `files` filter. They apply to ALL files including `.js`. TS-specific rules (no-explicit-any, strict-boolean-expressions, etc.) will error on `.js` files because the TS parser can't handle them. Fix: add `files: ["**/*.ts", "**/*.tsx"]` to every TS-specific config block.

**BUG 2: Missing `stylisticTypeChecked`.** Engine only spreads `strictTypeChecked`. Steady-parent also uses `stylisticTypeChecked` which adds: prefer-for-of, prefer-string-starts-ends-with, prefer-function-type, no-inferrable-types, array-type, etc. Fix: add `...tseslint.configs.stylisticTypeChecked` to CONFIG_OPEN.

**BUG 3: Scaffold missing `tsconfigRootDir`.** The scaffolded root eslint.config.mjs must include `tsconfigRootDir: import.meta.dirname` in languageOptions.parserOptions. Without it, type-aware rules break when ESLint runs from subdirectories (which IDEs do). The ENGINE correctly omits it (it's resolved from the root config file), but the SCAFFOLD must include it.

**BUG 4: Hex arch boundaries are generic/unscoped.** The current `SERVICE_BOUNDARIES_SECTION` uses generic patterns (`src/domain/**`) with no `files` filter and no `basePattern`. For monorepos with multiple service apps, this can't distinguish between `apps/admin/src/domain/` and `apps/api/src/domain/`. Fix: generate per-app boundary blocks with `files: ['apps/{name}/**']` and correct `basePattern`.

**BUG 5: Content isolation not implemented.** Engine has no `no-restricted-imports` for content apps. The `CONTENT_TAILWIND_BAN_SECTION` bans generic CSS prefixes (`bg-`, `text-`) which would ban ALL such classes, not just design token violations. Fix: implement workspace package discovery and generate content-isolation import restrictions. Fix tailwind-ban to use project-specific deny list or remove from engine (let user configure).

## Edge cases

1. **User already has eslint.config.mjs without engine import:** This is the migration case. `ts generate` should NOT modify their file. validate warns about the missing import. The user manually adds the import when ready. Alternatively, `guardrail3 ts adopt` could insert the import (future feature).

2. **Engine and user config have conflicting `languageOptions`:** ESLint flat config merges languageOptions at the object level. Engine sets `projectService: true`, user sets `globals: { ...globals.browser }`. Both apply — no conflict. If both set the same parser option, last one wins (user's overrides engine's).

3. **Engine has `**/node_modules/**` ignore, user has their own ignores:** ESLint flat config ignores are per-config-object. The engine's ignore block and the user's ignore block are separate objects in the array. Both apply (union). No conflict.

4. **Per-app config uses Vue/Svelte parser:** The engine targets `**/*.ts` and `**/*.tsx` files. Vue parser targets `**/*.vue`. Different file patterns — they coexist in the same array without conflict.

5. **`tsconfigRootDir` mismatch:** The engine might set `tsconfigRootDir` to something, but the user's config needs it relative to their `eslint.config.mjs` location. Solution: the engine should NOT set `tsconfigRootDir` — let the user's config set it. Engine only sets `projectService: true`.

6. **import-x/max-dependencies and import-x/no-cycle:** These are currently in steady-parent's root config but NOT in guardrail3's engine. They should be ADDED to the engine. But they require `eslint-plugin-import-x` which is a dep the user must install. The engine should import it conditionally or document it as a required dep.

7. **Workspace package names for content isolation:** The engine needs to know workspace package names to generate `no-restricted-imports` for content apps. These come from: (a) reading `pnpm-workspace.yaml` or root `package.json` workspaces field, (b) discovering package names from `packages/*/package.json`. This is discoverable at generate time.

8. **Hex arch layer paths:** The engine generates boundary config for service apps. It needs the app's `src/modules/` path relative to repo root. This comes from guardrail3.toml app config (we know the app path from discovery).

## Dependencies (npm packages the user must have)

For the engine to work, the user's project must have these in devDependencies:
- `@eslint/js`
- `typescript-eslint`
- `eslint-plugin-unicorn`
- `eslint-plugin-regexp`
- `eslint-plugin-sonarjs`
- `eslint-plugin-react`
- `eslint-plugin-react-hooks`
- If content app: `eslint-plugin-jsx-a11y`, `eslint-plugin-tailwind-ban`
- If service app: `eslint-plugin-boundaries`

validate (T-PLUG checks) already verifies these are installed.

## Summary

| What | Who owns | Where |
|---|---|---|
| Engine (guardrail rules) | guardrail3 | `.guardrail3/generated/eslint-engine.mjs` |
| Root config (project rules) | User | `eslint.config.mjs` |
| Per-app config | User | `apps/*/eslint.config.mjs` |
| Plugin installation | Validate checks | `package.json` devDependencies |
