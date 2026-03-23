export type AuditTarget = "live" | "spec";

export interface ValidationSummary {
  target: AuditTarget;
  checkedAt: string;
  passed: number;
  failed: number;
  warnings: number;
  serviceMode: "online" | "degraded" | "cached";
  queueLagSeconds: number;
}

export interface ValidationRunRequest {
  target: AuditTarget;
  locale: string;
  previewMode: boolean;
}
