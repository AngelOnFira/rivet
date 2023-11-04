// MARK: Metrics
#[macro_export]
macro_rules! __sql_query_metrics_prepare {
	($prepare_timer:ident) => {
		// Start timer
		let $prepare_timer = tokio::time::Instant::now();
	};
}

#[macro_export]
macro_rules! __sql_query_metrics_start {
	($ctx:expr, $action:expr, $prepare_timer:ident, $start_timer:ident) => {{
		let ctx = &$ctx;
		let location = concat!(file!(), ":", line!(), ":", column!());

		// Count prepare
		let prepare_duration = $prepare_timer.elapsed().as_secs_f64();
		rivet_pools::metrics::SQL_QUERY_PREPARE_DURATION
			.with_label_values(&[stringify!($action), ctx.name(), location])
			.observe(prepare_duration);

		// Count metric
		rivet_pools::metrics::SQL_QUERY_TOTAL
			.with_label_values(&[stringify!($action), ctx.name(), location])
			.inc();
	}

	// Start timer
	let $start_timer = tokio::time::Instant::now();};
}

#[macro_export]
macro_rules! __sql_query_metrics_finish {
	($ctx:expr, $action:expr, $start_timer:ident) => {{
		let ctx = &$ctx;

		let duration = $start_timer.elapsed().as_secs_f64();

		// Log query
		let location = concat!(file!(), ":", line!(), ":", column!());
		tracing::info!(%location, ty = %stringify!($rv), dt = ?duration, action = stringify!($action), "sql query");

		// Count metric
		rivet_pools::metrics::SQL_QUERY_DURATION
			.with_label_values(&[stringify!($action), ctx.name(), location])
			.observe(duration);
	}};
}

// MARK: Helpers
#[macro_export]
macro_rules! __sql_query {
    ([$ctx:expr, $crdb:expr] $sql:expr, $($bind:expr),* $(,)?) => {
		async {

			// Build query before recording metrics so we don't record time for things that don't
			// affect the query
			$crate::__sql_query_metrics_prepare!(_prepare);
			let crdb = $crdb;
			let query = sqlx::query(indoc!($sql))
			$(
				.bind($bind)
			)*
			.execute(crdb);

			// Execute query
			$crate::__sql_query_metrics_start!($ctx, execute, _prepare, _start);
			let res = query.await.map_err(Into::<GlobalError>::into);
			$crate::__sql_query_metrics_finish!($ctx, execute, _start);

			res
		}
    };
    ([$ctx:expr] $sql:expr, $($bind:expr),* $(,)?) => {
		__sql_query!([$ctx, &$ctx.crdb().await?] $sql, $($bind),*)
    };
}

#[macro_export]
macro_rules! __sql_query_as {
    ([$ctx:expr, $rv:ty, $action:ident, $crdb:expr] $sql:expr, $($bind:expr),* $(,)?) => {
		async {
			// Build query before recording metrics so we don't record time for things that don't
			// affect the query
			$crate::__sql_query_metrics_prepare!(_prepare);
			let crdb = $crdb;
			let query = sqlx::query_as::<_, $rv>(indoc!($sql))
			$(
				.bind($bind)
			)*
			.$action(crdb);

			// Execute query
			$crate::__sql_query_metrics_start!($ctx, $action, _prepare, _start);
			let res = query.await.map_err(Into::<GlobalError>::into);
			// $crate::__sql_query_metrics_finish!($ctx, $action, _start);

			res
		}
    };
    ([$ctx:expr, $rv:ty, $action:ident] $sql:expr, $($bind:expr),* $(,)?) => {
		__sql_query_as!([$ctx, $rv, $action, &$ctx.crdb().await?] $sql, $($bind),*)
    };
}

/// Returns a query without being wrapped in an async block, and therefore cannot time the query.
/// Used for the `fetch` function.
#[macro_export]
macro_rules! __sql_query_as_raw {
    ([$ctx:expr, $rv:ty, $action:ident, $crdb:expr] $sql:expr, $($bind:expr),* $(,)?) => {{

		// TODO: Figure out how to wrap this future to be able to record the metrics finish
		$crate::__sql_query_metrics_prepare!(_prepare);
		let query = sqlx::query_as::<_, $rv>(indoc!($sql))
		$(
			.bind($bind)
		)*
		.$action($crdb);
		$crate::__sql_query_metrics_start!($ctx, $action, _prepare, _start);

		query
    }};
    ([$ctx:expr, $rv:ty, $action:ident] $sql:expr, $($bind:expr),* $(,)?) => {
		__sql_query_as!([$ctx, $rv, $action, &$ctx.crdb().await?] $sql, $($bind),*)
    };
}

// MARK: Specific actions
#[macro_export]
macro_rules! sql_execute {
    ($($arg:tt)*) => {
		__sql_query!($($arg)*)
    };
}

#[macro_export]
macro_rules! sql_fetch {
    ([$ctx:expr, $rv:ty, $crdb:expr] $sql:expr, $($bind:expr),* $(,)?) => {
		__sql_query_as_raw!([$ctx, $rv, fetch, $crdb] $sql, $($bind),*)
    };
    ([$ctx:expr, $rv:ty] $sql:expr, $($bind:expr),* $(,)?) => {
		__sql_query_as_raw!([$ctx, $rv, fetch] $sql, $($bind),*)
    };
}

#[macro_export]
macro_rules! sql_fetch_all {
    ([$ctx:expr, $rv:ty, $crdb:expr] $sql:expr, $($bind:expr),* $(,)?) => {
		__sql_query_as!([$ctx, $rv, fetch_all, $crdb] $sql, $($bind),*)
    };
    ([$ctx:expr, $rv:ty] $sql:expr, $($bind:expr),* $(,)?) => {
		__sql_query_as!([$ctx, $rv, fetch_all] $sql, $($bind),*)
    };
}

#[macro_export]
macro_rules! sql_fetch_many {
    ([$ctx:expr, $rv:ty, $crdb:expr] $sql:expr, $($bind:expr),* $(,)?) => {
		__sql_query_as!([$ctx, $rv, fetch_many, $crdb] $sql, $($bind),*)
    };
    ([$ctx:expr, $rv:ty] $sql:expr, $($bind:expr),* $(,)?) => {
		__sql_query_as!([$ctx, $rv, fetch_many] $sql, $($bind),*)
    };
}

#[macro_export]
macro_rules! sql_fetch_one {
    ([$ctx:expr, $rv:ty, $crdb:expr] $sql:expr, $($bind:expr),* $(,)?) => {
		__sql_query_as!([$ctx, $rv, fetch_one, $crdb] $sql, $($bind),*)
    };
    ([$ctx:expr, $rv:ty] $sql:expr, $($bind:expr),* $(,)?) => {
		__sql_query_as!([$ctx, $rv, fetch_one] $sql, $($bind),*)
    };
}

#[macro_export]
macro_rules! sql_fetch_optional {
    ([$ctx:expr, $rv:ty, $crdb:expr] $sql:expr, $($bind:expr),* $(,)?) => {
		__sql_query_as!([$ctx, $rv, fetch_optional, $crdb] $sql, $($bind),*)
    };
    ([$ctx:expr, $rv:ty] $sql:expr, $($bind:expr),* $(,)?) => {
		__sql_query_as!([$ctx, $rv, fetch_optional] $sql, $($bind),*)
    };
}
