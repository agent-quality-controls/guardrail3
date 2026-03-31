import type { ChatDraft } from "@/modules/adapters/outbound/ai-chat/modules/domain/types";
import type { LlmPort } from "@/modules/adapters/outbound/ai-chat/modules/ports/outbound/llm";

export async function sendMessage(llm: LlmPort, draft: ChatDraft): Promise<string> {
  return llm.complete(draft);
}
