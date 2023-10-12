use anyhow::*;
use bolt_core::{context::ProjectContext, tasks, utils};
use clap::Parser;

#[derive(Parser)]
pub struct TestOpts {
	#[clap(index = 1, action = clap::ArgAction::Append)]
	service_names: Vec<String>,
	#[clap(long)]
	filter: Vec<String>,
	/// Amount of parallel tests to run simultaneously.
	#[clap(long, short = 'c')]
	parallel_tests: Option<usize>,
}

impl TestOpts {
	pub async fn execute(self, ctx: ProjectContext) -> Result<()> {
		let TestOpts {
			service_names,
			filter,
			parallel_tests,
		} = self;

		if let Some(parallel_tests) = parallel_tests {
			ensure!(parallel_tests <= 100, "too many parallel tests (> 100)");
		}

		// Test services
		if !service_names.is_empty() {
			tasks::test::test_services(&ctx, &service_names, filter, parallel_tests).await?;
		} else {
			ensure!(
				filter.is_empty(),
				"cannot provide filters when testing all services"
			);
			tasks::test::test_all(&ctx).await?;
		}

		utils::ringadingding();

		Ok(())
	}
}
