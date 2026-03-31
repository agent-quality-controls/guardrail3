import type { ValidationSummary } from "@domain/types";

export interface ValidationDashboardUseCases {
  validateLive(locale: string, previewMode: boolean): Promise<ValidationSummary>;
  validateSpecs(locale: string, previewMode: boolean): Promise<ValidationSummary>;
}
