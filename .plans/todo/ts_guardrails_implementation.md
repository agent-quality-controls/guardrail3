# Guardrails Implementation Plan — Presets + Diffs

Practical implementation guide. Start from recommended presets, then list only the delta.

Companion to `ts_new_guardrails.md` (full rule inventory).

---

## 1. `eslint-plugin-unicorn` — `flat/recommended` preset + overrides

**Base:** `unicorn.configs['flat/recommended']` (enables ~100 rules at error level, disables ~40 that are too opinionated)

**Disable — conflicts with our stack:**

| Rule | Why disable |
|------|------------|
| `unicorn/no-null` | Conflicts with `exactOptionalPropertyTypes: true` in tsconfig. DOM APIs and libraries return `null`. |
| `unicorn/prevent-abbreviations` | Renames `props` → `properties`, `params` → `parameters`, `req` → `request`. Fights React/Next.js conventions. |
| `unicorn/filename-case` | Next.js requires `page.tsx`, `layout.tsx`, `[slug]`, `route.ts`. Framework conventions win. |
| `unicorn/no-process-exit` | Pipeline scripts and CLI tools legitimately need `process.exit()`. |
| `unicorn/no-array-reduce` | `.reduce()` is fine for simple accumulations. Banning it entirely is a style opinion. |
| `unicorn/no-array-callback-reference` | Bans `array.map(Number)`. Prevents a real footgun with multi-arg callbacks but also bans safe single-arg cases. Too noisy. |
| `unicorn/no-useless-undefined` | Conflicts with `exactOptionalPropertyTypes` — sometimes you must pass explicit `undefined`. |
| `unicorn/prefer-module` | Config files (`.mjs`, `.js`) sometimes need CJS patterns. Next.js config files use CJS. |

**Enable — not in recommended but valuable:**

| Rule | Why enable |
|------|-----------|
| `unicorn/no-keyword-prefix` | Prevents `newUser`, `classNames` as identifiers — confusing with JS keywords. |
| `unicorn/no-unused-properties` | Catches dead object properties. |
| `unicorn/require-post-message-target-origin` | Security — `postMessage()` without `targetOrigin` leaks data cross-origin. |
| `unicorn/no-anonymous-default-export` | Forces named exports — better stack traces, refactoring safety. |

---

## 2. `eslint-plugin-regexp` — `flat/recommended` preset + overrides

**Base:** `regexp.configs['flat/recommended']` (enables ~67 rules — nearly all bug-catchers and consistency rules)

**Disable — nothing.** The recommended preset is well-curated. No conflicts with our stack.

**Enable — not in recommended but valuable:**

| Rule | Why enable |
|------|-----------|
| `regexp/require-unicode-regexp` | Enforce `u` flag on all regexes — fixes Unicode handling bugs. |
| `regexp/require-unicode-sets-regexp` | Enforce `v` flag where possible — stricter and more correct than `u`. |
| `regexp/prefer-named-capture-group` | Named groups make regex maintenance easier — agents write better regex when forced to name captures. |
| `regexp/prefer-named-backreference` | Same rationale — named references are self-documenting. |
| `regexp/prefer-result-array-groups` | Use `match.groups.name` over positional `match[1]` — less fragile. |
| `regexp/no-misleading-capturing-group` | Catches capturing groups that don't behave as expected. |

---

## 3. `eslint-plugin-sonarjs` — NO preset. Cherry-pick only.

**Base:** None. The `recommended` preset enables 268 rules, ~50 of which duplicate rules we already have, ~30 are irrelevant (AWS, Angular, Chai, Mocha). Use individual rules only.

**Cherry-pick — unique rules nothing else provides:**

