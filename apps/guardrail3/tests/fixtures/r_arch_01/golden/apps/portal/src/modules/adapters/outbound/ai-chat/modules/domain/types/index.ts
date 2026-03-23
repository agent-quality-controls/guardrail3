export type ChatRole = "user" | "assistant" | "system";

export type Message = {
  role: ChatRole;
  content: string;
};

export type ChatDraft = {
  conversationId: string;
  tenantSlug: string;
  messages: Message[];
  tone: "direct" | "friendly";
};
