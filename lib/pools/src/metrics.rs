use rivet_metrics::{prometheus::*, REGISTRY};

lazy_static::lazy_static! {
	// MARK: CRDB
	pub static ref CRDB_POOL_SIZE: IntGauge = register_int_gauge_with_registry!(
		"crdb_pool_conn_size",
		"Number of SQL connections in the pool.",
		*REGISTRY,
	)
	.unwrap();
	pub static ref CRDB_POOL_NUM_IDLE: IntGauge = register_int_gauge_with_registry!(
		"crdb_pool_num_idle",
		"Number of idle SQL connections in the pool.",
		*REGISTRY,
	)
	.unwrap();
}
