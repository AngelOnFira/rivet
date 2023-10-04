terraform {
	required_providers {
		# TODO Revert to gavinbunney/kubectl once https://github.com/gavinbunney/terraform-provider-kubectl/issues/270 is resolved
		kubectl = {
			source = "alekc/kubectl"
			version = ">= 2.0.2"
		}
	}
}

locals {
	entrypoints = var.tls_enabled ? {
		"web" = {}
		"websecure" = {
			tls = {
				secretName = "ingress-tls-cert"
				options = {
					name = "ingress-tls"
					namespace = kubernetes_namespace.traefik.metadata[0].name
				}
			}
		}
		"nomad" = {
			tls = {
				secretName = "ingress-tls-cert-tunnel-server"
				options = {
					name = "ingress-tls-tunnel-server"
					namespace = kubernetes_namespace.traefik_tunnel.metadata[0].name
				}
			}
		}
		"api-route" = {
			tls = {
				secretName = "ingress-tls-cert-tunnel-server"
				options = {
					name = "ingress-tls-tunnel-server"
					namespace = kubernetes_namespace.traefik_tunnel.metadata[0].name
				}
			}
		}
	} : {
		"web" = {}
	}
}
