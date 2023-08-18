# resource "helm_release" "traefik" {
# 	name = "traefik"
# 	namespace = "kube-system"

# 	repository = "https://traefik.github.io/charts"
# 	chart = "traefik"
# 	values = [yamlencode({
# 		additionalArguments = [
# 			"--api",
# 			"--api.dashboard=true",
# 			# "--api.insecure=true",
# 			"--log.level=DEBUG"
# 		]
# 		providers = {
# 			kubernetesCRD = {
# 				allowCrossNamespace = true
# 			}
# 		}

# 		# Create an IngressRoute for the dashboard
# 		ingressRoute = {
# 			dashboard = {
# 				enabled = true
# 				# Custom match rule with host domain
# 				matchRule = "PathPrefix(`/api`, `/dashboard`)"
# 				entryPoints = ["traefik", "websecure"]
# 				# Add custom middlewares : authentication and redirection
# 				middlewares = [
# 					{
# 						name = "traefik-dashboard-auth"
# 					}
# 				]
# 			}
# 		}

# 		# Create the custom middlewares used by the IngressRoute dashboard (can also be created in another way).
# 		extraObjects = [
# 			{
# 				apiVersion = "v1"
# 				kind = "Secret"
# 				metadata = {
# 					name = "traefik-dashboard-auth-secret"
# 				}
# 				type = "kubernetes.io/basic-auth"
# 				stringData = {
# 					username = "admin"
# 					password = ""
# 				}
# 			},
# 			{
# 				apiVersion = "traefik.io/v1alpha1"
# 				kind = "Middleware"
# 				metadata = {
# 					name = "traefik-dashboard-auth"
# 				}
# 				spec = {
# 					basicAuth = {
# 						secret = "traefik-dashboard-auth-secret"
# 					}
# 				}
# 			}
# 		]
# 	})]
# }
