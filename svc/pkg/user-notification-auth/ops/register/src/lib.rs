use proto::backend::pkg::*;
use rivet_operation::prelude::*;

#[operation(name = "user-notification-auth-register")]
async fn handle(
	ctx: OperationContext<user_notification_auth::register::Request>,
) -> GlobalResult<user_notification_auth::register::Response> {
	let user_id = unwrap_ref!(ctx.user_id).as_uuid();

	match unwrap_ref!(ctx.registration) {
		user_notification_auth::register::request::Registration::Firebase(registration) => {
			sqlx::query("UPSERT INTO db_user_notification_auth.users (user_id, firebase_access_key) VALUES ($1, $2)")
				.bind(user_id)
				.bind(registration.access_key.clone())
				.execute(&ctx.crdb().await?)
				.await?;
		}
	}

	Ok(user_notification_auth::register::Response {})
}
