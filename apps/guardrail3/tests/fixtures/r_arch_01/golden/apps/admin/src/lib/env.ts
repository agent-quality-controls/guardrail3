export interface AdminEnv {
  validatorBaseUrl: string;
  previewMode: boolean;
}

export function readAdminEnv(): AdminEnv {
  return {
    validatorBaseUrl: process.env.NEXT_PUBLIC_VALIDATOR_URL ?? "http://localhost:4100",
    previewMode: process.env.NEXT_PUBLIC_PREVIEW_MODE === "true",
  };
}
