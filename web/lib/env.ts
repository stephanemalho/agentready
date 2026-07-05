import "server-only";

export function apiUrl(): string {
  const url = process.env.API_URL;
  if (!url) {
    throw new Error(
      "API_URL is not set: point it to the AgentReady API server (e.g. http://localhost:8080)",
    );
  }
  return url.replace(/\/+$/, "");
}
