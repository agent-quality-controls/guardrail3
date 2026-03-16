// process.env is banned, use config module
// GREP_BUG: grep flags T30, tree-sitter should NOT flag this.
const config = { env: "production" };
export default config;
