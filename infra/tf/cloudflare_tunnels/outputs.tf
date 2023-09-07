output "k8s_output" {
	value = {
		tunnels = {
			for k, v in module.cloudflare_tunnels:
			k => {
				tunnel_name = v.tunnel_name
				tunnel_id = v.tunnel_id
				cert = v.cert
				ingress = v.ingress
			}
		}
	}
    sensitive = true
}
