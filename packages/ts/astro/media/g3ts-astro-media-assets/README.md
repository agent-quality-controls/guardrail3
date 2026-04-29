# g3ts-astro-media-assets

Small Astro integration for required static media assets.

It fails `astro build` when:

- the configured favicon is missing from the built output
- a configured app icon is missing from the built output
- the configured default social image is missing from the built output
- SVG is used while `allowSvgIcons` is `false`
- a configured asset path is empty, external, or traverses with `..`

G3TS only verifies that this integration is installed and wired. The actual
asset existence check runs inside Astro.

```ts
import { defineConfig } from "astro/config";
import g3tsAstroMediaAssets from "g3ts-astro-media-assets";

export default defineConfig({
  integrations: [
    g3tsAstroMediaAssets({
      favicon: "/favicon.svg",
      appIcons: ["/apple-touch-icon.png"],
      defaultSocialImage: "/og/default.png",
      allowSvgIcons: true
    })
  ]
});
```
