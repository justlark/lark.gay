resource "cloudflare_r2_bucket" "links" {
  account_id = var.cloudflare_account_id
  name       = "lark-space-links"
}
