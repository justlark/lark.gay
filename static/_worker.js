const PROXY_PREFIXES = ["/_matrix/", "/_conduwuit/"];
const PROXY_DOMAIN = "matrix.lark.gay";

export default {
  async fetch(request, env) {
    const url = new URL(request.url);

    if (PROXY_PREFIXES.some((prefix) => url.pathname.startsWith(prefix))) {
      if (url.pathname.startsWith(prefix)) {
        return await fetch(`https://${PROXY_DOMAIN}${url.pathname}`, {
          method: request.method,
          headers: request.headers,
          body: request.body,
        });
      }
    }

    return env.ASSETS.fetch(request);
  },
};