| Rule | Config | What it catches |
|------|--------|-----------------|
| `sonarjs/cognitive-complexity` | `['error', 15]` | Functions too hard to understand — unique metric, not same as cyclomatic `complexity` |
| `sonarjs/no-identical-functions` | `'error'` | Two functions with identical bodies — AST-level duplicate detection |
| `sonarjs/no-all-duplicated-branches` | `'error'` | Every branch in a conditional has the same code |
| `sonarjs/no-duplicated-branches` | `'error'` | Two branches with the same code |
| `sonarjs/no-collapsible-if` | `'error'` | `if (a) { if (b) {} }` → should be combined |
| `sonarjs/no-identical-conditions` | `'error'` | Same condition in `if/else if` chain |
| `sonarjs/no-identical-expressions` | `'error'` | `a === a`, `x && x` — always-true/false |
| `sonarjs/no-inverted-boolean-check` | `'error'` | `if (!x !== y)` — logic error |
| `sonarjs/no-redundant-boolean` | `'error'` | `x === true`, `y ? true : false` |
| `sonarjs/prefer-single-boolean-return` | `'error'` | `if (x) return true; return false;` → `return x` |
| `sonarjs/no-gratuitous-expressions` | `'error'` | Boolean expressions that are always true or false |
| `sonarjs/no-invariant-returns` | `'error'` | Function always returns the same value regardless of branches |
| `sonarjs/no-collection-size-mischeck` | `'error'` | `collection.length >= 0` — always true |
| `sonarjs/no-empty-collection` | `'error'` | Iterating or accessing an always-empty collection |
| `sonarjs/no-element-overwrite` | `'error'` | Writing to same collection index unconditionally — second write kills first |
| `sonarjs/no-unused-collection` | `'error'` | Collection populated but never read |
| `sonarjs/no-use-of-empty-return-value` | `'error'` | Using return value of a void function |
| `sonarjs/no-nested-switch` | `'error'` | Switch inside switch — unreadable |
| `sonarjs/no-nested-template-literals` | `'error'` | Template literal inside template literal — unreadable |
| `sonarjs/no-redundant-jump` | `'error'` | Useless `break`, `continue`, `return` at end of block |
| `sonarjs/expression-complexity` | `['error', 4]` | Single expression too complex — forces decomposition |
| `sonarjs/no-async-constructor` | `'error'` | Async operations in constructor — common agent mistake |
| `sonarjs/no-hook-setter-in-body` | `'error'` | React useState setter called in component body — infinite loop |
| `sonarjs/no-useless-react-setstate` | `'error'` | `setState(currentState)` — no-op that triggers re-render |
| `sonarjs/no-duplicate-string` | `['error', 5]` | String literal duplicated 5+ times — catches magic strings agents scatter. Threshold 5 avoids noise from short common strings. |

---

## 4. React rules — add to existing React plugin section

**Base:** Already using `pluginReact.configs.flat.recommended` + `jsx-runtime` + hooks recommended.

**Enable — all bug-catchers outside recommended:**

| Rule | Config | What it catches |
|------|--------|-----------------|
| `react/no-unstable-nested-components` | `'error'` | Component defined inside render — remount every render |
| `react/no-danger` | `'error'` | `dangerouslySetInnerHTML` — XSS vector |
| `react/iframe-missing-sandbox` | `'error'` | `<iframe>` without sandbox — security |
| `react/no-array-index-key` | `'error'` | Array index as key — rendering bugs |
| `react/button-has-type` | `'error'` | `<button>` defaults to "submit" — accidental form submits |
| `react/jsx-no-script-url` | `'error'` | `href="javascript:..."` — XSS |
| `react/jsx-no-constructed-context-values` | `'error'` | Inline context object — all consumers re-render |
| `react/no-invalid-html-attribute` | `'error'` | Invalid `rel` values |
| `react/hook-use-state` | `'error'` | `[value, setValue]` naming |
| `react/checked-requires-onchange-or-readonly` | `'error'` | Controlled checkbox without handler |

(`react/jsx-no-leaked-render` already added in the current session.)

---

## 5. Built-in ESLint + TypeScript rules — add to existing sections

**Add to the "Strict TypeScript Rules" section:**

| Rule | Config |
|------|--------|
| `no-param-reassign` | `'error'` |
| `@typescript-eslint/no-shadow` | `'error'` |
| `complexity` | `['error', 15]` |
| `max-depth` | `['error', 4]` |
| `max-params` | `['error', 4]` |
| ~~`no-nested-ternary`~~ | SKIP — `unicorn/no-nested-ternary` (in recommended preset) already covers this with better error messages. Enabling both causes double-reporting. |
| `max-nested-callbacks` | `['error', 4]` |
| `no-return-assign` | `['error', 'always']` |
| `@typescript-eslint/only-throw-error` | `'error'` |
| `prefer-template` | `'error'` |
| `object-shorthand` | `['error', 'always']` |
| `no-sequences` | `'error'` |
| `no-void` | `'error'` |
| `@typescript-eslint/switch-exhaustiveness-check` | `'error'` |
| `@typescript-eslint/no-confusing-void-expression` | `['error', { ignoreArrowShorthand: true }]` |
| `@typescript-eslint/naming-convention` | See note below |
| `@typescript-eslint/method-signature-style` | `['error', 'property']` |

