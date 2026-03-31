import { cn } from "@/lib/utils";

type ButtonProps = {
  children: React.ReactNode;
  variant?: "primary" | "ghost";
};

export function Button({ children, variant = "primary" }: ButtonProps): React.JSX.Element {
  return (
    <button
      className={cn(
        "inline-flex items-center justify-center rounded-full px-4 py-2 text-sm font-medium transition",
        variant === "primary" && "bg-emerald-400 text-slate-950",
        variant === "ghost" && "border border-slate-700 text-slate-100",
      )}
    >
      {children}
    </button>
  );
}
