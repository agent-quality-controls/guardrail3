export default function RootLayout({ children }: { children: React.ReactNode }): React.JSX.Element {
  return (
    <html lang="en">
      <body className="min-h-screen bg-slate-950 text-slate-100">{children}</body>
    </html>
  );
}
