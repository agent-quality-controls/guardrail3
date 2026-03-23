import type { ValidationRunRequest, ValidationSummary } from "@domain/types";
import type { ValidatorGateway } from "@/modules/ports/outbound/validator";

export class ValidatorHttpClient implements ValidatorGateway {
  constructor(private readonly baseUrl: string) {}

  async runValidation(request: ValidationRunRequest): Promise<ValidationSummary> {
    const degraded = this.baseUrl.includes("localhost") || request.previewMode;

    return {
      target: request.target,
      checkedAt: new Date("2026-03-01T09:00:00Z").toISOString(),
      passed: request.target === "live" ? 18 : 24,
      failed: request.target === "live" ? 2 : 1,
      warnings: degraded ? 4 : 1,
      serviceMode: degraded ? "degraded" : "online",
      queueLagSeconds: degraded ? 96 : 8,
    };
  }
}
