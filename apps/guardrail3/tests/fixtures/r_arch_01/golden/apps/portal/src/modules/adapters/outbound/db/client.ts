import type { Order } from "@/modules/domain/types";

const seededOrders: Record<string, Order> = {
  "ord_1001": {
    id: "ord_1001",
    customerEmail: "ops@acme.test",
    currency: "USD",
    status: "awaiting-payment",
    fraudReviewRequired: false,
    lines: [
      { sku: "team-plan", title: "Team plan", quantity: 1, unitPriceCents: 24000 },
      { sku: "priority-onboarding", title: "Priority onboarding", quantity: 1, unitPriceCents: 8000 },
    ],
  },
  "ord_1002": {
    id: "ord_1002",
    customerEmail: "finance@beta.test",
    currency: "USD",
    status: "flagged",
    fraudReviewRequired: true,
    lines: [{ sku: "enterprise-seat", title: "Enterprise seat", quantity: 8, unitPriceCents: 12000 }],
  },
};

export function createDbClient(): { getOrder(orderId: string): Promise<Order | null> } {
  return {
    async getOrder(orderId: string): Promise<Order | null> {
      return seededOrders[orderId] ?? null;
    },
  };
}
