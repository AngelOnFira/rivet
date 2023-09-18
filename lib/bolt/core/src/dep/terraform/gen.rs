use anyhow::Result;
use indoc::{formatdoc, indoc};
use serde_json::json;
use std::collections::HashMap;
use tokio::fs;

use crate::{
	config::{
		ns,
		service::{RuntimeKind, ServiceDomain, UploadPolicy},
	},
	context::ProjectContext,
	dep,
	utils::media_resize,
};

use super::net;

pub async fn project(ctx: &ProjectContext) {
	// Init all Terraform projects in parallel
	let init_futs = crate::tasks::infra::all_terraform_plans(ctx)
		.unwrap()
		.into_iter()
		.map(|plan_id| {
			let ctx = ctx.clone();
			async move {
				// Generate helper resources
				gen_bolt_tf(&ctx, &plan_id).await?;

				// Initiate Terraform
				super::cli::init_if_needed(&ctx, &plan_id).await;

				Result::<()>::Ok(())
			}
		});
	futures_util::future::try_join_all(init_futs).await.unwrap();

	vars(ctx).await;
}

/// Generates the `_bolt.tf` file that lives inside the Terraform project to
/// help configure backends and remote states.
pub async fn gen_bolt_tf(ctx: &ProjectContext, plan_id: &str) -> Result<()> {
	// Configure the backend
	let backend = match ctx.ns().terraform.backend {
		ns::TerraformBackend::Local {} => String::new(),
		ns::TerraformBackend::Postgres {} => indoc!(
			"
			terraform {
				backend \"pg\" {}
			}
			"
		)
		.to_string(),
	};

	// Generate the remote states
	let remote_states =
		if let Some(remote_states) = super::remote_states::dependency_graph(ctx).get(plan_id) {
			let variables = match ctx.ns().terraform.backend {
				ns::TerraformBackend::Local {} => String::new(),
				ns::TerraformBackend::Postgres {} => indoc!(
					"
					variable \"remote_state_pg_conn_str\" {
						type = string
					}
					"
				)
				.to_string(),
			};

			let blocks = remote_states
				.iter()
				.map(|remote| gen_remote_state(ctx, remote))
				.collect::<Vec<_>>()
				.join("\n\n");

			format!("{variables}\n\n{blocks}")
		} else {
			String::new()
		};

	let bolt_tf =
		format!("# This is generated by Bolt. Do not modify.\n\n{backend}\n\n{remote_states}");
	let path = ctx.tf_path().join(plan_id).join("_bolt.tf");

	tokio::fs::write(&path, bolt_tf).await?;

	Ok(())
}

/// Generates a `terraform_remote_state` block for Terraform.
fn gen_remote_state(
	ctx: &ProjectContext,
	remote_state: &super::remote_states::RemoteState,
) -> String {
	let plan_id = &remote_state.plan_id;
	let data_name = remote_state.data_name();
	let workspace =
		dep::terraform::cli::build_localized_workspace_name(ctx.ns_id(), &remote_state.plan_id);

	let meta = if let Some(condition) = &remote_state.condition {
		format!("count = {condition} ? 1 : 0")
	} else {
		String::new()
	};

	match ctx.ns().terraform.backend {
		ns::TerraformBackend::Local {} => formatdoc!(
			"
			data \"terraform_remote_state\" \"{data_name}\" {{
				{meta}

				backend = \"local\"

				config = {{
					path = \"../{plan_id}/terraform.tfstate.d/{workspace}/terraform.tfstate\"
				}}
			}}
			"
		),
		ns::TerraformBackend::Postgres {} => formatdoc!(
			"
			data \"terraform_remote_state\" \"{data_name}\" {{
				{meta}

				backend = \"pg\"
				workspace = \"{workspace}\"

				config = {{
					conn_str = var.remote_state_pg_conn_str
				}}
			}}
			"
		),
	}
}

