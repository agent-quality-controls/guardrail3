const posts = [
  {
    slug: "reset-the-bedroom-routine",
    title: "Reset the bedroom routine in three nights",
    excerpt: "Use one predictable script, one visual cue, and one fallback plan.",
  },
  {
    slug: "family-planning-sprint",
    title: "Run a Sunday family planning sprint",
    excerpt: "A short planning ritual that keeps logistics from turning into conflict.",
  },
];

export default function BlogPage(): React.JSX.Element {
  return (
    <main className="mx-auto max-w-4xl space-y-6 px-6 py-10">
      <h1 className="text-3xl font-semibold tracking-tight text-slate-950">Family systems blog</h1>
      <div className="grid gap-4">
        {posts.map((post) => (
          <article key={post.slug} className="rounded-2xl border border-slate-200 bg-white p-5">
            <h2 className="text-xl font-medium text-slate-950">{post.title}</h2>
            <p className="mt-2 text-sm leading-6 text-slate-600">{post.excerpt}</p>
          </article>
        ))}
      </div>
    </main>
  );
}
