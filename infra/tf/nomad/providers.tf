provider "nomad" {
	address = var.deploy_method_cluster ? "https://nomad.${var.domain_main}" : "http://127.0.0.1:4646"

	headers {
		name = "CF-Access-Client-Id"
		value = var.deploy_method_cluster ? module.secrets.values["cloudflare/access/terraform_nomad/client_id"] : ""
	}

	headers {
		name = "CF-Access-Client-Secret"
		value = var.deploy_method_cluster ? module.secrets.values["cloudflare/access/terraform_nomad/client_secret"] : ""
	}
}
