use anyhow::*;
use bolt_config::ns::ClusterKind;

use crate::{
	config::{self, ns},
	context::ProjectContext,
	dep::{salt, terraform},
	tasks,
	utils::command_helper::CommandHelper,
};

pub mod migrate;

pub struct ExecutePlanOpts {
	pub auto_approve: bool,
}

#[derive(Debug, Clone)]
pub struct PlanStep {
	pub name_id: &'static str,
	pub kind: PlanStepKind,
}

#[derive(Debug, Clone)]
pub enum PlanStepKind {
	Terraform {
		plan_id: String,

		/// If this plan needs to be destroyed when destroying all plans.
		///
		/// This should be false on plans that don't have side effects outside of
		/// provisioned infra. For example, plans that update configurations.
		///
		/// The purpose of this is to speed up the destroy step in CI.
		needs_destroy: bool,
	},
	Salt {
		filter: Option<String>,
		/// Filter which SLS files to apply.
		sls: Option<Vec<String>>,
		config_opts: salt::config::BuildOpts,
	},
	Migrate,
	Up,
}

impl PlanStepKind {
	async fn execute(&self, ctx: ProjectContext, opts: &ExecutePlanOpts) -> Result<()> {
		// Generate the project before each step since things likely changed between steps
		tasks::gen::generate_project(&ctx).await;

		match self {
			PlanStepKind::Terraform { plan_id, .. } => {
				let varfile_path = ctx.gen_tf_env_path();

				let mut cmd = terraform::cli::build_command(&ctx, plan_id).await;
				cmd.arg("apply")
					.arg(format!("-var-file={}", varfile_path.display()))
					.arg("-parallelism=16");
				if opts.auto_approve {
					cmd.arg("-auto-approve");
				}
				cmd.exec().await?;

				terraform::output::clear_cache(&ctx, &plan_id).await;
			}
			PlanStepKind::Salt {
				filter,
				sls,
				config_opts,
			} => {
				let apply_opts = salt::cli::ApplyOpts {
					sls: (*sls).clone(),
					..Default::default()
				};
				if let Some(filter) = &filter {
					salt::cli::apply(&ctx, filter, &apply_opts, config_opts).await?;
				} else {
					salt::cli::apply_all(&ctx, &apply_opts, config_opts).await?;
				}
			}
			PlanStepKind::Migrate => {
				tasks::migrate::up_all(&ctx).await?;
			}
			PlanStepKind::Up => tasks::up::up_all(&ctx).await?,
		}

		Ok(())
	}

	async fn destroy(&self, ctx: ProjectContext, opts: &ExecutePlanOpts) -> Result<()> {
		match self {
			PlanStepKind::Terraform {
				plan_id,
				needs_destroy,
			} => {
				if !needs_destroy {
					return Ok(());
				}

				let varfile_path = ctx.gen_tf_env_path();

				let mut cmd = terraform::cli::build_command(&ctx, plan_id).await;
				cmd.arg("destroy")
					.arg(format!("-var-file={}", varfile_path.display()))
					.arg("-parallelism=16");
				if opts.auto_approve {
					cmd.arg("-auto-approve");
				}
				cmd.exec().await?;

				terraform::output::clear_cache(&ctx, &plan_id).await;
			}
			PlanStepKind::Salt { .. } | PlanStepKind::Migrate | PlanStepKind::Up => {
				// Do nothing
			}
		}

		Ok(())
	}
}

