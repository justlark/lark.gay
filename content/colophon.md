+++
title = "Colophon"
description = "About the making of this site"
template = "colophon.html"
date = 2024-10-12
+++

## Philosophy

I'm tired of building complex, bloated websites. My goal for this site was to
build something simple, fast, accessible, hackable, and maintainable.

Here are some of the principles I followed when building this site:

1. **Make it hackable.** I want to make it easy for folks to read the code for
   this site, learn from it, and adapt it for their own needs. One of the
   principles of the indieweb is making your own websites, and I'm a big fan of
   lowering the barrier to entry. Sites that are obfuscated by complex build
   pipelines and minification make it harder to understand and adapt. Making the
   site [open-source](https://github.com/justlark/lark.gay) is also important
   for this. This principle informs many of the points below.
2. **Use minimal JavaScript, and make it optional.** Static sites shouldn't
   require JS to function. There are some good frontend frameworks out there
   (I'm a fan of [Vue](https://vuejs.org/)), but users shouldn't be forced to
   enable JS unless it's actually necessary. This site is perfectly functional
   without JS, and only misses minimal features like the [theme
   picker](/#theme-picker).
3. **No complex build pipelines.** I am so incredibly tired of the endless
   ecosystem of bundlers and transpilers and minifiers and tree shakers. I'm
   tired of trying to recite the correct incantations in my `tsconfig.json` to
   make the TypeScript compiler happy. I want something that just works, with
   minimal fuss. While I do regret that I can't use TypeScript, that's not as
   much of a problem when you keep it simple (see #2).
4. **No minification.** For a site of this complexity, it's really not necessary
   (see #1 and #3).
5. **No CSS frameworks.** I've used [Bootstrap](https://getbootstrap.com/) and
   [Tailwind](https://tailwindcss.com/) for sites in the past, and they're
   useful, but they also significantly increase the size of the client bundle,
   necessitating complex build pipelines (see #3) to strip out unused styles.
   There's also not a strong need to minify your CSS if you're only using
   precisely the styles you need (see #4).
6. **No Sass.** The static site generator I use supports
   [Sass](https://sass-lang.com/), but I've opted to use plain CSS instead for
   simplicity (see #1 and #3).
7. **No tracking.** I don't use analytics or tracking scripts on this site. I
   use [Hyvor Talk](https://talk.hyvor.com/) for comments, which is a
   privacy-focused alternative to Disqus, and I have IP logging disabled. I take
   your privacy seriously.

## Tools

Here are some of the tools I used to build this site:

- [Zola](https://www.getzola.org/) is the static site generator I use. It's
  somewhat like [Hugo](https://gohugo.io/), but much simpler.
- [Cloudflare Pages](https://developers.cloudflare.com/pages) is the hosting
  provider.
- [GitHub](https://github.com/justlark/lark.gay) is where the code is hosted.
- [Neovim](https://neovim.io/) is my editor of choice (if you couldn't tell).

## Features

Here are some of the cool and less-obvious features of this site:

- [Atom](https://validator.w3.org/feed/docs/atom.html) for web feeds (I use the
  term "RSS" on the site because it's more widely recognized). I love web feeds
  and wish people used them more.
- [Open Graph](https://ogp.me/) and [Twitter
  Card](https://developer.x.com/en/docs/x-for-websites/cards/overview/abouts-cards)
  metadata for rich previews when sharing links.
- [JSON-LD](https://json-ld.org/) metadata for rich semantic data about the
  site.
- [Theme picker](/#theme-picker) for changing the site's color scheme. The site
  is built using the standard 16-color ANSI palette, so it's easy to add pretty
  much any standard terminal color scheme.

## Accessibility

Simple sites are more accessible!

Unfortunately, screen readers (like [Orca](https://orca.gnome.org/), the one I
use for testing) are fundamentally broken on Wayland, and Fedora 41 has
completely dropped support for X11, leaving users who use a screen reader with
no options. I am quite pissed about this.
