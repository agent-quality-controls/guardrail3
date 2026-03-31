import type { ValidationRunRequest, ValidationSummary } from "@domain/types";

export interface ValidatorGateway {
  runValidation(request: ValidationRunRequest): Promise<ValidationSummary>;
}
