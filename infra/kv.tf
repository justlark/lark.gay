resource "cloudflare_workers_kv_namespace" "flipboard" {
  account_id = var.cloudflare_account_id
  title      = "lark-space-flipboard"
}
