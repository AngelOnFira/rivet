use proto::backend::pkg::*;
use rivet_operation::prelude::*;

#[derive(sqlx::FromRow)]
struct BuildRow {
	build_id: Uuid,
	game_id: Uuid,
	upload_id: Uuid,
	display_name: String,
	image_tag: String,
	create_ts: i64,
}

#[operation(name = "build-get")]
async fn handle(ctx: OperationContext<build::get::Request>) -> GlobalResult<build::get::Response> {
	let build_ids = ctx
		.build_ids
		.iter()
		.map(common::Uuid::as_uuid)
		.collect::<Vec<_>>();

	let builds = sqlx::query_as::<_, BuildRow>(indoc!(
		"
		SELECT build_id, game_id, upload_id, display_name, image_tag, create_ts
		FROM db_build.builds
		WHERE build_id = ANY($1)
		"
	))
	.bind(build_ids)
	.fetch_all(&ctx.crdb().await?)
	.await?
	.into_iter()
	.map(|build| build::get::response::Build {
		build_id: Some(build.build_id.into()),
		game_id: Some(build.game_id.into()),
		upload_id: Some(build.upload_id.into()),
		display_name: build.display_name.clone(),
		image_tag: build.image_tag.clone(),
		create_ts: build.create_ts,
	})
	.collect::<Vec<_>>();

	Ok(build::get::Response { builds })
}
