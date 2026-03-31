import type { Charge } from "@/modules/adapters/inbound/payments/modules/domain/types";

export interface ChargeUseCase {
  charge(amountCents: number, customerEmail: string): Promise<Charge>;
}
