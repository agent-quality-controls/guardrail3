// Fixture: ": any" pattern inside a string literal.
// GREP_BUG: grep flags T31, tree-sitter should NOT flag this.
const s = ": any";
export default s;
