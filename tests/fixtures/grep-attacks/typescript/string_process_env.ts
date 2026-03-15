// Fixture: process.env pattern inside a string literal.
// GREP_BUG: grep flags T30, tree-sitter should NOT flag this.
const s = "process.env.NODE_ENV";
export default s;
