#!/usr/bin/env nu

def get-tofu-env [] {
  let repo_path = $env.FILE_PWD | path dirname
  let sops_config = $repo_path | path join "infra" "vars" ".sops.yaml"
  let secrets_file = $repo_path | path join "infra" "vars" "env.enc.yaml"

  sops --config $sops_config decrypt $secrets_file | from yaml
}

def get-tofu-vars [] {
  let repo_path = $env.FILE_PWD | path dirname
  let sops_config = $repo_path | path join "infra" "vars" ".sops.yaml"
  let vars_file = $repo_path | path join "infra" "vars" "vars.yaml"
  let secrets_file = $repo_path | path join "infra" "vars" "secrets.enc.yaml"

  let tf_plaintext_vars = open $vars_file
  let tf_secret_vars = sops --config $sops_config decrypt $secrets_file | from yaml
  let tf_vars = $tf_plaintext_vars | merge $tf_secret_vars

  $tf_vars | items {|key, value| [$"TF_VAR_($key)", $value] } | into record
}

def --wrapped main [...rest] {
  let env_vars = get-tofu-env | merge (get-tofu-vars)

  with-env ($env_vars) {
    tofu -chdir=./infra/ ...$rest
  }
}
