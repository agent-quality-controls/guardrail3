import type { ReactNode } from "react";

export default function RootLayout({ children }: { children: ReactNode }): React.JSX.Element {
  return (
    <html lang="en">
      <body className="bg-slate-950 text-slate-50">
        <div className="mx-auto flex min-h-screen max-w-6xl flex-col gap-6 px-6 py-8">
          <header className="flex items-center justify-between border-b border-slate-800 pb-4">
            <div>
              <p className="text-xs uppercase tracking-[0.24em] text-slate-400">Operations</p>
              <h1 className="text-2xl font-semibold">Validation Admin</h1>
            </div>
            <p className="text-sm text-slate-400">Live + spec review dashboard</p>
          </header>
          {children}
        </div>
      </body>
    </html>
  );
}
