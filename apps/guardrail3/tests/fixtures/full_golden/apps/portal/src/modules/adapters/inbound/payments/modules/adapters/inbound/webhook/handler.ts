import { StripeClient } from "@/modules/adapters/inbound/payments/modules/adapters/outbound/stripe-client/client";
import { chargeCard } from "@/modules/adapters/inbound/payments/modules/application/commands/charge";

type WebhookPayload = {
  amountCents: number;
  customerEmail: string;
};

export async function handleWebhook(payload: WebhookPayload) {
  const stripe = new StripeClient();
  return chargeCard(stripe, payload.amountCents, payload.customerEmail);
}
