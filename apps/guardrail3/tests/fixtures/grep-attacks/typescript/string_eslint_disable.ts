// Fixture: eslint-disable pattern inside a string literal.
// GREP_BUG: grep flags T23, tree-sitter should NOT flag this.
const s = "eslint-disable-next-line";
export default s;
