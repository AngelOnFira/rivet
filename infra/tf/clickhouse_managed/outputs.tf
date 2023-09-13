output "host"{
	value = clickhouse_service.main.endpoints[0].host
}

output "port"{
	value = clickhouse_service.main.endpoints[0].port
}

output "cluster_ca_crt"{
	value = null
}

output "username" {
	value = "default"
}

output "password" {
	value = random_password.default.result
	sensitive = true
}

