use chirp_worker::prelude::*;
use proto::backend::{self, pkg::*};

#[worker_test]
async fn empty(ctx: TestCtx) {
	let module_id = Uuid::new_v4();
	let version_id = Uuid::new_v4();

	msg!([ctx] module::msg::create(module_id) -> module::msg::create_complete {
		module_id: Some(module_id.into()),
		name_id: "test".into(),
		team_id: Some(Uuid::new_v4().into()),
		creator_user_id: None,
	})
	.await
	.unwrap();

	msg!([ctx] module::msg::version_create(version_id) -> module::msg::version_create_complete {
		version_id: Some(version_id.into()),
		module_id: Some(module_id.into()),
		creator_user_id: None,

		major: 1,
		minor: 0,
		patch: 0,

		functions: vec![
			backend::module::Function {
				name: "foo".into(),
				request_schema: "{}".into(),
				response_schema: "{}".into(),
				callable: Some(backend::module::function::Callable {}),
			},
		],

		image: Some(module::msg::version_create::message::Image::Docker(module::msg::version_create::message::Docker {
			image_tag: "test".into(),
		})),
	})
	.await
	.unwrap();

	let crdb = ctx.crdb("db-module").await.unwrap();

	let (exists,): (bool,) =
		sqlx::query_as("SELECT EXISTS (SELECT 1 FROM versions WHERE version_id = $1)")
			.bind(version_id)
			.fetch_one(&crdb)
			.await
			.unwrap();
	assert!(exists, "version not created");

	let (exists,): (bool,) = sqlx::query_as(
		"SELECT EXISTS (SELECT 1 FROM functions WHERE version_id = $1 AND name = 'foo')",
	)
	.bind(version_id)
	.fetch_one(&crdb)
	.await
	.unwrap();
	assert!(exists, "function not created");

	let (exists,): (bool,) = sqlx::query_as(
		"SELECT EXISTS (SELECT 1 FROM functions_callable WHERE version_id = $1 AND name = 'foo')",
	)
	.bind(version_id)
	.fetch_one(&crdb)
	.await
	.unwrap();
	assert!(exists, "function not callable");
}
