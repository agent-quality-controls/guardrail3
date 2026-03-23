import type { ButtonHTMLAttributes, PropsWithChildren } from "react";
import { joinClasses } from "@/lib/utils";

type ButtonProps = PropsWithChildren<ButtonHTMLAttributes<HTMLButtonElement>> & {
  tone?: "primary" | "secondary";
};

export function Button({
  children,
  className,
  tone = "primary",
  type = "button",
  ...props
}: ButtonProps): React.JSX.Element {
  return (
    <button
      type={type}
      className={joinClasses(
        "inline-flex items-center justify-center rounded-full px-4 py-2 text-sm font-medium transition-colors",
        tone === "primary" ? "bg-slate-950 text-white" : "bg-white text-slate-950 ring-1 ring-slate-300",
        className,
      )}
      {...props}
    >
      {children}
    </button>
  );
}
