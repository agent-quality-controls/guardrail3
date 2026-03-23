export type OrderStatus = "draft" | "awaiting-payment" | "paid" | "flagged";

export type OrderLine = {
  sku: string;
  title: string;
  quantity: number;
  unitPriceCents: number;
};

export type Order = {
  id: string;
  customerEmail: string;
  currency: "USD" | "EUR";
  status: OrderStatus;
  lines: OrderLine[];
  fraudReviewRequired: boolean;
};

export type CheckoutSummary = {
  subtotalCents: number;
  serviceFeeCents: number;
  totalCents: number;
};
