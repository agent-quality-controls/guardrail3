import type { Charge } from "@/modules/adapters/inbound/payments/modules/domain/types";

export interface StripePort {
  createCharge(amountCents: number, customerEmail: string): Promise<Pick<Charge, "stripeId" | "status">>;
}
