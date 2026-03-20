export interface PaymentGateway { charge(amount: number): Promise<boolean>; }
