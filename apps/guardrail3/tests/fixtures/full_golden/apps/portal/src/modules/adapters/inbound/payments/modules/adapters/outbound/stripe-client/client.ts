import type { StripePort } from "@/modules/adapters/inbound/payments/modules/ports/outbound/stripe";

export class StripeClient implements StripePort {
  async createCharge(amountCents: number, customerEmail: string) {
    const requiresAction = amountCents >= 50000 || customerEmail.endsWith("@finance.test");

    return {
      stripeId: `ch_${amountCents}`,
      status: requiresAction ? "requires-action" : "succeeded",
    } as const;
  }
}
