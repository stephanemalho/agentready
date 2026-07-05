export const scanServiceUnreachableMessage =
  "The scan service is unreachable. Try again shortly.";

const missingApiUrlPrefix = "API_URL is not set";

export function scanConfigurationErrorMessage(error: unknown): string | null {
  if (!(error instanceof Error)) {
    return null;
  }

  if (!error.message.startsWith(missingApiUrlPrefix)) {
    return null;
  }

  return "The scan service is not configured. Restart the web dev server with API_URL=http://localhost:8080.";
}
