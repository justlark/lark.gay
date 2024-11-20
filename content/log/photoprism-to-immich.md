+++
title = "Migrating my photo library to Immich"
description = "I recently switched from PhotoPrism to Immich for my self-hosted photo library, and I'm quite pleased with it."
date = 2024-11-20

[taxonomies]
tags = ["tech"]

[extra]
uuid = "30d66200-1969-4fac-91e8-f5270ceec604"
+++

A few years ago I undertook a project to de-Google my life, as much as possible.
There are a few Google services I still use--I haven't been able to divorce
myself from Google Calendar just yet--but overall the endeavor has been fairly
successful!

Beyond the obvious reason to move away from Google services--privacy--having
custody over my own data is important to me. I've heard too many stories of
people having their Google accounts suspended due to a false-positive in
Google's abuse detection systems, often without recourse.

One of the major Google services I wanted to move away from is Google Photos.
It's admittedly a good product, but I wanted something open-source and
self-hosted with reasonable feature parity. I eventually settled on
[PhotoPrism](https://www.photoprism.app/).

PhotoPrism replicates some of Google Photos' most useful features, namely face
and object detection. It also adds some new ones, like:

- Screenshots, memes, and other non-photographic images are automatically
  partitioned off into a separate folder. This is really nice for allowing me to
  back up everything without cluttering my photos timeline with memes.
- Nearly-identical photos (usually photos that were taken in burst mode or in
  quick succession) are stacked together, so they're still individually
  accessible but don't clutter up your library.
- You can mark some photos as private so they're hidden from your library by
  default. This makes handing your phone to someone to scroll through your
  photos much less anxiety-provoking.
- You can configure it to automatically detect NSFW photos and mark them as
  private.

Unfortunately though, the PhotoPrism UX isn't great. It's slow, there's no
native app for mobile (only a PWA), and it relies on a clunky third-party app to
sync photos from your phone.

A friend recently introduced me to [Immich](https://immich.app/), and I wanted
to give it a try. The UI is much faster than PhotoPrism and more touch-friendly
on mobile, comparable to Google Photos. It doesn't have private photos, photo
stacking, or non-photographic image detection like PhotoPrism, but it does have
a [promising roadmap](https://immich.app/roadmap/) which includes at least some
of those features.

Migrating my photo library over from PhotoPrism to Immich wasn't terribly
difficult. I found [this script](https://github.com/v411e/ppim-migrator) for
migrating albums and favorites, which worked reasonably well. The most
challenging part was that my ISP only offers paltry upload speeds.

I really love just how much high-quality open-source software there is out there
going toe-to-toe with proprietary services from major tech companies. If you're
interested in trying Immich yourself, but find self-hosting intimidating, I
recommend checking out [PikaPods](https://www.pikapods.com/). They offer managed
hosting for a bunch of FOSS apps, and they make it pretty simple.
