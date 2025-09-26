+++
title = "FanJam: A free event planning app for small cons"
description = "After several months of work, I finally have something ready to start showing off."
date = 2025-09-26

[taxonomies]
tags = ["tech", "community"]

[extra]
uuid = "5a370c99-1a28-4980-a9bb-6db9f674dc75"
+++

A friend approached me with the idea back in the Spring. They were frustrated
because the small con they had recently attended didn't have a good way of
viewing the event schedule. All they had was a public link to a spreadsheet on
Google Docs. This friend had been playing around with Airtable and similar apps,
and decided to copy the con schedule into [NocoDB](https://nocodb.com/) so they
could have something more readable.

The pitch was, "What if you built a mobile app for attendees that was just a
frontend for NocoDB"? That would save a _ton_ of work over trying to build out a
dashboard-like app for organizers. We'd just have to build the mobile app.

It turned out to be a lot of work anyways.

- [FanJam](https://fanjam.live)
- [GitHub](https://github.com/justlark/fanjam)

The mobile app turned out to be the easy part! The hardest part was figuring out
how to make this app free for small cons. If I was planning on eating the
hosting costs, I would need it to be cheap to host. Like, _real_ cheap. My goal
was a dozen cons and a couple thousand concurrent users for less than $40 USD
per month. Yikes.

There are a couple factors that make this more reasonable than you might think:

- Cons generally only last a few days, and generally don't all happen the same
  weekend, so traffic is pretty evenly distributed, and resources can be scaled
  up to accommodate burst traffic without exceeding the budget.
- Planning the schedule for a given con isn't generally a 24/7/365 affair, so I
  can automatically suspend those servers when they're not being used to save
  money.
- The data set is tiny, which means I can keep a full local copy on the user's
  device. This means no loading time in the app and less frequent requests to
  the server.

But there were some challenges to overcome as well, namely that NocoDB is quite
slow. There's no way I can rely on it to absorb significant traffic, which means
I needed to build a caching layer in front of it.

I might share more of the technical details in future log entries. There's
documentation on the architecture of the app in the GitHub repo, including
diagrams.

When I show people FanJam, they'll often pull out their phone and show me the
app from a con they've attended, usually to complain about it. It turns out
there are a _lot_ of bad apps out there trying to do what FanJam does. If I can
build something that does it better and cheaper, I think there might actually be
a market out there for me.

I'm really excited about this project. I have a long backlog of new features and
enhancements I still want to implement, but in the coming weeks I'm going to
start soliciting feedback from more than just a few friends, and maybe even
start approaching some cons.

I imagine the hardest part will be convincing them it's not a scam lol.
