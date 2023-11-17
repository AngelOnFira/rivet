use api_helper::ctx::Ctx;
use proto::backend::pkg::*;
use rivet_operation::prelude::*;
use serde_json::json;

use crate::auth::Auth;

// MARK: POST /groups/{}/developer
pub async fn convert_developer(
	ctx: Ctx<Auth>,
	group_id: Uuid,
	_body: serde_json::Value,
) -> GlobalResult<serde_json::Value> {
	let dev_team_res = op!([ctx] team_dev_get {
		team_ids: vec![group_id.into()],
	})
	.await?;
	if !dev_team_res.teams.is_empty() {
		tracing::info!("team is already a dev team");
		return Ok(json!({}));
	}

	msg!([ctx] team_dev::msg::create(group_id) -> team::msg::update {
		team_id: Some(group_id.into()),
	})
	.await?;

	Ok(json!({}))
}