pub fn build_plan(ctx: &ProjectContext, start_at: Option<String>) -> Result<Vec<PlanStep>> {
	let mut plan = Vec::new();

	// TLS
	plan.push(PlanStep {
		name_id: "tf-tls",
		kind: PlanStepKind::Terraform {
			plan_id: "tls".into(),
			needs_destroy: true,
		},
	});

	// Infra
	match ctx.ns().kubernetes.provider {
		ns::KubernetesProvider::K3d { .. } => {
			plan.push(PlanStep {
				name_id: "k8s-k3d",
				kind: PlanStepKind::Terraform {
					plan_id: "k8s_k3d".into(),
					needs_destroy: true,
				},
			});
		}
		ns::KubernetesProvider::AwsEks { .. } => {
			plan.push(PlanStep {
				name_id: "k8s-aws",
				kind: PlanStepKind::Terraform {
					plan_id: "k8s_aws".into(),
					needs_destroy: true,
				},
			});
		}
	}

	// Kubernetes
	plan.push(PlanStep {
		name_id: "k8s-infra",
		kind: PlanStepKind::Terraform {
			plan_id: "k8s_infra".into(),
			needs_destroy: false,
		},
	});

	// CockroachDB
	match ctx.ns().cockroachdb.provider {
		ns::CockroachDBProvider::Kubernetes {} => {
			plan.push(PlanStep {
				name_id: "cockroachdb-k8s",
				kind: PlanStepKind::Terraform {
					plan_id: "cockroachdb_k8s".into(),
					needs_destroy: false,
				},
			});
		}
		ns::CockroachDBProvider::Managed { .. } => {
			plan.push(PlanStep {
				name_id: "cockroachdb-managed",
				kind: PlanStepKind::Terraform {
					plan_id: "cockroachdb_managed".into(),
					needs_destroy: true,
				},
			});
		}
	}

	// Redis
	match ctx.ns().redis.provider {
		ns::RedisProvider::Kubernetes {} => {
			plan.push(PlanStep {
				name_id: "redis-k8s",
				kind: PlanStepKind::Terraform {
					plan_id: "redis_k8s".into(),
					needs_destroy: false,
				},
			});
		}
		ns::RedisProvider::Aws { .. } => {
			plan.push(PlanStep {
				name_id: "redis-aws",
				kind: PlanStepKind::Terraform {
					plan_id: "redis_aws".into(),
					needs_destroy: true,
				},
			});
		}
	}

	// Pools
	plan.push(PlanStep {
		name_id: "tf-pools",
		kind: PlanStepKind::Terraform {
			plan_id: "pools".into(),
			needs_destroy: true,
		},
	});

	// DNS
	plan.push(PlanStep {
		name_id: "tf-dns",
		kind: PlanStepKind::Terraform {
			plan_id: "dns".into(),
			needs_destroy: true,
		},
	});

	// Cloudflare
	plan.push(PlanStep {
		name_id: "tf-cf-workers",
		kind: PlanStepKind::Terraform {
			plan_id: "cloudflare_workers".into(),
			needs_destroy: true,
		},
	});

	if let ns::DnsProvider::Cloudflare {
		access: Some(_), ..
	} = ctx.ns().dns.provider
	{
		plan.push(PlanStep {
			name_id: "tf-cf-tunnels",
			kind: PlanStepKind::Terraform {
				plan_id: "cloudflare_tunnels".into(),
				needs_destroy: true,
			},
		});
	}

	// S3
	let s3_providers = &ctx.ns().s3.providers;
	if s3_providers.minio.is_some() {
		plan.push(PlanStep {
			name_id: "tf-s3-minio",
			kind: PlanStepKind::Terraform {
				plan_id: "s3_minio".into(),
				needs_destroy: false,
			},
		});
	}
	if s3_providers.backblaze.is_some() {
		plan.push(PlanStep {
			name_id: "tf-s3-backblaze",
			kind: PlanStepKind::Terraform {
				plan_id: "s3_backblaze".into(),
				needs_destroy: true,
			},
		});
	}
	if s3_providers.aws.is_some() {
		plan.push(PlanStep {
			name_id: "tf-s3-aws",
			kind: PlanStepKind::Terraform {
				plan_id: "s3_aws".into(),
				needs_destroy: true,
			},
		});
	}

	// Grafana
	if ctx.ns().grafana.is_some() {
		plan.push(PlanStep {
			name_id: "tf-grafana",
			kind: PlanStepKind::Terraform {
				plan_id: "grafana".into(),
				needs_destroy: true,
			},
		});
	}

	plan.push(PlanStep {
		name_id: "migrate",
		kind: PlanStepKind::Migrate,
	});

	plan.push(PlanStep {
		name_id: "up",
		kind: PlanStepKind::Up,
	});

	// Start at the specified step
	if let Some(start_at) = start_at {
		let idx = plan
			.iter()
			.position(|x| x.name_id == start_at)
			.ok_or_else(|| anyhow!("invalid start_at value: {}", start_at))?;

		plan = plan[idx..].to_vec();
	}

	Ok(plan)
}

/// List all of the Terraform plans in use for the generated plan.
pub fn all_terraform_plans(ctx: &ProjectContext) -> Result<Vec<String>> {
	let plan_ids = build_plan(ctx, None)?
		.into_iter()
		.flat_map(|x| {
			if let PlanStepKind::Terraform { plan_id, .. } = x.kind {
				Some(plan_id)
			} else {
				None
			}
		})
		.collect::<Vec<_>>();

	Ok(plan_ids)
}

pub async fn execute_plan(
	ctx: &ProjectContext,
	plan: &[PlanStep],
	opts: ExecutePlanOpts,
) -> Result<()> {
	tasks::gen::generate_project(&ctx).await;

	for (i, step) in plan.iter().enumerate() {
		eprintln!();
		eprintln!();
		rivet_term::status::info(
			"Executing",
			format!("({}/{}) {}", i + 1, plan.len(), step.name_id),
		);
		step.kind.execute(ctx.clone(), &opts).await?;
	}

	Ok(())
}

pub async fn destroy_plan(
	ctx: &ProjectContext,
	plan: &[PlanStep],
	opts: ExecutePlanOpts,
) -> Result<()> {
	tasks::gen::generate_project(&ctx).await;

	for (i, step) in plan.iter().enumerate().rev() {
		eprintln!();
		eprintln!();
		rivet_term::status::info(
			"Destroying",
			format!("({}/{}) {}", i + 1, plan.len(), step.name_id),
		);
		step.kind.destroy(ctx.clone(), &opts).await?;
	}

	Ok(())
}