**`naming-convention` config:**
```js
'@typescript-eslint/naming-convention': ['error',
  { selector: 'default', format: ['camelCase'] },
  { selector: 'variable', format: ['camelCase', 'UPPER_CASE'] },
  { selector: 'variable', modifiers: ['const', 'exported'], format: ['camelCase', 'UPPER_CASE', 'PascalCase'] },
  { selector: 'parameter', format: ['camelCase'], leadingUnderscore: 'allow' },
  { selector: 'typeLike', format: ['PascalCase'] },
  { selector: 'enumMember', format: ['PascalCase', 'UPPER_CASE'] },
  { selector: 'property', format: null },  // too many external APIs with varying conventions
  { selector: 'function', format: ['camelCase', 'PascalCase'] },  // PascalCase for React components
]
```

**Relax in test files (add to existing test override):**

| Rule | Config in tests |
|------|----------------|
| `max-params` | `'off'` |
| `@typescript-eslint/naming-convention` | `'off'` |
| `sonarjs/cognitive-complexity` | `'off'` |
| `sonarjs/no-identical-functions` | `'off'` |
| `sonarjs/no-duplicate-string` | `'off'` |

---

## 6. Stylelint — CSS quality + accessibility

### Packages

```bash
pnpm add -Dw stylelint stylelint-config-standard stylelint-config-tailwindcss @double-great/stylelint-a11y
```

| Package | Version | Purpose |
|---------|---------|---------|
| `stylelint` | 17.4.0 | Core CSS linter |
| `stylelint-config-standard` | 40.0.0 | Standard CSS quality rules (property ordering, valid values, duplicate detection, etc.) |
| `stylelint-config-tailwindcss` | 1.0.1 | Registers Tailwind v4 at-rules (`@theme`, `@apply`, `@plugin`, `@custom-variant`, `@utility`, `@source`, `@variant`, `@reference`, `@layer`, `@config`) and functions (`theme()`) as known syntax instead of disabling `at-rule-no-unknown` entirely |
| `@double-great/stylelint-a11y` | 3.4.6 | CSS accessibility rules. Maintained fork of abandoned `stylelint-a11y` (which only supports stylelint ≤13). Requires stylelint ≥16. |

### Config file: `.stylelintrc.mjs`

```js
/** @type {import('stylelint').Config} */
export default {
  extends: [
    'stylelint-config-standard',
    'stylelint-config-tailwindcss',
  ],

  plugins: ['@double-great/stylelint-a11y'],

  rules: {
    // ── Accessibility — all rules from strict preset ──────────────
    'a11y/content-property-no-static-value': true,  // screen readers announce static content:
    'a11y/font-size-is-readable': true,              // minimum readable font sizes
    'a11y/line-height-is-vertical-rhythmed': true,   // line-height vertical rhythm
    'a11y/media-prefers-reduced-motion': true,        // require prefers-reduced-motion for animations
    'a11y/no-display-none': true,                     // flag display:none on meaningful content
    'a11y/no-obsolete-attribute': true,               // obsolete HTML attributes in selectors
    'a11y/no-obsolete-element': true,                 // obsolete HTML elements in selectors
    'a11y/no-outline-none': true,                     // no removing focus outlines without replacement
    'a11y/no-spread-text': true,                      // no excessive letter-spacing
    'a11y/no-text-align-justify': true,               // no text-align:justify (dyslexia hazard)
    'a11y/selector-pseudo-class-focus': true,         // require :focus wherever :hover exists

    // ── Architecture exceptions ───────────────────────────────────
    'a11y/media-prefers-color-scheme': null,  // we use class-based dark mode (.dark), not @media
    'no-duplicate-selectors': null,            // separate :root blocks for different concerns

    // ── Notation conventions ──────────────────────────────────────
    'lightness-notation': 'number',     // oklch: 0.988 not 98.8%
    'hue-degree-notation': null,        // oklch: 78.293 not 78.293deg
    'alpha-value-notation': 'number',   // oklch: 0.4 not 40%

    // ── Tailwind v4 compat ────────────────────────────────────────
    'no-descending-specificity': [true, { ignore: ['selectors-within-list'] }],
    'custom-property-pattern': null,
    'selector-class-pattern': null,
    'declaration-block-no-redundant-longhand-properties': null,
    'declaration-block-no-duplicate-custom-properties': null,
  },

  ignoreFiles: [
    'node_modules/**', '.next/**', 'target/**',
    'legacy/**', 'dist/**', 'coverage/**', '.velite/**',
  ],
};
```

### Pre-commit hook addition

Add CSS detection + stylelint step to `.githooks/pre-commit`, after ESLint, before Rust checks:

