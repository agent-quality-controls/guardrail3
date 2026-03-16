// Fixture: real `any` usage in a generic type parameter default.
// CORRECT: This IS actual any usage and SHOULD be flagged by T31.
function foo<T = any>(): T {
  return null as T;
}
export default foo;
