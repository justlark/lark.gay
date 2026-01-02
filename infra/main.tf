terraform {
  required_providers {
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.0"
    }
  }

  backend "s3" {
    key    = "tofu.tfstate"
    bucket = "lark-space-tofu"
    endpoints = {
      s3 = "https://9d143afa60e911e9904e835bd1db8504.r2.cloudflarestorage.com"
    }
    region                      = "auto"
    use_lockfile                = true
    skip_credentials_validation = true
    skip_region_validation      = true
    skip_requesting_account_id  = true
    skip_metadata_api_check     = true
  }
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}
