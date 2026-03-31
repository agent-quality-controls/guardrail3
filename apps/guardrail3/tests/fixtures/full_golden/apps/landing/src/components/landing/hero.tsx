import { Button } from "@/components/ui/button";
import {
  COMMUNITY_STATS,
  SITE_NAME,
  SITE_TAGLINE,
  SPOTLIGHT_COURSES,
  TRUST_SIGNALS,
} from "@/lib/site-config";

export function Hero(): React.JSX.Element {
  return (
    <section className="space-y-8 rounded-3xl border border-slate-200 bg-white p-10 shadow-sm">
      <div className="space-y-3">
        <p className="text-sm font-semibold uppercase tracking-[0.24em] text-slate-500">Family systems</p>
        <h1 className="max-w-3xl text-4xl font-semibold tracking-tight text-slate-950">{SITE_NAME}</h1>
        <p className="max-w-2xl text-lg text-slate-600">{SITE_TAGLINE}</p>
        <div className="flex flex-wrap gap-3 pt-2">
          <Button>Start with the family reset</Button>
          <Button tone="secondary">Browse guides</Button>
        </div>
      </div>
      <div className="grid gap-4 md:grid-cols-3">
        {COMMUNITY_STATS.map((item) => (
          <div key={item.label} className="rounded-2xl bg-slate-50 p-4">
            <p className="text-2xl font-semibold text-slate-950">{item.value}</p>
            <p className="text-sm text-slate-600">{item.label}</p>
          </div>
        ))}
      </div>
      <div className="grid gap-4 md:grid-cols-2">
        {SPOTLIGHT_COURSES.map((course) => (
          <article key={course.slug} className="rounded-2xl border border-slate-200 p-5">
            <h2 className="text-xl font-semibold text-slate-950">{course.title}</h2>
            <p className="mt-2 text-sm leading-6 text-slate-600">{course.summary}</p>
            <p className="mt-3 text-sm text-slate-500">{course.audience}</p>
            <p className="mt-4 text-sm font-medium text-slate-900">{course.ctaLabel}</p>
          </article>
        ))}
      </div>
      <ul className="grid gap-3 md:grid-cols-3">
        {TRUST_SIGNALS.map((item) => (
          <li key={item} className="rounded-2xl bg-slate-950 px-4 py-3 text-sm text-slate-100">
            {item}
          </li>
        ))}
      </ul>
    </section>
  );
}
