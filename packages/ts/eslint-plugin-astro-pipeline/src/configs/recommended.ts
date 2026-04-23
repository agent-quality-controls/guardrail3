const recommended = {
  rules: {
    "astro-pipeline/no-authored-content-fs-read": "error",
    "astro-pipeline/no-authored-content-glob": "error",
    "astro-pipeline/no-direct-astro-content-in-routes": "error",
    "astro-pipeline/no-runtime-mdx-eval": "error",
    "astro-pipeline/no-side-loader-imports": "error"
  }
} as const;

export default recommended;
