# .stylelintrc.mjs

## Location

**Where stylelint looks:** cosmiconfig walk-up from linted file. Checks: `stylelint` key in package.json, `.stylelintrc`, `.stylelintrc.{json,yaml,yml,js,cjs,mjs}`, `stylelint.config.{js,cjs,mjs}`. Nearest wins, no merge. Supports `extends`.

**In steady-parent:** `.stylelintrc.mjs` at root (74 lines). No per-app stylelint configs.

## Contents (verified)

```
extends: stylelint-config-standard, stylelint-config-tailwindcss     ← GUARDRAIL
plugins: @double-great/stylelint-a11y                                 ← GUARDRAIL
11 a11y rules (all true)                                              ← GUARDRAIL
media-prefers-color-scheme: null (dark mode exception)                ← GUARDRAIL (architecture decision)
no-duplicate-selectors: null (theme tokens)                           ← GUARDRAIL (architecture decision)
lightness-notation: 'number'                                          ← PROJECT (oklch convention)
hue-degree-notation: null                                             ← PROJECT (oklch convention)
alpha-value-notation: 'number'                                        ← PROJECT (oklch convention)
no-descending-specificity: [true, {ignore}]                          ← PROJECT (Tailwind v4 compat)
custom-property-pattern: null                                         ← PROJECT (Tailwind compat)
selector-class-pattern: null                                          ← PROJECT (Tailwind compat)
declaration-block-no-redundant-longhand-properties: null              ← PROJECT (Tailwind compat)
declaration-block-no-duplicate-custom-properties: null                ← PROJECT (Tailwind compat)
ignoreFiles: [7 patterns]                                             ← PROJECT
```

~45 lines guardrail / ~29 lines project.

## Category: Shadow-imported

Same model as ESLint. Stylelint supports `extends` natively, making this cleaner.

**`.guardrail3/generated/stylelint-engine.mjs` (guardrail3-owned):**
```js
export default {
  extends: ['stylelint-config-standard', 'stylelint-config-tailwindcss'],
  plugins: ['@double-great/stylelint-a11y'],
  rules: {
    'a11y/content-property-no-static-value': true,
    // ... 11 a11y rules ...
    'a11y/media-prefers-color-scheme': null,
    'no-duplicate-selectors': null,
  },
};
```

**User's `.stylelintrc.mjs` (user-owned):**
```js
import engine from './.guardrail3/generated/stylelint-engine.mjs';
export default {
  ...engine,
  rules: {
    ...engine.rules,
    // project CSS conventions
    'lightness-notation': 'number',
    'custom-property-pattern': null,
    // ...
  },
  ignoreFiles: ['node_modules/**', '.next/**', 'legacy/**'],
};
```

## Algorithm

Same as ESLint:
1. Always regenerate `.guardrail3/generated/stylelint-engine.mjs`
2. If root `.stylelintrc.*` doesn't exist: scaffold starter that imports engine
3. If exists: don't touch. Validate checks engine import.
4. Only generate stylelint engine if at least one content app (stylelint is for CSS — only relevant for apps with UI)

## Edge cases

1. **Stylelint extends can reference engine directly:** Unlike ESLint (which needs array spread), stylelint config objects can be deep-merged with spread. The `extends` array in the engine adds standard+tailwindcss. The user's config adds the engine as a base. This works because stylelint config is a plain object, not an array of config objects.

2. **Only for content apps?** Stylelint checks CSS. Service-only projects (pure API) don't have CSS files. But: admin in steady-parent IS a service app that HAS Tailwind CSS. So stylelint is relevant for any app with CSS, not just "content" type. Should be generated when ANY app has CSS, which in practice means any app that's not a pure library/CLI.
