data "cloudflare_zone" "lark" {
  name       = "lark.gay"
  account_id = var.cloudflare_account_id
}

resource "cloudflare_zone_dnssec" "lark" {
  zone_id = data.cloudflare_zone.lark.id
}

resource "cloudflare_zone_settings_override" "lark" {
  zone_id = data.cloudflare_zone.lark.id

  settings {
    always_use_https         = "on"
    automatic_https_rewrites = "on"
    brotli                   = "on"
    browser_check            = "on"
    early_hints              = "on"
    email_obfuscation        = "on"
    hotlink_protection       = "off"
    http3                    = "on"
    ip_geolocation           = "on"
    ipv6                     = "on"
    ssl                      = "strict"
    security_level           = "medium"
    min_tls_version          = "1.2"
    tls_1_3                  = "zrt"
    zero_rtt                 = "on"

    security_header {
      enabled            = true
      preload            = true
      max_age            = 31536000
      include_subdomains = true
      nosniff            = true
    }
  }
}

