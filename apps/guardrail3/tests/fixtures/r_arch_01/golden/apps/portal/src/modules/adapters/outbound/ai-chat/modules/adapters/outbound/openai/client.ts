import type { ChatDraft } from "@/modules/adapters/outbound/ai-chat/modules/domain/types";
import type { LlmPort } from "@/modules/adapters/outbound/ai-chat/modules/ports/outbound/llm";

export class OpenAiClient implements LlmPort {
  async complete(draft: ChatDraft): Promise<string> {
    const latest = draft.messages[draft.messages.length - 1];
    return `Draft reply for ${draft.tenantSlug}: ${latest?.content ?? "no message"}`;
  }
}
