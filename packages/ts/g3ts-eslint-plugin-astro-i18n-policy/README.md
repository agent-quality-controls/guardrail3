# g3ts-eslint-plugin-astro-i18n-policy

Astro i18n source-policy rules for G3TS-managed content apps.

The package is an ESLint plugin, not a CLI. G3TS enforces that this plugin is installed and active on the configured Astro public source lanes.

## Rules

- `astro-i18n-policy/no-unlocalized-internal-hrefs`
- `astro-i18n-policy/no-inline-image-alt`
- `astro-i18n-policy/require-content-image-key`

Every rule requires explicit options. Missing required options reports an ESLint config error.
