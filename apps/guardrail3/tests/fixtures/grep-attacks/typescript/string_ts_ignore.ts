// Fixture: @ts-ignore pattern inside a string literal.
// GREP_BUG: grep flags T27, tree-sitter should NOT flag this.
const s = "@ts-ignore";
export default s;
