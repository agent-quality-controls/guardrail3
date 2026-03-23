import type { ChatDraft } from "@/modules/adapters/outbound/ai-chat/modules/domain/types";

export interface LlmPort {
  complete(draft: ChatDraft): Promise<string>;
}