async fn vars(ctx: &ProjectContext) {
	let all_svc = ctx.all_services().await;
	let config = ctx.ns();
	let ns = ctx.ns_id();

	let mut vars = HashMap::<String, serde_json::Value>::new();

	// Namespace
	vars.insert("namespace".into(), json!(ns));

	match &config.cluster.kind {
		ns::ClusterKind::SingleNode {
			public_ip,
			preferred_subnets,
			..
		} => {
			vars.insert("deploy_method_local".into(), json!(true));
			vars.insert("deploy_method_cluster".into(), json!(false));
			vars.insert("public_ip".into(), json!(public_ip));
			vars.insert("local_preferred_subnets".into(), json!(preferred_subnets));
		}
		ns::ClusterKind::Distributed {} => {
			vars.insert("deploy_method_local".into(), json!(false));
			vars.insert("deploy_method_cluster".into(), json!(true));
		}
	}

	// Remote state
	match ctx.ns().terraform.backend {
		ns::TerraformBackend::Local {} => {}
		ns::TerraformBackend::Postgres {} => {
			let remote_state_pg_conn_str = ctx
				.read_secret(&["terraform", "pg_backend", "conn_str"])
				.await
				.unwrap();
			vars.insert(
				"remote_state_pg_conn_str".into(),
				json!(remote_state_pg_conn_str),
			);
		}
	}

	// Project
	vars.insert(
		"project_root".into(),
		json!(ctx.path().display().to_string()),
	);

	// Domains
	vars.insert("domain_main".into(), json!(ctx.domain_main()));
	vars.insert("domain_cdn".into(), json!(ctx.domain_cdn()));
	vars.insert("domain_job".into(), json!(ctx.domain_job()));

	// Net
	vars.insert("svc_region_netmask".into(), json!(net::svc::REGION_NETMASK));
	vars.insert("svc_pool_netmask".into(), json!(net::svc::POOL_NETMASK));
	vars.insert("vpc_subnet".into(), json!(net::vpc::SUBNET));
	vars.insert("vpc_netmask".into(), json!(net::vpc::NETMASK));
	vars.insert("nebula_subnet".into(), json!(net::nebula::SUBNET));
	vars.insert("nebula_netmask".into(), json!(net::nebula::NETMASK));
	vars.insert("nebula_subnet_svc".into(), json!(net::nebula::SUBNET_SVC));
	vars.insert("nebula_netmask_svc".into(), json!(net::nebula::NETMASK_SVC));
	vars.insert("nebula_subnet_job".into(), json!(net::nebula::SUBNET_JOB));
	vars.insert("nebula_netmask_job".into(), json!(net::nebula::NETMASK_JOB));
	vars.insert(
		"nebula_lighthouse_nebula_ip".into(),
		json!(net::nebula::nebula_lighthouse_nebula_ip(&ctx)),
	);
	vars.insert(
		"salt_master_nebula_ip".into(),
		json!(net::nebula::salt_master_nebula_ip(&ctx)),
	);

	// Cloudflare
	match &config.dns.provider {
		ns::DnsProvider::Cloudflare {
			account_id, zones, ..
		} => {
			vars.insert("cloudflare_account_id".into(), json!(account_id));

			vars.insert("cloudflare_zone_id_rivet_gg".into(), json!(zones.root));
			vars.insert("cloudflare_zone_id_rivet_game".into(), json!(zones.game));
			vars.insert("cloudflare_zone_id_rivet_job".into(), json!(zones.job));
		}
	}

	// Regions
	vars.insert(
		"primary_region".into(),
		json!(ctx.primary_region_or_local()),
	);

	let regions = super::regions::build_regions(&ctx).unwrap();
	vars.insert("regions".into(), json!(&regions));

	// Pools
	let pools = super::pools::build_pools(&ctx).await.unwrap();
	vars.insert("pools".into(), json!(&pools));

	// Servers
	let servers = super::servers::build_servers(&ctx, &regions, &pools).unwrap();
	vars.insert("servers".into(), json!(servers));

	// Services
	{
		let mut services = HashMap::new();
		for (service_id, service) in &config.services {
			services.insert(
				service_id.clone(),
				json!({
					"count": service.count,
					"resources": {
						"cpu": if let ns::CpuResources::Cpu(x) = &service.resources.cpu { *x } else { 0 },
						"cpu_cores": if let ns::CpuResources::CpuCores(x) = &service.resources.cpu { *x } else { 0 },
						"memory": service.resources.memory,
					}
				}),
			);
		}
		vars.insert("services".into(), json!(services));
	}

	// Docker
	vars.insert(
		"authenticate_all_docker_hub_pulls".into(),
		json!(ctx.ns().docker.authenticate_all_docker_hub_pulls),
	);

	// Extra DNS
	{
		let mut extra_dns = Vec::new();

		// Add services
		for svc_ctx in all_svc {
			if let Some(router) = svc_ctx.config().kind.router() {
				for mount in &router.mounts {
					let (domain, zone_name) = match mount.domain {
						ServiceDomain::Base => (ctx.domain_main(), "base"),
						ServiceDomain::BaseGame => (ctx.domain_cdn(), "base_game"),
						ServiceDomain::BaseJob => (ctx.domain_job(), "base_job"),
					};

					extra_dns.push(json!({
						"zone_name": zone_name,
						"name": if let Some(subdomain) = &mount.subdomain {
							format!("{}.{}", subdomain, domain)
						} else {
							domain
						},
					}));
				}
			}
		}

		// Add Minio
		let s3_providers = &ctx.ns().s3.providers;
		if s3_providers.minio.is_some() {
			extra_dns.push(json!({
				"zone_name": "base",
				"name": format!("minio.{}", ctx.domain_main()),
			}));
		}

		vars.insert("extra_dns".into(), json!(extra_dns));
	}

	// Redis services
	{
		let mut redis_svcs = HashMap::<String, serde_json::Value>::new();

		for svc_ctx in all_svc {
			if let RuntimeKind::Redis { persistent } = svc_ctx.config().runtime {
				let name = svc_ctx.name();

				redis_svcs.insert(
					svc_ctx.redis_db_name(),
					json!({
						"endpoint": format!("redis://redis-{name}.svc.cluster.local:6379"),
						"persistent": persistent,
					}),
				);
			}
		}

		vars.insert("redis_dbs".into(), json!(redis_svcs));
	}

	// S3
	{
		// Allow testing domains for non-production environments
		let cors_allowed_origins = ctx.s3_cors_allowed_origins();

		let mut s3_buckets = HashMap::<String, serde_json::Value>::new();

		for svc_ctx in all_svc {
			if let RuntimeKind::S3 { upload_policy } = &svc_ctx.config().runtime {
				s3_buckets.insert(
					svc_ctx.s3_bucket_name().await,
					json!({
						"cors_allowed_origins": cors_allowed_origins,
						"policy": match upload_policy {
							UploadPolicy::None => "none",
							UploadPolicy::Download => "download",
							UploadPolicy::Public => "public",
							UploadPolicy::Upload => "upload",
						},
					}),
				);
			}
		}

		vars.insert("s3_buckets".into(), json!(s3_buckets));

		vars.insert(
			"s3_default_provider".into(),
			json!(ctx.default_s3_provider().unwrap().0.as_str()),
		);
		vars.insert("s3_providers".into(), s3_providers(ctx).await.unwrap());
	}

	// Media presets
	vars.insert(
		"imagor_presets".into(),
		json!(media_resize::build_presets(ctx.ns_id())
			.into_iter()
			.map(media_resize::ResizePresetSerialize::from)
			.collect::<Vec<_>>()),
	);

	vars.insert("kubeconfig_path".into(), json!(ctx.gen_kubeconfig_path()));
	vars.insert(
		"k8s_storage_class".into(),
		json!(match ctx.ns().kubernetes.provider {
			ns::KubernetesProvider::K3d { .. } => "local-path",
			ns::KubernetesProvider::AwsEks { .. } => "ebs-sc",
		}),
	);
	vars.insert("k8s_health_port".into(), json!(dep::k8s::gen::HEALTH_PORT));

	vars.insert(
		"cdn_cache_size_gb".into(),
		json!(config.rivet.cdn.cache_size_gb),
	);

	let tf_gen_path = ctx.gen_tf_env_path();
	let _ = fs::create_dir_all(&tf_gen_path.parent().unwrap()).await;
	let vars_json = serde_json::to_string(&vars).unwrap();
	fs::write(&tf_gen_path, vars_json).await.unwrap();
}

async fn s3_providers(ctx: &ProjectContext) -> Result<serde_json::Value> {
	let mut res = serde_json::Map::with_capacity(1);

	let providers = &ctx.ns().s3.providers;
	if providers.minio.is_some() {
		let s3_config = ctx.s3_config(s3_util::Provider::Minio).await?;
		res.insert(
			"minio".to_string(),
			json!({
				"endpoint_internal": s3_config.endpoint_internal,
				"endpoint_external": s3_config.endpoint_external,
				"region": s3_config.region,
			}),
		);
	}
	if providers.backblaze.is_some() {
		let s3_config = ctx.s3_config(s3_util::Provider::Backblaze).await?;
		res.insert(
			"backblaze".to_string(),
			json!({
				"endpoint_internal": s3_config.endpoint_internal,
				"endpoint_external": s3_config.endpoint_external,
				"region": s3_config.region,
			}),
		);
	}
	if providers.aws.is_some() {
		let s3_config = ctx.s3_config(s3_util::Provider::Aws).await?;
		res.insert(
			"aws".to_string(),
			json!({
				"endpoint_internal": s3_config.endpoint_internal,
				"endpoint_external": s3_config.endpoint_external,
				"region": s3_config.region,
			}),
		);
	}

	Ok(res.into())
}
