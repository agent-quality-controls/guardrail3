import stylePolicy from "g3ts-eslint-plugin-style-policy";
import eslintComments from "@eslint-community/eslint-plugin-eslint-comments";

export default [
  {
    files: ["src/**/*.tsx"],
    plugins: {
      "style-policy": stylePolicy,
      "@eslint-community/eslint-comments": eslintComments,
    },
    rules: {
      "style-policy/no-denied-class-tokens": [
        "error",
        { denyPrefixes: ["text-["] },
      ],
      "@eslint-community/eslint-comments/no-restricted-disable": [
        "error",
        "style-policy/*",
        "tailwind-ban/*",
      ],
    },
  },
];
