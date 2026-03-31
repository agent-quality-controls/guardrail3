import type { ReactNode } from "react";

export default function RootLayout({ children }: { children: ReactNode }): React.JSX.Element {
  return (
    <html lang="en">
      <body className="bg-slate-100 text-slate-950">{children}</body>
    </html>
  );
}
