terraform {
	required_providers {
		cloudflare = {
			source = "cloudflare/cloudflare"
			version = "4.7.1"
		}
	}
}

module "secrets" {
	source = "../modules/secrets"

	keys = ["cloudflare/terraform/auth_token"]
}

locals {
	cf_request_meta_routes = toset([
		"matchmaker.api.${var.domain_main}/v1/lobbies/list",
		"matchmaker.api.${var.domain_main}/v1/lobbies/join",
		"matchmaker.api.${var.domain_main}/v1/regions",
		"matchmaker.api.${var.domain_main}/v1/lobbies/find",
		"party.api.${var.domain_main}/v1/parties/self/activity/matchmaker/lobbies/find",
		"party.api.${var.domain_main}/v1/parties/self/activity/matchmaker/lobbies/join",
	])
}

resource "cloudflare_worker_script" "request_meta" {
	account_id = var.cloudflare_account_id
	name = "${var.namespace}-request-meta"
	content = file("${path.module}/files/request_meta.js")
}

resource "cloudflare_worker_route" "request_meta_route" {
	for_each = local.cf_request_meta_routes

	zone_id = var.cloudflare_zone_id_rivet_gg
	pattern = each.value
	script_name = cloudflare_worker_script.request_meta.name
}
