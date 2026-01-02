#!/usr/bin/env nu

def get-tofu-env [] {
  let repo_path = $env.FILE_PWD | path dirname
  let secrets_file = $repo_path | path join "infra" "vars" "env.enc.yaml"

  sops decrypt $secrets_file | from yaml
}

def get-tofu-vars [] {
  let repo_path = $env.FILE_PWD | path dirname
  let vars_file = $repo_path | path join "infra" "vars" "vars.yaml"
  let secrets_file = $repo_path | path join "infra" "vars" "secrets.enc.yaml"

  let tf_plaintext_vars = open $vars_file
  let tf_secret_vars = sops decrypt $secrets_file | from yaml
  let tf_vars = $tf_plaintext_vars | merge $tf_secret_vars

  $tf_vars | items {|key, value| [$"TF_VAR_($key)", $value] } | into record
}

def --wrapped main [...rest] {
  let env = get-tofu-env | merge (get-tofu-vars)

  with-env ($env) {
    tofu -chdir=./infra/ ...$rest
  }
}
