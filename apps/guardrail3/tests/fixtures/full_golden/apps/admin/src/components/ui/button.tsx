import type { ButtonHTMLAttributes, PropsWithChildren } from "react";

type ButtonProps = PropsWithChildren<ButtonHTMLAttributes<HTMLButtonElement>> & {
  variant?: "solid" | "ghost";
};

export function Button({
  children,
  className = "",
  variant = "solid",
  type = "button",
  ...props
}: ButtonProps): React.JSX.Element {
  const base =
    "inline-flex items-center justify-center rounded-xl px-4 py-2 text-sm font-medium transition-colors";
  const tone =
    variant === "solid"
      ? "bg-emerald-400 text-slate-950 hover:bg-emerald-300"
      : "bg-transparent text-slate-200 hover:bg-slate-800";

  return (
    <button type={type} className={`${base} ${tone} ${className}`.trim()} {...props}>
      {children}
    </button>
  );
}
