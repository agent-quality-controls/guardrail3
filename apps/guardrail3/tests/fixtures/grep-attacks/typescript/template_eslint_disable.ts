// Fixture: eslint-disable pattern inside a template literal.
// GREP_BUG: grep flags T23, tree-sitter should NOT flag this.
const s = `eslint-disable`;
export default s;
