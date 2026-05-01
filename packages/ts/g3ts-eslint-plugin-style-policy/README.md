# g3ts-eslint-plugin-style-policy

Strict style source-policy rules for G3TS-managed TypeScript, React, and Astro apps.

The package is an ESLint plugin, not a CLI. G3TS enforces that this plugin is installed and active on configured style source lanes.

## Flat Config

```js
import stylePolicy from "g3ts-eslint-plugin-style-policy";

export default [
  {
    files: ["src/**/*.{astro,js,jsx,ts,tsx}"],
    plugins: {
      "style-policy": stylePolicy
    },
    rules: {
      "style-policy/no-denied-class-tokens": [
        "error",
        {
          denyList: ["text-black", "bg-red-500"],
          classAttributes: ["class", "className"],
          classListAttributes: ["class:list"],
          classHelpers: ["cn", "clsx", "twMerge"]
        }
      ]
    }
  }
];
```

The `recommended` export is a config factory for projects that want a small helper:

```js
import { recommended as stylePolicyRecommended } from "g3ts-eslint-plugin-style-policy";

export default [
  stylePolicyRecommended({
    denyList: ["text-black"],
    classAttributes: ["class", "className"],
    classListAttributes: ["class:list"],
    classHelpers: ["cn", "clsx", "twMerge"]
  })
];
```

## Rules

- `style-policy/no-denied-class-tokens`

The rule reports configured denied class tokens in static class positions:

- `class` and `className` attributes
- Astro `class:list`
- arrays and objects used in class expressions
- conditional and logical expressions with static string branches
- configured helper calls such as `cn(...)`, `clsx(...)`, and `twMerge(...)`

Dynamic class values are not guessed. If no static denied token is visible in the AST, this rule does not report it.
