locals {
	service_vector = lookup(var.services, "vector", {
		count = 1
		resources = {
			cpu = 50
			cpu_cores = 0
			memory = 2000
		}
	})
}

resource "kubernetes_namespace" "vector" {
	metadata {
		name = "vector"
	}
}

resource "helm_release" "vector" {
	name = "vector"
	namespace = kubernetes_namespace.vector.metadata.0.name

	repository = "https://helm.vector.dev"
	chart = "vector"
	version = "0.26.0"
	values = [yamlencode({
		role = "Aggregator"
		resources = var.limit_resources ? {
			limits = {
				memory = "${local.service_vector.resources.memory}Mi"
				cpu = (
					local.service_vector.resources.cpu_cores > 0 ?
					"${local.service_vector.resources.cpu_cores * 1000}m"
					: "${local.service_vector.resources.cpu}m"
				)
			}
		} : null
		podMonitor = {
			enabled = true
		}
		customConfig = {
			data_dir = "/vector-data-dir"
			api = {
				enabled = true
				address = "127.0.0.1:8686"
				playground = false
			}
			sources = {
				vector = {
					type = "vector"
					address = "0.0.0.0:6000"
				}
				
				vector_metrics = {
					type = "internal_metrics"
				}
				vector_logs = {
					type = "internal_logs"
				}
			}
			sinks = {
				prom-exporter = {
					type = "prometheus_exporter"
					inputs = ["vector", "vector_metrics"]
					address = "0.0.0.0:9598"
				}

				console = {
					type = "console"
					inputs = ["vector_logs"]
					encoding = {
						codec = "text"
					}
				}
			}
		}

		# env = [
		# 	{
		# 		name = "VECTOR_LOG"
		# 		value = "debug"
		# 	}
		# ]
	})]
}

