use std::path::Path;

use anyhow::{ensure, Result};
use indoc::formatdoc;
use tokio::{fs, process::Command};

use crate::{config, context::ProjectContext};

pub struct BuildOpts<'a, T: AsRef<str>> {
	pub build_calls: Vec<BuildCall<'a, T>>,
	pub release: bool,
	/// How many threads to run in parallel when building.
	pub jobs: Option<usize>,
}

pub struct BuildCall<'a, T: AsRef<str>> {
	pub path: &'a Path,
	pub bins: &'a [T],
}

pub async fn build<'a, T: AsRef<str>>(ctx: &ProjectContext, opts: BuildOpts<'a, T>) -> Result<()> {
	let jobs_flag = if let Some(jobs) = opts.jobs {
		format!("--jobs {jobs}")
	} else {
		String::new()
	};

	let format_flag = if let Some(fmt) = &ctx.config_local().rust.message_format {
		format!("--message-format={fmt}")
	} else {
		String::new()
	};

	let release_flag = if opts.release { "--release" } else { "" };

	let build_calls = opts
		.build_calls
		.iter()
		.map(|build_call| {
			let path = build_call.path.display();
			let bin_flags = build_call
				.bins
				.iter()
				.map(|x| format!("--bin {}", x.as_ref()))
				.collect::<Vec<String>>()
				.join(" ");

			// TODO: Not sure why the .cargo/config.toml isn't working with nested projects, have to hardcode
			// the target dir
			formatdoc!(
				"
				if [ $? -eq 0 ]; then
					(cd {path} && cargo build {jobs_flag} {format_flag} {release_flag} {bin_flags} --target-dir $TARGET_DIR)
				fi
				"
			)
		})
		.collect::<Vec<_>>()
		.join("\n");

	// Generate build script
	let build_script = formatdoc!(
		r#"
		TARGET_DIR=$(readlink -f ./target)
		# Used for Tokio Console. See https://github.com/tokio-rs/console#using-it
		export RUSTFLAGS="--cfg tokio_unstable"
		# Used for debugging
		# export CARGO_LOG=cargo::core::compiler::fingerprint=info

		{build_calls}

		EXIT_CODE=$?
		"#,
	);

	// Execute build command
	match &ctx.ns().cluster.kind {
		config::ns::ClusterKind::SingleNode { .. } => {
			let mut cmd = Command::new("sh");
			cmd.current_dir(ctx.path());
			cmd.arg("-c");
			cmd.arg(formatdoc!(
				r#"
				{build_script}

				# Exit
				exit $EXIT_CODE
				"#
			));
			let status = cmd.status().await?;

			ensure!(status.success());
		}
		config::ns::ClusterKind::Distributed { .. } => {
			let optimization = if opts.release { "release" } else { "debug" };
			let build_script = formatdoc!(
				r#"
				{build_script}

				# Exit
				exit $EXIT_CODE
				"#
			);
			let repo = &ctx.ns().docker.repository;
			ensure!(repo.ends_with('/'), "docker repository must end with slash");
			let source_hash = ctx.source_hash();

			// Create directory for docker files
			let gen_path = ctx.gen_path().join("docker");
			fs::create_dir_all(&gen_path).await?;

			for call in &opts.build_calls {
				for bin in call.bins {
					let bin = bin.as_ref();
					let image_tag = format!("{repo}{bin}:{source_hash}");

					// TODO: Figure out what to tag images with

					// Write docker file
					let dockerfile_path = gen_path.join(format!("Dockerfile.{bin}"));
					fs::write(
						&dockerfile_path,
						formatdoc!(
							r#"
							FROM rust:1.72-slim as build

							RUN apt-get update
							RUN apt-get install -y protobuf-compiler pkg-config libssl-dev
				
							WORKDIR /usr/rivet
							COPY . .
							RUN ["sh", "-c", {build_script:?}]
				
							FROM debian:12.1-slim as run
							
							COPY --from=build /usr/rivet/target/{optimization}/{bin} /bin/svc
							RUN apt-get update
							RUN apt-get -y install openssl
							
							CMD ["bin/svc"]
							"#
						),
					)
					.await?;

					// Build docker image for each binary needed
					let mut cmd = Command::new("docker");
					cmd.current_dir(ctx.path());
					cmd.arg("build");
					cmd.arg("--rm");
					cmd.arg("-f").arg(dockerfile_path);
					cmd.arg("-t").arg(image_tag);
					cmd.arg(".");

					let status = cmd.status().await?;
					ensure!(status.success());
				}
			}
		}
	}

	Ok(())
}
