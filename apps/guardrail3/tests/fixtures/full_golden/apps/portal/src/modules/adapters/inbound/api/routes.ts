import { createDbClient } from "@/modules/adapters/outbound/db/client";
import { processOrder } from "@/modules/application/commands/process-order";
import type { OrderUseCase } from "@/modules/ports/inbound/use-cases";
import type { PaymentGateway } from "@/modules/ports/outbound/gateway";

class PortalPaymentGateway implements PaymentGateway {
  async createIntent(order: Parameters<typeof processOrder>[1], summary: ReturnType<typeof buildSummaryPreview>) {
    return {
      providerReference: `pay_${order.id}`,
      status: summary.totalCents > 50000 ? "action-required" : "authorized",
    } as const;
  }
}

function buildSummaryPreview(order: Parameters<typeof processOrder>[1]) {
  const subtotalCents = order.lines.reduce((sum, line) => sum + line.quantity * line.unitPriceCents, 0);
  const serviceFeeCents = order.fraudReviewRequired ? 1800 : 700;
  return { subtotalCents, serviceFeeCents, totalCents: subtotalCents + serviceFeeCents };
}

export function createRouter(): OrderUseCase {
  const db = createDbClient();
  const gateway = new PortalPaymentGateway();

  return {
    async process(orderId) {
      const order = await db.getOrder(orderId);
      if (!order) {
        throw new Error(`missing order ${orderId}`);
      }

      return processOrder(gateway, order);
    },
  };
}
