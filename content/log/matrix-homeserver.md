+++
title = "Self-hosting a Matrix homeserver"
description = "I'm excited about a chat platform called Matrix, which is an open-source, decentralized, end-to-end encrypted alternative to platforms like Discord. Against my better judgement, I decided to self-host my own Matrix server. This is my devlog."
date = 2025-03-24

[taxonomies]
tags = ["tech"]

[extra]
uuid = "d78bedd1-e974-43aa-9ba3-65d3ee6b1b88"
+++

## What is Matrix?

If you're not familiar with [Matrix](https://matrix.org/), it's an open-source,
decentralized, end-to-end encrypted chat platform. It's somewhat like Discord,
with direct messages, *rooms* (comparable to channels or group chats in
Discord), and *spaces* (comparable to Discord servers).

Matrix doesn't quite have feature parity with Discord, but it has pretty much
all the features you expect of a chat app. I think the platform needs to mature
before it's ready for mainstream adoption, but I'm holding out hope that someday
it will be viable enough for Discord-based communities to feel enticed to switch
over.

## What's the appeal?

With Discord almost certainly approaching an IPO, I fear that the
[enshittification](https://en.wikipedia.org/wiki/Enshittification) of the
platform is imminent. Paired with the wave of public awareness of decentralized
platforms like Mastodon, Lemmy, Pixelfed, and Peertube that we've seen in recent
months, it feels like a perfect storm for Matrix to start seeing more adoption.

I worry about how deeply invested I am in Discord. There's little stopping the
platform from implementing user-hostile changes, suspending my account without
recourse, spying on me, getting hacked, or disappearing entirely. I believe very
strongly in the power of federated, decentralized networks, and I think they're
the only path forward for a free and open internet. Matrix being end-to-end
encrypted is huge; it protects privacy and security in a way that Discord
fundamentally cannot.

It's encouraging to see that there's already a healthy diversity of
[client](https://matrix.org/ecosystem/clients/) and
[server](https://matrix.org/ecosystem/servers/) implementations, even if most of
them are not yet feature-complete. I would have less confidence in the strength
of the community and ecosystem if it consisted of only a single reference
implementation.

## Self-hosting a Matrix homeserver

Matrix has a concept of a *homeserver*, which is the server that hosts your user
account and stores your (encrypted) messages. The domain of your homeserver
forms part of your handle, which takes the form `@username:example.com`. From
the start, I was obsessed with the idea of having a `lark.gay` handle. This does
require hosting your own homeserver, but that also appeals to me because I want
ownership and sovereignty over my own data. The only way to protect your data
from disappearing someday is to host it yourself.

After assessing the landscape of server implementations, I settled on one called
[conduwuit](https://conduwuit.puppyirl.gay/).

The only fully feature-complete and stable server implementation available right
now is [Synapse](https://github.com/element-hq/synapse), however my
understanding is that it's mired in performance issues and is intended to
eventually be phased out by second-gen implementations like
[Dendrite](https://github.com/element-hq/dendrite). I chose conduwuit over
Dendrite because I have a lot more confidence in its maintainership; it seems to
be seeing incredibly active development, whereas Dendrite's commit history
indicates it has been in development for approximately **eight years**. I also
happen to love using software made by trans people 🏳️‍⚧️.

## The devlog

Self-hosting a Matrix homeserver turned out to be somewhat more complicated than
I would have hoped, even for an implementation that's meant to be easy to set
up. This section is a technical devlog detailed how I did it. Before we dive in,
I want to stress that **you do not need to do any of this to use Matrix**.
Unless you *really* know that you want this, I would strongly recommend you
[join an existing homeserver](https://servers.joinmatrix.org/) instead.
Registering is very easy.

conduwuit itself is just a statically linked binary with the database embedded,
which is refreshingly simple. I just dropped the binary in `/usr/local/bin`,
created a system user to run it, and used one of the provided [systemd unit
files](https://conduwuit.puppyirl.gay/configuration/examples.html)—which are
configured to provide a reasonable degree of sandboxing—to run it.

Here are the relevant sections from my conduwuit config:

```toml
[global]

server_name = "lark.gay"
port = 6167
database_path = "/var/lib/conduwuit"

# I had to set this to true temporarily to register my user account. Because I
# don't plan on sharing this server with anyone else, I set it back to false
# immediately after.
allow_registration = false

# In order to set up my user account, I had to generate a random token. After
# disabling registration, there's no need for one.
#registration_token = ""

[global.well_known]

# I'll explain this later.
client = "https://matrix.lark.gay"
server = "matrix.lark.gay:8448"
```

As recommended in the documentation, I set up [Caddy](https://caddyserver.com/)
as a reverse proxy, which automatically handles acquiring and renewing TLS
certificates. The config is remarkably simple:

```
matrix.lark.gay, matrix.lark.gay:8448 {
    reverse_proxy 127.0.0.1:6167
}
```

I had to open up port 8448 (the default port for the Matrix federation API) on
my firewall, in addition to 443 (HTTPS) and 80 (HTTP). Port 80 is necessary for
Caddy to acquire TLS certificates. Because my server is running Fedora Atomic,
which has SELinux enabled, I had to run the following commands to allow Caddy to
bind the port.

```shell
sudo semanage port -a -t http_port_t -p tcp 8448
sudo semanage port -a -t http_port_t -p udp 8448
```

Some of the complexity setting this up comes from the fact that I *really*
wanted my homeserver's domain to be `lark.gay`, which will obviously need to be
shared with this site. To accomplish this, Matrix has a feature called
*delegation*.

To implement Matrix server delegation, I have to serve these two files from this
site, which tell Matrix clients and other servers that they can *actually* find
my homeserver at `matrix.lark.gay`.

`/.well-known/matrix/server`

```json
{
  "m.server": "matrix.lark.gay:8448"
}
```

`/.well-known/matrix/client`

```json
{
  "m.homeserver": "https://matrix.lark.gay"
}
```

For Matrix server delegation to work, you also need to proxy traffic to
`/_matrix/*` and `/_conduwuit/*` to your homeserver. However, `lark.gay` doesn't
point to a web server that I can configure to do that; it points directly to a
CDN. However, Cloudflare [has a
feature](https://developers.cloudflare.com/pages/functions/advanced-mode/) that
lets me put a serverless function in front of the CDN, which you can see
[here](https://github.com/justlark/lark.gay/blob/main/static/_worker.js).

I believe that about covers it! Not too complex in hindsight, but it did take me
some time to figure out, particularly troubleshooting some of the more subtle
bits, like SELinux.

A fun anecdote: I started the process of setting this all up while I was bored
on a flight, connecting to the server over SSH. At one point I needed to reboot,
and committed the classic blunder of locking myself out of SSH access. The
server has full-disk encryption configured with a password, meaning that I need
to be *physically present* to enter the password so it can boot. After that
ordeal, I decided to configure LUKS to use the TPM instead, which allows it to
boot unattended. I also flipped the switch in the UEFI firmware settings setting
it to automatically power back on if it loses and then regains power—say if the
electricity goes out.

## Conclusion

I'm quite pleased with my progress, and excited to start using Matrix… once I
actually have friends to talk to 😅. All my friends are still on Discord, and it
might take some convincing to get them to register an account and install a new
chat app just for me. Thankfully I have some nerd friends who I'm sure will be
happy to indulge me.
