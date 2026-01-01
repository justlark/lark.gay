const MATRIX_PROXY_PREFIXES = ["/_matrix/", "/_tuwunel/"];
const NOTIFICATIONS_PREFIX = "/notifications/";
const MATRIX_PROXY_DOMAIN = "matrix.lark.gay";

type HeaderPatterns = Record<string, Record<string, string>>;

const HEADERS: HeaderPatterns = {
  "/.*": {
    "X-Content-Type-Options": "nosniff",
    "X-Frame-Options": "DENY",
    "Content-Security-Policy":
      "default-src 'self'; script-src-elem 'self' https://talk.hyvor.com; connect-src 'self' wss: https://*.hyvor.com; img-src 'self' https://hyvor.com https://*.hyvor.com https://www.gravatar.com; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; font-src 'self' https://fonts.gstatic.com; frame-ancestors 'none';",
    "Referrer-Policy": "strict-origin",
    "Strict-Transport-Security": "max-age=31536000; includeSubDomains; preload",
    "Cache-Control": "no-cache",
  },
  "/assets/.+\\.(jpg|png|svg|webp)": {
    "Cache-Control": "public, max-age=14400",
  },
  "/\\.well-known/matrix/server": {
    "Content-Type": "application/json",
  },
  "/\\.well-known/matrix/client": {
    "Content-Type": "application/json",
  },
} as const;

interface Env {
  ASSETS: Fetcher;
  NOTIFICATIONS_TOKEN: string;
  DISCORD_WEBHOOK: string;
}

export default {
  async fetch(request: Request, env: Env) {
    const url = new URL(request.url);

    // Proxy API endpoints to the Matrix server.
    if (
      MATRIX_PROXY_PREFIXES.some((prefix) => url.pathname.startsWith(prefix))
    ) {
      const requestUrl = new URL(request.url);
      requestUrl.hostname = MATRIX_PROXY_DOMAIN;

      return await fetch(requestUrl, {
        method: request.method,
        headers: request.headers,
        body: request.body,
      });
    }

    // Send notifications to Discord.
    //
    // Make an API call like this to send "Hello World" to Lark's Discord:
    //
    //   POST https://lark.gay/notifications/?token=REDACTED&msg=Hello%20World
    if (url.pathname.startsWith(NOTIFICATIONS_PREFIX)) {
      const encoder = new TextEncoder();

      const expectedToken = encoder.encode(env.NOTIFICATIONS_TOKEN);
      const actualToken = encoder.encode(
        url.searchParams.get("token") ?? undefined,
      );

      let tokenIsValid = false;

      try {
        // This will throw an exception if the lengths are different.
        tokenIsValid = crypto.subtle.timingSafeEqual(
          actualToken,
          expectedToken,
        );
      } catch {
        // Redundant, but included for clarity.
        tokenIsValid = false;
      }

      if (!tokenIsValid) {
        return new Response("Invalid token", { status: 403 });
      }

      const message = url.searchParams.get("msg");

      return await fetch(env.DISCORD_WEBHOOK, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          content: message,
        }),
      });
    }

    const response = await env.ASSETS.fetch(request);
    const newResponse = new Response(response.body, response);

    for (const [pattern, patternHeaders] of Object.entries(HEADERS)) {
      if (new RegExp(pattern).test(url.pathname)) {
        for (const [key, value] of Object.entries(patternHeaders)) {
          newResponse.headers.set(key, value);
        }
      }
    }

    return newResponse;
  },
};
