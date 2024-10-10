resource "cloudflare_ruleset" "cache_settings" {
  zone_id     = data.cloudflare_zone.lark.id
  name        = "set cache settings"
  description = "set cache settings for the request"
  kind        = "zone"
  phase       = "http_request_cache_settings"

  rules {
    action = "set_cache_settings"

    action_parameters {
      browser_ttl {
        mode = "respect_origin"
      }
    }

    expression  = "(http.host eq \"${data.cloudflare_zone.lark.name}\")"
    description = "set browser cache TTL to respect origin headers"
    enabled     = true
  }
}
