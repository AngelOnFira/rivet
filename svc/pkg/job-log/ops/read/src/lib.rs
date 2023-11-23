use proto::backend::{self, pkg::*};
use rivet_operation::prelude::*;

#[derive(clickhouse::Row, serde::Deserialize)]
struct LogEntry {
	ts: i64,
	message: Vec<u8>,
}

#[operation(name = "job-log-read")]
async fn handle(
	ctx: OperationContext<job_log::read::Request>,
) -> GlobalResult<job_log::read::Response> {
	let clickhouse = rivet_pools::utils::clickhouse::client()?
		.with_user("chirp")
		.with_password(util::env::read_secret(&["clickhouse", "users", "chirp", "password"]).await?)
		.with_database("db_job_log");

	let run_id = unwrap_ref!(ctx.run_id).as_uuid();
	let req_query = unwrap_ref!(ctx.query);

	let order_by = if ctx.order_asc { "ASC" } else { "DESC" };

	let entries = match req_query {
		job_log::read::request::Query::All(_) => {
			query_all(ctx.body(), &clickhouse, run_id, order_by).await?
		}
		job_log::read::request::Query::BeforeTs(ts) => {
			query_before_ts(ctx.body(), &clickhouse, run_id, *ts, order_by).await?
		}
		job_log::read::request::Query::AfterTs(ts) => {
			query_after_ts(ctx.body(), &clickhouse, run_id, *ts, order_by).await?
		}
		job_log::read::request::Query::TsRange(query) => {
			query_ts_range(
				ctx.body(),
				&clickhouse,
				run_id,
				query.after_ts,
				query.before_ts,
				order_by,
			)
			.await?
		}
	};

	Ok(job_log::read::Response { entries })
}

async fn query_all(
	req: &job_log::read::Request,
	clickhouse: &clickhouse::Client,
	run_id: Uuid,
	order_by: &str,
) -> GlobalResult<Vec<backend::job::log::LogEntry>> {
	let mut entries_cursor = clickhouse
		.query(&formatdoc!(
			"
			SELECT ts, message
			FROM run_logs
			WHERE run_id = ? AND task = ? AND stream_type = ?
			ORDER BY ts {order_by}
			LIMIT ?
			"
		))
		.bind(run_id)
		.bind(&req.task)
		.bind(req.stream_type as u8)
		.bind(req.count)
		.fetch::<LogEntry>()?;

	let mut entries = Vec::new();
	while let Some(entry) = entries_cursor.next().await? {
		entries.push(convert_entry(entry));
	}

	Ok(entries)
}

async fn query_before_ts(
	req: &job_log::read::Request,
	clickhouse: &clickhouse::Client,
	run_id: Uuid,
	ts: i64,
	order_by: &str,
) -> GlobalResult<Vec<backend::job::log::LogEntry>> {
	let mut entries_cursor = clickhouse
		.query(&formatdoc!(
			"
			SELECT ts, message
			FROM run_logs
			WHERE run_id = ? AND task = ? AND stream_type = ? AND ts <= fromUnixTimestamp64Milli(?)
			ORDER BY ts {order_by}
			LIMIT ?
			"
		))
		.bind(run_id)
		.bind(&req.task)
		.bind(req.stream_type as u8)
		.bind(ts)
		.bind(req.count)
		.fetch::<LogEntry>()?;

	let mut entries = Vec::new();
	while let Some(entry) = entries_cursor.next().await? {
		entries.push(convert_entry(entry));
	}

	Ok(entries)
}

async fn query_after_ts(
	req: &job_log::read::Request,
	clickhouse: &clickhouse::Client,
	run_id: Uuid,
	ts: i64,
	order_by: &str,
) -> GlobalResult<Vec<backend::job::log::LogEntry>> {
	let mut entries_cursor = clickhouse
		.query(&formatdoc!(
			"
			SELECT ts, message
			FROM run_logs
			WHERE run_id = ? AND task = ? AND stream_type = ? AND ts >= fromUnixTimestamp64Milli(?)
			ORDER BY ts {order_by}
			LIMIT ?
			"
		))
		.bind(run_id)
		.bind(&req.task)
		.bind(req.stream_type as u8)
		.bind(ts)
		.bind(req.count)
		.fetch::<LogEntry>()?;

	let mut entries = Vec::new();
	while let Some(entry) = entries_cursor.next().await? {
		entries.push(convert_entry(entry));
	}

	Ok(entries)
}

async fn query_ts_range(
	req: &job_log::read::Request,
	clickhouse: &clickhouse::Client,
	run_id: Uuid,
	after_ts: i64,
	before_ts: i64,
	order_by: &str,
) -> GlobalResult<Vec<backend::job::log::LogEntry>> {
	let mut entries_cursor = clickhouse
		.query(&formatdoc!(
			"
			SELECT ts, message
			FROM run_logs
			WHERE run_id = ? AND task = ? AND stream_type = ? AND ts > fromUnixTimestamp64Milli(?) AND ts < fromUnixTimestamp64Milli(?)
			ORDER BY ts {order_by}
			LIMIT ?
			"
		))
		.bind(run_id)
		.bind(&req.task)
		.bind(req.stream_type as u8)
		.bind(after_ts)
		.bind(before_ts)
		.bind(req.count)
		.fetch::<LogEntry>()?;

	let mut entries = Vec::new();
	while let Some(entry) = entries_cursor.next().await? {
		entries.push(convert_entry(entry));
	}

	Ok(entries)
}

fn convert_entry(entry: LogEntry) -> backend::job::log::LogEntry {
	backend::job::log::LogEntry {
		ts: entry.ts,
		message: entry.message,
	}
}
