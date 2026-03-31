import { Footer } from "@/components/landing/footer";
import { Hero } from "@/components/landing/hero";
import { formatCount } from "@/lib/utils";

const quickWins = [
  { title: "Reset bedtime in under a week", families: 1280 },
  { title: "Reduce after-school chaos", families: 940 },
  { title: "Build a realistic Sunday planning ritual", families: 1535 },
];

export default function HomePage(): React.JSX.Element {
  return (
    <main className="mx-auto flex min-h-screen max-w-6xl flex-col gap-10 px-6 py-12">
      <Hero />
      <section className="grid gap-4 md:grid-cols-3">
        {quickWins.map((item) => (
          <article key={item.title} className="rounded-2xl border border-slate-200 bg-white p-5">
            <p className="text-sm font-medium text-slate-500">{formatCount(item.families)} families saved this</p>
            <h2 className="mt-2 text-xl font-semibold text-slate-950">{item.title}</h2>
          </article>
        ))}
      </section>
      <Footer />
    </main>
  );
}
