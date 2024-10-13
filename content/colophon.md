+++
title = "Colophon"
description = "About the making of this site"
template = "colophon.html"
date = 2024-10-12
+++

## Colophon

This is a static site built with [Zola](https://www.getzola.org/).

### Philosophy

I'm tired of complex, bloated websites. My goal for this site is to build
something simple, hackable, and fast.

I want to make it easy for folks to read the code for this site, learn from it,
and adapt it for their own needs. One of the principles of the indieweb is
making your own websites, and I'm a big fan of lowering the barrier to entry.
Sites that are obfuscated by complex build pipelines, transpiled languages, and
minification make it harder to understand and adapt.

I also want this to be a site *for everyone*. In addition to typical web
accessibility considerations, I want this site to be light enough to load
quickly on a range of devices; the median web user does not use a $1000
smartphone on a 5G connection. I also want to respect users who prefer to keep
JavaScript disabled, so any use of JavaScript should be minimal and
**optional**.

To avoid complex build pipelines and improve hackability, this site does not use
Sass, CSS frameworks, or TypeScript, it doesn't have any JS dependencies, and it
doesn't do any bundling or minifying. Those are all useful tools that I would
happily reach for on a larger project, but they're not necessary for a site of
this complexity.

The privacy of my users is also important to me, so I don't use analytics or
tracking scripts on this site. I use [Hyvor Talk](https://talk.hyvor.com/) for
comments, which is a privacy-focused alternative to services like Disqus, and I
have IP logging disabled.

### Features

Here are some of the cool and less-obvious features of this site:

- [Atom](https://validator.w3.org/feed/docs/atom.html) for web feeds (I use the
  term "RSS" on the site because it's more widely recognized). I love web feeds
  and wish more people used them.
- [Open Graph](https://ogp.me/) and [Twitter
  Card](https://developer.x.com/en/docs/x-for-websites/cards/overview/abouts-cards)
  metadata for rich previews when sharing links.
- [JSON-LD](https://json-ld.org/) to provide machine-readable metadata.
- [Theme picker](/#theme-picker) for changing the site's color scheme. The site
  is built using the standard 16-color ANSI palette, so it's easy to add pretty
  much any standard terminal color scheme.
