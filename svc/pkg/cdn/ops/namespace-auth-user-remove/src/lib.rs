use proto::backend::pkg::*;
use rivet_operation::prelude::*;

#[operation(name = "cdn-namespace-auth-user-remove")]
async fn handle(
	ctx: OperationContext<cdn::namespace_auth_user_remove::Request>,
) -> GlobalResult<cdn::namespace_auth_user_remove::Response> {
	let namespace_id = internal_unwrap!(ctx.namespace_id).as_uuid();

	sqlx::query(
		"DELETE FROM db_cdn.game_namespace_auth_users WHERE namespace_id = $1 AND user_name = $2",
	)
	.bind(namespace_id)
	.bind(&ctx.user)
	.execute(&ctx.crdb().await?)
	.await?;

	msg!([ctx] cdn::msg::ns_config_update(namespace_id) {
		namespace_id: Some(namespace_id.into()),
	})
	.await?;

	Ok(cdn::namespace_auth_user_remove::Response {})
}
