export interface LlmPort { complete(prompt: string): Promise<string>; }
