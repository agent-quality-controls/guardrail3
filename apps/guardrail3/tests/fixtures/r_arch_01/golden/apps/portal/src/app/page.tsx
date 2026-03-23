import { Button } from "@/components/ui/button";
import { formatCurrency } from "@/lib/utils";
import { createRouter } from "@/modules/adapters/inbound/api/routes";
import { handleWebhook } from "@/modules/adapters/inbound/payments/modules/adapters/inbound/webhook/handler";
import { OpenAiClient } from "@/modules/adapters/outbound/ai-chat/modules/adapters/outbound/openai/client";
import { sendMessage } from "@/modules/adapters/outbound/ai-chat/modules/application/commands/send-message";

export default async function Home(): Promise<React.JSX.Element> {
  const router = createRouter();
  const checkout = await router.process("ord_1001");
  const charge = await handleWebhook({
    amountCents: checkout.summary.totalCents,
    customerEmail: checkout.order.customerEmail,
  });
  const assistantReply = await sendMessage(new OpenAiClient(), {
    conversationId: "conv_1",
    tenantSlug: "acme",
    tone: "direct",
    messages: [{ role: "user", content: "Customer wants the invoice PDF and renewal date." }],
  });

  return (
    <main className="mx-auto grid max-w-5xl gap-6 px-6 py-10 lg:grid-cols-[1.3fr_0.7fr]">
      <section className="rounded-3xl border border-slate-800 bg-slate-900 p-8">
        <p className="text-xs uppercase tracking-[0.24em] text-emerald-300">Portal workspace</p>
        <h1 className="mt-3 text-4xl font-semibold">Checkout, support, and payment flows in one clean fixture.</h1>
        <p className="mt-4 max-w-2xl text-sm text-slate-300">
          This app mixes order processing, payment webhooks, and AI-assisted support drafts so later tests can mutate a
          realistic TS codebase instead of toy snippets.
        </p>
        <div className="mt-6 flex gap-3">
          <Button>Review order flow</Button>
          <Button variant="ghost">Inspect adapters</Button>
        </div>
      </section>

      <section className="rounded-3xl border border-slate-800 bg-slate-900 p-6">
        <p className="text-xs uppercase tracking-[0.24em] text-slate-400">Current checkout</p>
        <h2 className="mt-2 text-2xl font-semibold">{checkout.order.id}</h2>
        <p className="mt-2 text-sm text-slate-300">
          Total due {formatCurrency(checkout.summary.totalCents, checkout.order.currency)}
        </p>
        <p className="mt-2 text-sm text-slate-400">
          Provider status {checkout.intent.status}, webhook charge {charge.status}
        </p>
        <p className="mt-4 rounded-2xl bg-slate-950 p-4 text-sm text-slate-300">{assistantReply}</p>
      </section>
    </main>
  );
}
