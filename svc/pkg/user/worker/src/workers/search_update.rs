use chirp_worker::prelude::*;
use proto::backend::pkg::*;

#[worker(name = "user-search-update")]
async fn worker(ctx: &OperationContext<user::msg::search_update::Message>) -> GlobalResult<()> {
	let crdb = ctx.crdb("db-user").await?;
	let user_id = internal_unwrap_owned!(ctx.user_id).as_uuid();

	sqlx::query(indoc!(
		"
		UPDATE users
		SET
			is_searchable = TRUE,
			update_ts = $1
		WHERE user_id = $2
		"
	))
	.bind(ctx.ts())
	.bind(user_id)
	.execute(&crdb)
	.await?;

	Ok(())
}
