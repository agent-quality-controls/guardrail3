export interface StripePort { createCharge(amount: number): Promise<string>; }
