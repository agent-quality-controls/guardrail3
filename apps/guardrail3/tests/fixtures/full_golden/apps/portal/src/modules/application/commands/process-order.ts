import type { CheckoutSummary, Order } from "@/modules/domain/types";
import type { PaymentGateway } from "@/modules/ports/outbound/gateway";

function summarizeOrder(order: Order): CheckoutSummary {
  const subtotalCents = order.lines.reduce(
    (sum, line) => sum + line.quantity * line.unitPriceCents,
    0,
  );
  const serviceFeeCents = order.fraudReviewRequired ? 1800 : 700;

  return {
    subtotalCents,
    serviceFeeCents,
    totalCents: subtotalCents + serviceFeeCents,
  };
}

export async function processOrder(
  gateway: PaymentGateway,
  order: Order,
): Promise<{ order: Order; summary: CheckoutSummary; intent: Awaited<ReturnType<PaymentGateway["createIntent"]>> }> {
  const summary = summarizeOrder(order);
  const intent = await gateway.createIntent(order, summary);

  return { order, summary, intent };
}
