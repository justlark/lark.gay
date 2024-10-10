resource "cloudflare_pages_project" "lark" {
  account_id        = var.cloudflare_account_id
  name              = "larkspace"
  production_branch = "main"

  build_config {
    build_command   = "zola build"
    destination_dir = "public"
  }

  source {
    type = "github"

    config {
      owner                         = "justlark"
      repo_name                     = "lark.gay"
      production_branch             = "main"
      pr_comments_enabled           = false
      deployments_enabled           = true
      production_deployment_enabled = true
      preview_deployment_setting    = "custom"
      preview_branch_includes       = ["dev"]
    }
  }
}

resource "cloudflare_pages_domain" "lark" {
  account_id   = var.cloudflare_account_id
  project_name = cloudflare_pages_project.lark.name
  domain       = "lark.gay"
}

