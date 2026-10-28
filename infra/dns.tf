resource "cloudflare_record" "gemini_cname" {
  zone_id = data.cloudflare_zone.lark.id
  type    = "CNAME"
  name    = "gemini"
  content = "lark-gemini-capsule.fly.dev"
  proxied = false
}

resource "cloudflare_record" "matrix_cname" {
  zone_id = data.cloudflare_zone.lark.id
  type    = "CNAME"
  name    = "matrix"
  content = "lark-tuwunel-matrix.fly.dev"
  proxied = false
}

resource "cloudflare_record" "apex_mx" {
  for_each = {
    route1 = {
      content  = "eu1-smtp.messagingengine.com"
      priority = 10
    }
    route2 = {
      content  = "eu2-smtp.messagingengine.com"
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
  content = "v=spf1 include:spf.messagingengine.com ?all"
  proxied = false
}

resource "cloudflare_record" "apex_cname_dkim" {
  for_each = {
    protonmail1 = {
      name    = "fm1._domainkey"
      content = "fm1.lark.gay.dkim.fmhosted.com"
    }

    protonmail2 = {
      name    = "fm2._domainkey"
      content = "fm2.lark.gay.dkim.fmhosted.com"
    }

    protonmail3 = {
      name    = "fm3._domainkey"
      content = "fm3.lark.gay.dkim.fmhosted.com"
    }
  }

  zone_id = data.cloudflare_zone.lark.id
  type    = "CNAME"
  name    = each.value.name
  content = each.value.content
  proxied = false
}

resource "cloudflare_record" "apex_txt_atproto" {
  zone_id = data.cloudflare_zone.lark.id
  type    = "TXT"
  name    = "_atproto"
  content = "did=did:plc:dpkvjlvuoz7d3h2an2uhfci7"
}

resource "cloudflare_record" "floof_apex_cname" {
  zone_id = data.cloudflare_zone.floof.id
  type    = "CNAME"
  name    = "@"
  content = "floof-tuwunel-matrix.fly.dev"
  proxied = false
}

resource "cloudflare_record" "floof_xmpp_cname" {
  zone_id = data.cloudflare_zone.floof.id
  type    = "CNAME"
  name    = "xmpp"
  content = "floof-ejabberd-xmpp.fly.dev"
  proxied = false
}

resource "cloudflare_record" "floof_xmpp_client_srv" {
  zone_id = data.cloudflare_zone.floof.id
  type    = "SRV"
  name    = "_xmpp-client._tcp"

  data {
    service  = "_xmpp-client"
    proto    = "_tcp"
    name     = "floof.chat"
    priority = 0
    weight   = 5
    port     = 5222
    target   = "xmpp.floof.chat"
  }
}

resource "cloudflare_record" "floof_xmpp_server_srv" {
  zone_id = data.cloudflare_zone.floof.id
  type    = "SRV"
  name    = "_xmpp-server._tcp"

  data {
    service  = "_xmpp-server"
    proto    = "_tcp"
    name     = "floof.chat"
    priority = 0
    weight   = 5
    port     = 5269
    target   = "xmpp.floof.chat"
  }
}

resource "cloudflare_record" "sneakerparty_apex_cname" {
  zone_id = data.cloudflare_zone.sneakerparty.id
  type    = "CNAME"
  name    = "@"
  content = "sneakerparty-gathio.fly.dev"
  proxied = false
}

resource "cloudflare_record" "sneakerparty_apex_txt_mxroute_verification" {
  zone_id = data.cloudflare_zone.sneakerparty.id
  type    = "TXT"
  name    = "_da-verify-c3fc95dea613673d3daed256d7f2616c"
  content = "domain-verified"
  proxied = false
}

resource "cloudflare_record" "sneakerparty_apex_mx" {
  for_each = {
    route1 = {
      content  = "witcher.mxrouting.net"
      priority = 10
    }
    route2 = {
      content  = "witcher-relay.mxrouting.net"
      priority = 20
    }
  }

  zone_id  = data.cloudflare_zone.sneakerparty.id
  type     = "MX"
  name     = "@"
  content  = each.value.content
  priority = each.value.priority
  proxied  = false
}

resource "cloudflare_record" "sneakerparty_apex_txt_spf" {
  zone_id = data.cloudflare_zone.sneakerparty.id
  type    = "TXT"
  name    = "@"
  content = "v=spf1 include:mxroute.com -all"
  proxied = false
}

resource "cloudflare_record" "sneakerparty_apex_txt_dmarc" {
  zone_id = data.cloudflare_zone.sneakerparty.id
  type    = "TXT"
  name    = "x._domainkey"
  content = "v=DKIM1; k=rsa; p=MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEApoLdaWZuNonT08voVLddSs7npIJoumPAyyVVe6ix1DPaAvuwpVrECaQYjgqS3XctEa3MzSrKcK74MyQNOktRjUgU+jhHq79fNnqLbU8V9hA6CXBwp3ibVC9yY15olGwKGPLBeHW3s+zZzJbw7gj/wvjdDI9H037gEHYOgL1ED8ezXYAIqa/YLsMIY9BvTtadRvP7aR3UmACXr0+oSxpKOKIRKeinHqQhmqjf6+zDkki48mDXYJ3Xh/ClZ0cueh/6wPpVshpFCOaVAN2ENUjycd/87rheWFkEUGzeOvfdDRjNM3v42a72Dg5lVOGawn2HdCJYL0C2OtDGJeTTyXgoKQIDAQAB"
  proxied = false
}