```bash
# --- Detect CSS changes ---
CSS_CHANGED=$(echo "$STAGED_FILES" | grep -cE '\.css$' || true)

# --- CSS checks (only if CSS files changed) ---
if [ "$CSS_CHANGED" -gt 0 ]; then
    echo "Running Stylelint (CSS quality + accessibility)..."
    if ! pnpm exec stylelint --max-warnings 0 $(echo "$STAGED_FILES" | grep -E '\.css$'); then
        echo "Stylelint failed. Fix CSS quality/accessibility issues before committing."
        exit 1
    fi
fi
```

### Inline disable syntax

When `display: none` is intentional (e.g., hiding a remark plugin's decorative label):

```css
/* stylelint-disable-next-line a11y/no-display-none -- decorative label from remark plugin */
.markdown-alert > p:first-child {
  display: none;
}
```

Note: the disable comment goes **before the rule selector**, not inside the declaration block — the a11y plugin reports on the selector, not the declaration.

---

## 7. `eslint-plugin-jsx-a11y` — JSX accessibility (WCAG)

### Package

```bash
pnpm add -Dw eslint-plugin-jsx-a11y
```

| Package | Version | Purpose |
|---------|---------|---------|
| `eslint-plugin-jsx-a11y` | 6.10.2 | JSX accessibility linting powered by `axe-core`. Strict mode enables all 32 rules at error level. |

Note: `axe-core` is a transitive dependency — no separate install needed.

### ESLint config addition

```js
import jsxA11y from 'eslint-plugin-jsx-a11y';

// In the config array, after the React plugin section:
{
  files: ['**/*.tsx', '**/*.jsx'],
  ...jsxA11y.flatConfigs.strict,
  rules: {
    ...jsxA11y.flatConfigs.strict.rules,
    // Strict preset leaves this off — turn it on for max coverage
    'jsx-a11y/control-has-associated-label': 'error',
  },
},
```

### What the strict preset enables (32 rules, all at error level)

| Rule | What it catches |
|------|-----------------|
| `jsx-a11y/alt-text` | Missing alt text on `<img>`, `<area>`, `<input type="image">`, `<object>` |
| `jsx-a11y/anchor-has-content` | `<a>` with no text content — invisible to screen readers |
| `jsx-a11y/anchor-is-valid` | `<a>` with no `href` or invalid `href` |
| `jsx-a11y/aria-activedescendant-has-tabindex` | `aria-activedescendant` without `tabIndex` |
| `jsx-a11y/aria-props` | Invalid ARIA attributes |
| `jsx-a11y/aria-proptypes` | Wrong ARIA attribute value types |
| `jsx-a11y/aria-role` | Invalid ARIA roles |
| `jsx-a11y/aria-unsupported-elements` | ARIA on elements that don't support it |
| `jsx-a11y/autocomplete-valid` | Invalid `autocomplete` values |
| `jsx-a11y/click-events-have-key-events` | `onClick` without keyboard equivalent |
| `jsx-a11y/control-has-associated-label` | Interactive control with no text label (**we add this — strict leaves it off**) |
| `jsx-a11y/heading-has-content` | Empty headings |
| `jsx-a11y/html-has-lang` | `<html>` without `lang` attribute |
| `jsx-a11y/iframe-has-title` | `<iframe>` without `title` |
| `jsx-a11y/img-redundant-alt` | Alt text containing "image" or "picture" |
| `jsx-a11y/interactive-supports-focus` | Interactive role without `tabIndex` |
| `jsx-a11y/label-has-associated-control` | `<label>` not linked to a form control |
| `jsx-a11y/media-has-caption` | `<audio>`/`<video>` without `<track>` for captions |
| `jsx-a11y/mouse-events-have-key-events` | `onMouseOver`/`onMouseOut` without keyboard equivalents |
| `jsx-a11y/no-access-key` | `accessKey` prop — inconsistent across browsers |
| `jsx-a11y/no-autofocus` | `autoFocus` — disorienting for screen reader users |
| `jsx-a11y/no-distracting-elements` | `<marquee>`, `<blink>` |
| `jsx-a11y/no-interactive-element-to-noninteractive-role` | Interactive element with noninteractive role |
| `jsx-a11y/no-noninteractive-element-interactions` | Click handlers on `<div>`, `<span>` etc. |
| `jsx-a11y/no-noninteractive-element-to-interactive-role` | Noninteractive element with interactive role |
| `jsx-a11y/no-noninteractive-tabindex` | `tabIndex` on noninteractive elements |
| `jsx-a11y/no-redundant-roles` | `<button role="button">` — already implicit |
| `jsx-a11y/no-static-element-interactions` | Click handlers on static elements without role |
| `jsx-a11y/role-has-required-aria-props` | Role missing required ARIA attributes |
| `jsx-a11y/role-supports-aria-props` | ARIA attribute not supported by element's role |
| `jsx-a11y/scope` | `scope` only valid on `<th>` elements |
| `jsx-a11y/tabindex-no-positive` | Positive `tabIndex` — disrupts natural tab order |

### Additional React rule for accessibility

Added to the React plugin section (not jsx-a11y, but a11y-relevant):

```js
'react/jsx-no-leaked-render': ['error', { validStrategies: ['ternary', 'coerce'] }],
```

Prevents `{count && <Comp />}` from rendering "0" — both a visual bug and a screen reader issue.

---

## 8. `eslint-plugin-tailwind-ban` — design token enforcement

Prevents raw Tailwind utility classes when a semantic design token exists. Forces components to use the design system vocabulary instead of ad-hoc visual classes.

### Package

```bash
pnpm add -Dw eslint-plugin-tailwind-ban
```

### ESLint config

Applied to component TSX files. Exempt: `components/ui/` and `components/pro-blocks/` (vendor-managed by shadcn).

```js
import tailwindBan from 'eslint-plugin-tailwind-ban';

{
  files: ['apps/landing/src/**/*.tsx'],
  ignores: ['apps/landing/src/components/ui/**', 'apps/landing/src/components/pro-blocks/**'],
  plugins: {
    'tailwind-ban': tailwindBan,
  },
  rules: {
    'tailwind-ban/no-deny-tailwind-tokens': ['warn', {
      denyList: [
        // Typography colors — use body-text or meta-text tokens
        'text-muted-foreground', 'text-foreground',
        'text-foreground/80', 'text-foreground/85', 'text-foreground/90',
        'text-muted-foreground/85', 'text-muted-foreground/60',
        // Typography size + line-height — use body-text token
        'text-lg/8', 'text-lg/7', 'text-base/7', 'text-base/6',
        // Line-height — baked into body-text and meta-text tokens
        'leading-relaxed', 'leading-snug', 'leading-5', 'leading-6',
        // Prose color overrides — handled by prose.css via design tokens
        'prose-neutral',
      ],
    }],
  },
},
```

### How it works

The `denyList` is project-specific — it lists the raw Tailwind classes that have been replaced by semantic design token utility classes (e.g., `.body-text`, `.meta-text`, `.heading-xl`). When an agent writes `text-muted-foreground` in a component, the rule fires and tells them to use the token instead.

The deny list grows over time as more tokens are created. Layout utilities (`flex`, `grid`, `p-4`, `gap-2`) stay as raw Tailwind — only visual decisions (colors, typography, spacing between content) get tokenized.

---

## 9. `knip` — standalone dead code detection

Install: `pnpm add -Dw knip`

Add script to `package.json`:
```json
"knip": "knip"
```

Run periodically or add to pre-commit for non-TS changes. Not in the ESLint pipeline — it's a separate analysis pass.

---

## Performance Budget

| Component | Current | After all additions | Delta |
|-----------|---------|-------------------|-------|
| ESLint (full app) | 28.6s | ~36s | +7s (+25%) |
| ESLint (3-5 staged files) | ~3s | ~4s | +1s |
| Stylelint | 1.7s | 1.7s | 0 |
| Pre-commit total | ~15s | ~17s | +2s |

Acceptable. The bottleneck remains type-aware TS-ESLint rules, not the new plugins.

---

## Summary: What to install and configure

```bash
pnpm add -Dw eslint-plugin-jsx-a11y eslint-plugin-unicorn eslint-plugin-sonarjs eslint-plugin-regexp eslint-plugin-tailwind-ban knip stylelint stylelint-config-standard stylelint-config-tailwindcss @double-great/stylelint-a11y
```

| Plugin | Strategy | Rules enabled |
|--------|----------|---------------|
| jsx-a11y | `flatConfigs.strict` + `control-has-associated-label` ON | 32 |
| Stylelint + a11y | `config-standard` + `config-tailwindcss` + `@double-great/stylelint-a11y` strict − 1 disabled | 11 a11y + standard CSS |
| tailwind-ban | `no-deny-tailwind-tokens` with project-specific deny list | 1 (configurable) |
| unicorn | `flat/recommended` − 8 disabled + 4 enabled | ~96 |
| regexp | `flat/recommended` + 6 enabled | ~73 |
| sonarjs | Cherry-pick only, no preset | 24 |
| React | Add 10 rules + `jsx-no-leaked-render` to existing section | 11 |
| Built-in ESLint/TS | Add 17 rules to existing section | 17 |
| Pre-commit hook | Add CSS lint step (stylelint on staged .css files) | — |
| knip | Standalone dead code detection | — |
| **Total new rules** | | **~265** |
