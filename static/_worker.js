const MATRIX_PROXY_PREFIXES = ["/_matrix/", "/_conduwuit/"];
const NOTIFICATIONS_PREFIX = "/notifications/";
const MATRIX_PROXY_DOMAIN = "matrix.lark.gay";

export default {
  async fetch(request, env) {
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
      const tokenIsValid = crypto.subtle.timingSafeEqual(
        url.searchParams.get("token"),
        env.NOTIFICATIONS_TOKEN,
      );

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

    return env.ASSETS.fetch(request);
  },
};
