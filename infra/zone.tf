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
    security_header {
      enabled            = true
      preload            = true
      max_age            = 31536000
      include_subdomains = true
      nosniff            = true
    }
  }
}

data "cloudflare_zone" "floof" {
  name       = "floof.chat"
  account_id = var.cloudflare_account_id
}

resource "cloudflare_zone_dnssec" "floof" {
  zone_id = data.cloudflare_zone.floof.id
}

resource "cloudflare_zone_settings_override" "floof" {
  zone_id = data.cloudflare_zone.floof.id

  settings {
    security_header {
      enabled            = true
      preload            = true
      max_age            = 31536000
      include_subdomains = true
      nosniff            = true
    }
  }
}

