import type { CheckoutSummary, Order } from "@/modules/domain/types";

export type PaymentIntent = {
  providerReference: string;
  status: "authorized" | "action-required";
};

export interface PaymentGateway {
  createIntent(order: Order, summary: CheckoutSummary): Promise<PaymentIntent>;
}
