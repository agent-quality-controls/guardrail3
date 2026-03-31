export type BannerTone = "info" | "warning" | "success";

export type BannerProps = {
  title: string;
  description: string;
  tone?: BannerTone;
};

export function renderBanner({ title, description, tone = "info" }: BannerProps): string {
  return `[${tone.toUpperCase()}] ${title}: ${description}`;
}

export function stackClasses(...values: Array<string | false | null | undefined>): string {
  return values.filter(Boolean).join(" ");
}
