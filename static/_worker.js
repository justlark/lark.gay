const MATRIX_PROXY_PREFIXES = ["/_matrix/", "/_conduwuit/"];
const MATRIX_PROXY_DOMAIN = "matrix.lark.gay";

export default {
  async fetch(request, env) {
    const url = new URL(request.url);

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

    return env.ASSETS.fetch(request);
  },
};
