export type Charge = {
  id: string;
  stripeId: string;
  customerEmail: string;
  amountCents: number;
  currency: "USD" | "EUR";
  status: "queued" | "succeeded" | "requires-action";
};
