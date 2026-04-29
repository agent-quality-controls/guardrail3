# g3ts-eslint-plugin-astro-media-policy

Astro media source-policy rules for G3TS-managed content apps.

The package is an ESLint plugin, not a CLI. G3TS enforces that this plugin is installed and active on the configured Astro public source lanes.

## Rules

- `astro-media-policy/no-raw-public-image-paths`
- `astro-media-policy/no-inline-image-alt`
- `astro-media-policy/require-content-image-key`
- `astro-media-policy/require-approved-media-helper`

Every rule requires explicit options. Missing required options reports an ESLint config error.

`no-raw-public-image-paths` blocks direct root-relative image paths in source unless they are explicitly allowed.

`no-inline-image-alt` blocks inline alt text on configured content image components.

`require-content-image-key` requires configured content image components to use configured image key props instead of raw source props.

`require-approved-media-helper` blocks raw metadata image strings on configured metadata fields so page metadata uses approved media helpers.
