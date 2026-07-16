+++
title = "The Lark Files"
description = "A collaborative scavenger hunt for building lore, finding community, and getting offline."
template = "article.html"
+++

## What is this?

_The Lark Files_ is a series of images, videos, and documents distributed among
people Lark has met. These files form a
[sneakernet](https://en.wikipedia.org/wiki/Sneakernet)--a network of digital
information shared via physical media rather than over the internet. The goal is
to collect as many files as you can by meeting other players in-person and
exchanging what you've found.

_The Lark Files_ is a collaborative game rather than a competitive one. The goal
is to encourage folks to meet up, make friends, and get offline.

Each file is significant to Lark in some way--representing one of its interests,
hobbies, or a piece of lore. These files are intended to be esoteric; you might
not know what you're looking at or how it relates to Lark. A short title is
provided to give you a nugget of context, but the rest is left to your research
and interpretation.

If you want to participate, you can download your first bundle of files below.
From there, you'll need to find other players you can meet up with
in-person--likely first- and second-degree friends of Lark. If you meet Lark
in-person, they'll give you a few files if they have any on them--just ask.

[the-lark-files-bundle.zip](https://share.lark.gay/the-lark-files-bundle.zip)

More files may be added over time, which Lark will then circulate.

For specific instructions on how to play, continue reading.

## How do I participate?

If you download the `.zip` file above or get handed a flash drive from Lark or
one of its friends, you'll see some files:

- 📄 `README.txt`
- 📄 `collection.snk`
- 📁 `files/`

`README.txt` contains basic instructions and a link to this page. You can ignore
`collection.snk` for now. The `files/` folder will contain a few of _The Lark
Files_.

**The one rule of _The Lark Files_ is that you're not allowed to share files
over the internet.** You can meet up in-person, use a mutual friend as a
courier, send a flash drive via postal mail, hide a flash drive in public (a
dead drop), etc.

So what about that `collection.snk` file? _The Lark Files_ are also distributed
via the [sneakerweb](https://sneakerweb.org/), which is a peer-to-peer system
for publishing websites without using the internet. It's like the web, but
offline.

You don't need to be part of the sneakerweb to participate in _The Lark Files_,
but it's the only way to see the full title of each file and keep track of which
you still need to collect.

## What is the sneakerweb?

The sneakerweb works like this:

- Everyone has their own private collection of sneakersites, which may include
  some of _The Lark Files_.
- You can browse the sneakersites you have saved on you computer in your web
  browser.
- To share your collection of sneakersites with someone, you export them to a
  `.snk` file.
- If someone sends you a `.snk` file, you can import those sneakersites into
  your collection.

To be part of the sneakerweb, you'll need to install the CLI tool
[here](https://sneakerweb.org/downloads).

To import a `.snk` file into your collection:

```
sneakerweb import collection.snk
```

To export your collection to a `.snk` file:

```
sneakerweb export collection.snk
```

To browse your collection of sneakersites in your web browser, run this command:

```
sneakerweb serve
```

If you have any of _The Lark Files_ in your sneakerweb collection, you can
browse them by running the command above and opening the link below:

[The Lark Files | Index](http://351aab661378a02bcf30c8396921404b25c53bd1305b6edae217be5e012a9151.localhost:1312/)

## Why the sneakerweb?

Why use the sneakerweb? Why not just pass around flash drives with regular files
on them?

The sneakerweb solves a few problems:

- It's possible to have browsable index of _The Lark Files_; you can see a list
  of all the files in circulation to keep track of which you've collected and
  which you have yet to find. Importantly, when new files are put into
  circulation, the index updates as well.
- You can prove that a file is from Lark; only Lark holds the secret key to
  update which files appear in the index. If a link to a file does _not_ appear
  in the index, it's not one of _The Lark Files_.
