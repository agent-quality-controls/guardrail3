type HandlerResult = {
  status: number;
  body: Record<string, string | number | boolean>;
};

export async function withRouteHandling(
  handler: () => Promise<HandlerResult>,
): Promise<HandlerResult> {
  try {
    return await handler();
  } catch (error) {
    return {
      status: 500,
      body: {
        error: "validation-dashboard-failed",
        message: error instanceof Error ? error.message : "unknown error",
      },
    };
  }
}
