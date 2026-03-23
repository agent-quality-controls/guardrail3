import { readAdminEnv } from "@/lib/env";
import { Button } from "@/components/ui/button";
import { ValidatorHttpClient } from "@/modules/adapters/outbound/validator/client";
import { validateLiveContent } from "@/modules/application/commands/validate-live";
import { validateSpecContent } from "@/modules/application/commands/validate-specs";

export default async function RootPage(): Promise<React.JSX.Element> {
  const env = readAdminEnv();
  const client = new ValidatorHttpClient(env.validatorBaseUrl);
  const [live, spec] = await Promise.all([
    validateLiveContent(client, "en", env.previewMode),
    validateSpecContent(client, "en", env.previewMode),
  ]);

  return (
    <main className="grid gap-4 md:grid-cols-2">
      {[live, spec].map((summary) => (
        <section key={summary.target} className="rounded-2xl border border-slate-800 bg-slate-900 p-5">
          <div className="flex items-start justify-between gap-4">
            <div>
              <p className="text-xs uppercase tracking-[0.24em] text-slate-400">{summary.target}</p>
              <p className="mt-1 text-xs text-slate-500">
                {summary.serviceMode} mode, queue lag {summary.queueLagSeconds}s
              </p>
            </div>
            <Button variant="ghost">Open report</Button>
          </div>
          <h2 className="mt-2 text-xl font-semibold">{summary.passed} checks passing</h2>
          <p className="mt-1 text-sm text-slate-300">{summary.failed} failing checks need review.</p>
          <p className="mt-1 text-sm text-amber-300">{summary.warnings} warnings still need triage.</p>
          <p className="mt-4 text-xs text-slate-500">Snapshot: {summary.checkedAt}</p>
        </section>
      ))}
    </main>
  );
}
