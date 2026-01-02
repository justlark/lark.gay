# Lark Space

This is my personal website and blog. It's a static site built with
[Zola](https://www.getzola.org/) and hosted on
[Cloudflare Workers](https://developers.cloudflare.com/workers/).

See the [Colophon](https://lark.gay/colophon/) for more information about the
technical decision that went into building this site.

## Development

To build and deploy the site and its supporting infrastructure, you'll need to
install:

- [just](https://github.com/casey/just?tab=readme-ov-file#installation)
- [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
- [Nu](https://www.nushell.sh/book/installation.html)
- [OpenTofu](https://opentofu.org/docs/intro/install/)
- [SOPS](https://getsops.io/)
- [age](https://age-encryption.org/)

You can use `just` to build and deploy the site. Run `just` to see a list of
recipes.

## Blog

The comments section for my blog is provided by
[Hyvor Talk](https://talk.hyvor.com/).

## Infrastructure

Infrastructure for this site is managed with [OpenTofu](https://opentofu.org/).
Secrets for managing infrastructure are encrypted with Lark's SSH key via
[age](age-encryption.org) and committed to the repo.

To run `tofu` commands against this repo, use `just tofu`. This wrapper decrypts
secrets and passes them to OpenTofu.

- Plaintext OpenTofu variables are defined in
  [infra/vars/vars.yaml](./infra/vars/vars.yaml).
- Secret OpenTofu variables are defined in
  [infra/vars/secrets.enc.yaml](./infra/vars/secrets.enc.yaml).
- Secret environment variables for configuring the OpenTofu backend are defined
  in [infra/vars/env.enc.yaml](./infra/vars/env.enc.yaml).

To deploy infrastructure, you'll first need your SSH key authorized by adding it
to [infra/vars/.sops.yaml](./infra/vars/.sops.yaml) and running these commands:

```
cd ./infra/vars/
just sops updatekeys ./secrets.enc.yaml
just sops updatekeys ./env.enc.yaml
```

Once your key is authorized, set the env var `SOPS_AGE_SSH_PRIVATE_KEY_FILE` to
the path of your private SSH key. You can put this in a `./.env` file in the
root of the repo; it will be ignored by git.

You can edit OpenTofu secrets interactively like this:

```
cd ./infra/vars/
sops edit ./secrets.enc.yaml
```

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
