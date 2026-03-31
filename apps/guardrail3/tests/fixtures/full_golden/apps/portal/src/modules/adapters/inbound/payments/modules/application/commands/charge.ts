import type { Charge } from "@/modules/adapters/inbound/payments/modules/domain/types";
import type { StripePort } from "@/modules/adapters/inbound/payments/modules/ports/outbound/stripe";

export async function chargeCard(
  stripe: StripePort,
  amountCents: number,
  customerEmail: string,
): Promise<Charge> {
  const stripeCharge = await stripe.createCharge(amountCents, customerEmail);

  return {
    id: `charge_${stripeCharge.stripeId}`,
    stripeId: stripeCharge.stripeId,
    customerEmail,
    amountCents,
    currency: "USD",
    status: stripeCharge.status,
  };
}
