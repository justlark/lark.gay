set dotenv-load

# list recipes
default:
  @just --list

# build and serve the site locally
dev:
  zola --root ./site/ serve

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
