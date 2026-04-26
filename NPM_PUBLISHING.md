# NPM Publishing

This repo keeps npm credentials in root `.env.local` as `NPM_TOKEN`.

Do not use `npm whoami` as the publish gate for this token. On 2026-04-26 it returned `E401 Unauthorized`, while the same token successfully published packages. Treat `npm publish` itself as the auth check.

Use a temporary npm config that writes the token value into the config file. Do not rely on npm expanding `${NPM_TOKEN}` from `.npmrc`.

```sh
set -a
source /Users/tartakovsky/Projects/websmasher/guardrail3/.env.local
set +a

tmp_npmrc="$(mktemp)"
printf '%s\n' 'registry=https://registry.npmjs.org/' > "$tmp_npmrc"
printf '//registry.npmjs.org/:_authToken=%s\n' "$NPM_TOKEN" >> "$tmp_npmrc"

npm publish --userconfig "$tmp_npmrc" --access public --registry=https://registry.npmjs.org/
publish_status=$?

rm -f "$tmp_npmrc"
exit "$publish_status"
```

Run that command from the package directory being published.

Current G3TS npm packages:

- `packages/ts/g3ts-eslint-plugin-astro-pipeline`
- `packages/ts/g3ts-astro-nuasite-checks`

Verify publication with:

```sh
npm view g3ts-eslint-plugin-astro-pipeline version --registry=https://registry.npmjs.org/
npm view g3ts-astro-nuasite-checks version --registry=https://registry.npmjs.org/
```
