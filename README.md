# Lark Space

This is my personal website and blog. It's a static site built with
[Zola](https://www.getzola.org/) and hosted on
[Cloudflare Workers](https://developers.cloudflare.com/workers/).

See the [Colophon](https://lark.gay/colophon/) for more information about the
technical decision that went into building this site.

## Blog

The comments section for my blog is provided by
[Hyvor Talk](https://talk.hyvor.com/).

## Matrix

I self-host a [Matrix](https://matrix.org/) homeserver
([tuwunel](https://tuwunel.chat/)) on [Fly.io](https://fly.io/) at `lark.gay`.
Supporting both this site and the homeserver on the same domain requires some
special configuration, which is worth documenting here.

This site serves static JSON files at the following endpoints:

- `/.well-known/matrix/server`
- `/.well-known/matrix/client`

These are JSON files used to configure _delegation_, a feature of Matrix that
allows for the federation API endpoint to be served at a different domain and/or
port than the public-facing server domain. This means that my Matrix username
(`@lark:lark.gay`) case use the base domain, even though the homeserver is
hosted at `matrix.lark.gay`.

For delegation to work, it is also necessary to reverse-proxy traffic to the
following endpoints:

- `/_matrix/*`
- `/_tuwunel/*`
