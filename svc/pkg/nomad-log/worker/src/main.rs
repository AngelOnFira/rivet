use chirp_worker::prelude::*;
use chirp_worker::ManagerError;

mod workers;
use workers::*;

fn main() -> Result<(), ManagerError> {
	// Start runtime
	rivet_runtime::run(async move {
		worker_group![export, write].await?;

		Ok(())
	})?
}
