+++
title = "The Lark Files"
description = "A collaborative scavenger hunt for building lore, finding community, and getting offline."
template = "article.html"
+++

## Welcome

_The Lark Files_ is a series of images, videos, and documents distributed among
people Lark has met. These files form a
[sneakernet](https://en.wikipedia.org/wiki/Sneakernet)--a network of digital
information shared via physical media rather than over the internet. The goal is
to collect as many files as you can by meeting other players in-person and
exchanging what you've found.

_The Lark Files_ is a collaborative game rather than a competitive one. The goal
is to encourage folks to meet up, make friends, and get offline.

Each file is relevant to Lark in some way--representing one of its interests,
hobbies, or a piece of lore. These files are intended to be esoteric; you might
not know what you're looking at or how it relates to Lark. A short title is
provided to give you a nugget of context, but the rest is left to your research
and interpretation.

If you want to participate, you can download **File \#1** below. Each file is
numbered, so you can keep track of which you've collected and which you're
missing. From there, you'll need to find other players you can meet up with
in-person--likely first- and second-degree friends of Lark. If you meet Lark
in-person, they'll give you a random file if they have one on them--just ask.

[the-lark-files-1.zip](/assets/the-lark-files-1.zip)

More files may be added over time, which Lark will then circulate.

For specific instructions on how to play, continue reading.

## Instructions

_The Lark Files_ are distributed via the [sneakerweb](https://sneakerweb.org/),
which is a peer-to-peer protocol for publishing websites. It's like the web, but
offline.

To share your collection of _The Lark Files_, you'll need to install the
sneakerweb tool [here](https://sneakerweb.org/downloads). This is a command you
need to run in the terminal, but don't be intimidated! It's pretty simple.

When someone shares their collection with you, they'll send you a `.snk` file.
To add that file to your collection, run this command, replacing
`the-lark-files.snk` with the path of the file you were given.

```
sneakerweb import the-lark-files.snk
```

To share your collection with someone else, you can export it to a `.snk` file
with this command:

```
sneakerweb export the-lark-files.snk
```

To see the files you've collected, run this command, then open
[this link](http://351aab661378a02bcf30c8396921404b25c53bd1305b6edae217be5e012a9151.localhost:1312/)
in your browser.

```
sneakerweb serve
```
