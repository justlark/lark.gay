resource "cloudflare_record" "matrix_cname" {
  zone_id = data.cloudflare_zone.lark.id
  type    = "CNAME"
  name    = "matrix"
  content = "lark-tuwunel-matrix.fly.dev"
  proxied = false
}

resource "cloudflare_record" "apex_txt_sl_verification" {
  zone_id = data.cloudflare_zone.lark.id
  type    = "TXT"
  name    = "@"
  content = "sl-verification=qxkovwigozbpdbtigaeljuaffiaaqi"
  proxied = false
}

resource "cloudflare_record" "apex_txt_protonmail_verification" {
  zone_id = data.cloudflare_zone.lark.id
  type    = "TXT"
  name    = "@"
  content = "protonmail-verification=8d508f554d747060d3d805eb7fbaaabb3551fe4f"
  proxied = false
}

resource "cloudflare_record" "apex_mx" {
  for_each = {
    route1 = {
      content  = "mail.protonmail.ch"
      priority = 10
    }
    route2 = {
      content  = "mailsec.protonmail.ch"
      priority = 20
    }
  }

  zone_id  = data.cloudflare_zone.lark.id
  type     = "MX"
  name     = "@"
  content  = each.value.content
  priority = each.value.priority
  proxied  = false
}

resource "cloudflare_record" "apex_txt_spf" {
  zone_id = data.cloudflare_zone.lark.id
  type    = "TXT"
  name    = "@"
  content = "v=spf1 include:simplelogin.co include:_spf.protonmail.ch ~all"
  proxied = false
}

resource "cloudflare_record" "apex_cname_dkim" {
  for_each = {
    record1 = {
      name    = "dkim._domainkey"
      content = "dkim._domainkey.simplelogin.co."
    }

    record2 = {
      name    = "dkim02._domainkey"
      content = "dkim02._domainkey.simplelogin.co."
    }

    record3 = {
      name    = "dkim03._domainkey"
      content = "dkim03._domainkey.simplelogin.co."
    }

    protonmail1 = {
      name    = "protonmail._domainkey"
      content = "protonmail.domainkey.dx653tub4jmdyqidjldhxy5d3ssr3ylm2ltxavg64vrdnpxwjdn3q.domains.proton.ch."
    }

    protonmail2 = {
      name    = "protonmail2._domainkey"
      content = "protonmail2.domainkey.dx653tub4jmdyqidjldhxy5d3ssr3ylm2ltxavg64vrdnpxwjdn3q.domains.proton.ch."
    }

    protonmail3 = {
      name    = "protonmail3._domainkey"
      content = "protonmail3.domainkey.dx653tub4jmdyqidjldhxy5d3ssr3ylm2ltxavg64vrdnpxwjdn3q.domains.proton.ch."
    }
  }

  zone_id = data.cloudflare_zone.lark.id
  type    = "CNAME"
  name    = each.value.name
  content = each.value.content
  proxied = false
}

resource "cloudflare_record" "apex_txt_dmarc" {
  zone_id = data.cloudflare_zone.lark.id
  type    = "TXT"
  name    = "_dmarc"
  content = "v=DMARC1; p=quarantine; pct=100; adkim=s; aspf=s"
  proxied = false
}

resource "cloudflare_record" "apex_txt_atproto" {
  zone_id = data.cloudflare_zone.lark.id
  type    = "TXT"
  name    = "_atproto"
  content = "did=did:plc:dpkvjlvuoz7d3h2an2uhfci7"
}
