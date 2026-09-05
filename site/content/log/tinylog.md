+++
title = "Setting up my capsule tinylog"
description = "After trying various microblogging platforms and falling off every time, I think it's time to try something different."
date = 2026-09-05

[taxonomies]
tags = ["personal", "tech"]

[extra]
uuid = "16c72f9b-c4e9-477f-aab8-d897f67b84ea"
+++

# Starting a tinylog

After Twitter died but before Bluesky won the race to replace it[^1], Mastodon
was picking up a lot of attention from outside the usual FOSS crowd. I had never
tried microblogging before--or any social media for that matter--so I figured I
would give it a go.

I tried Mastodon three times, and fell off each time[^2]. Looking back, I think
microblogging just isn't for me. It felt like everyone who bubbled up in my feed
was funnier and more clever than me, and I couldn't help but compare myself to
them. I was hanging on my phone feeding off the pings every time I got a rare
like or retweet. It wasn't good for me.

I eventually settled on a read-only Bluesky so I can keep up with IRL friends.

Recently, my friend [Ån](https://foxriot.com/) got me back into
[Gemini](https://geminiprotocol.net/)--a sort of lightweight alternative to the
web. They sold me on making a
[tinylog](https://git.sr.ht/~bacardi55/gemini-tinylog-rfc)--the Gemini
equivalent of a microblog.

You can find mine here: <gemini://gemini.lark.gay/log/>

I'm optimistic about this go-around. I think shouting into the void might work
better for me than fishing for engagement; few enough people use Gemini that I
don't feel so much pressure to produce quality content. My hope is that this can
just be a place where I dump my miscellaneous thoughts over the course of the
day without thinking too hard about it. Maybe I have a handful of friends who
check in periodically. Maybe it's just for me.

I even set up a flow so I can post tinylog entries from my phone! That turned
out to be much more complicated than I anticipated, so the rest of this post is
going to be my devlog.

# The devlog

The tinylog itself is a single
[gemtext](https://geminiprotocol.net/docs/gemtext.gmi) file that is appended to
with new log entries. That file then needs to be checked into version control,
pushed to GitHub, and copied to my Gemini server.

To post a tinylog entry, I use [this web form](/tinylog/). I have a magic link
with a token in the query params that authenticates me.

That web form makes a fetch request against
[this edge function](https://github.com/justlark/lark.gay/tree/main/services/tinylog),
which pulls the gemtext file from GitHub, appends the new entry, and pushes it
back to `main`. I made this unnecessarily complicated by calling GitHub's
low-level API to modify the git database directly. I could have just shelled out
to git and executed a few commands, but instead I have to juggle refs, trees,
blobs, and commits by hand. It is unclear why I chose this path.

Once the updated gemtext file is in `main`, I use
[a CI pipeline](https://github.com/justlark/lark.gay/blob/main/.github/workflows/gemini.yaml)
to copy the file to my VPS over SFTP.

It's clunky and inefficient. Posting a single tinylog entry takes over 30
seconds start to finish.

Still, I'm excited to give it a try.

[^1]: Subjective opinion.

[^2]:
    Of course each time I took a break the instance I was using disappeared, so
    I needed to find a new instance and make a new account.
