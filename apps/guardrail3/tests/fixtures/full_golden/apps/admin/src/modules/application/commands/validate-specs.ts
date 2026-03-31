import type { ValidationSummary } from "@domain/types";
import type { ValidatorGateway } from "@/modules/ports/outbound/validator";

export async function validateSpecContent(
  gateway: ValidatorGateway,
  locale: string,
  previewMode: boolean,
): Promise<ValidationSummary> {
  return gateway.runValidation({ target: "spec", locale, previewMode });
}
