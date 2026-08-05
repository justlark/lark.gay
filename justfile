set shell := ["nu", "-c"]
set dotenv-load

links_namespace := "ed2a7afa62bb40d1a877e02d90e7a41a"

# list recipes
default:
  @just --list

# build and serve the site locally
dev:
  zola --root ./site/ serve --drafts

[working-directory: "./worker/"]
_install:
  npm ci

# deploy the site
[confirm("Deploy the site now?")]
deploy: _install
  zola --root ./site/ build
  npx wrangler@latest deploy --env prod

# run an OpenTofu command
tofu *args:
  ./tools/tofu.nu {{ args }}

# set a URL to redirect to from https://links.lark.gay
[group("links")]
set-redirect-link slug dest:
  npx wrangler@latest kv key put --remote --namespace-id {{ links_namespace }} 'slug:{{ slug }}:link' {{ dest }}

# set a file to serve from https://links.lark.gay
[group("links")]
set-file-link slug filename:
  npx wrangler@latest kv key put --remote --namespace-id {{ links_namespace }} 'slug:{{ slug }}:file' {{ filename }}

# remove a URL link from https://links.lark.gay
[group("links")]
remove-redirect-link slug:
  npx wrangler@latest kv key delete --remote --namespace-id {{ links_namespace }} 'slug:{{ slug }}:link'

# remove a file link from https://links.lark.gay
[group("links")]
remove-file-link slug:
  npx wrangler@latest kv key delete --remote --namespace-id {{ links_namespace }} 'slug:{{ slug }}:file'

# list link slugs from https://links.lark.gay
[group("links")]
list-links:
  npx wrangler@latest kv key list --remote --namespace-id {{ links_namespace }} --prefix 'slug:' | from json | each {|key| $key.name | split row ':' | $"($in.1) \(($in.2)\)" }

# upload a file to be served from https://links.lark.gay
[group("links")]
upload-link-file filename file:
  npx wrangler@latest r2 object put --remote 'lark-space-links/{{ filename }}' --file '{{ file }}'

# delete a file served from https://links.lark.gay
[group("links")]
delete-link-file filename:
  npx wrangler@latest r2 object delete --remote 'lark-space-links/{{ filename }}'
