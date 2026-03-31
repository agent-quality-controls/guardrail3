import type { CheckoutSummary, Order } from "@/modules/domain/types";
import type { PaymentIntent } from "@/modules/ports/outbound/gateway";

export interface OrderUseCase {
  process(orderId: string): Promise<{ order: Order; summary: CheckoutSummary; intent: PaymentIntent }>;
}
