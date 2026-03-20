export interface OrderUseCase { process(id: string): Promise<void>; }
