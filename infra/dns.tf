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

resource "cloudflare_record" "apex_mx" {
  for_each = {
    route1 = {
      content  = "mx1.simplelogin.co."
      priority = 10
    }
    route2 = {
      content  = "mx2.simplelogin.co."
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
  content = "v=spf1 include:_spf.mailersend.net include:simplelogin.co ~all"
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

    mailersend = {
      name    = "mlsend2._domainkey"
      content = "mlsend2._domainkey.mailersend.net"
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

resource "cloudflare_record" "mta_cname" {
  zone_id = data.cloudflare_zone.lark.id
  type    = "CNAME"
  name    = "mta"
  content = "mailersend.net"
  proxied = false
}
