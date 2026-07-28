resource "cloudflare_r2_bucket" "once" {
  account_id = var.cloudflare_account_id
  name       = "lark-space-once"
}
