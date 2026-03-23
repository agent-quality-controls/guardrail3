export function formatCount(raw: number): string {
  return new Intl.NumberFormat("en-US").format(raw);
}

export function joinClasses(...values: Array<string | false | null | undefined>): string {
  return values.filter(Boolean).join(" ");
}
