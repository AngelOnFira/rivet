use proto::backend::{self, pkg::*};
use rivet_operation::prelude::*;
use serde_json::{json, Value};
use std::{collections::HashMap, str::FromStr, sync::Once, time::Duration};

use rivet_api::{
	apis::{configuration::Configuration, *},
	models,
};

const LOBBY_GROUP_NAME_ID: &str = "test";

static GLOBAL_INIT: Once = Once::new();

struct Ctx {
	op_ctx: OperationContext<()>,
	game_id: Uuid,
	primary_region_id: Uuid,
	namespace_id: Uuid,
	mm_config_meta: backend::matchmaker::VersionConfigMeta,
}

impl Ctx {
	async fn init() -> Ctx {
		GLOBAL_INIT.call_once(|| {
			tracing_subscriber::fmt()
				.pretty()
				.with_max_level(tracing::Level::INFO)
				.with_target(false)
				.without_time()
				.init();
		});

		let pools = rivet_pools::from_env("api-database-test").await.unwrap();
		let cache = rivet_cache::CacheInner::new(
			"api-database-test".to_string(),
			std::env::var("RIVET_SOURCE_HASH").unwrap(),
			pools.redis_cache().unwrap(),
		);
		let client = chirp_client::SharedClient::from_env(pools.clone())
			.expect("create client")
			.wrap_new("api-database-test");
		let conn = rivet_connection::Connection::new(client, pools, cache);
		let op_ctx = OperationContext::new(
			"api-database-test".to_string(),
			std::time::Duration::from_secs(60),
			conn,
			Uuid::new_v4(),
			Uuid::new_v4(),
			util::timestamp::now(),
			util::timestamp::now(),
			(),
			Vec::new(),
		);

		let (primary_region_id, _) = Self::setup_region(&op_ctx).await;
		let (game_id, _, namespace_id, _, mm_config_meta) =
			Self::setup_game(&op_ctx, primary_region_id).await;

		Ctx {
			op_ctx,
			game_id,
			primary_region_id,
			namespace_id,
			mm_config_meta,
		}
	}

	fn config(&self, bearer_token: &str) -> Configuration {
		Configuration {
			base_path: util::env::svc_router_url("api-database"),
			bearer_access_token: Some(bearer_token.to_string()),
			..Default::default()
		}
	}

	// async fn issue_lobby_token(&self) -> String {
	// 	// Create lobby
	// 	let lobby_group_meta = &self.mm_config_meta.lobby_groups[0];
	// 	let lobby_id = Uuid::new_v4();

	// 	msg!([self.op_ctx] mm::msg::lobby_create(lobby_id) -> mm::msg::lobby_create_complete {
	// 		lobby_id: Some(lobby_id.into()),
	// 		namespace_id: Some(self.namespace_id.into()),
	// 		lobby_group_id: lobby_group_meta.lobby_group_id,
	// 		region_id: Some(self.primary_region_id.into()),
	// 		create_ray_id: None,
	// 		preemptively_created: false,
	// 	})
	// 	.await
	// 	.unwrap();

	// 	lobby_token(&self.op_ctx, lobby_id.to_string().as_str()).await
	// }

	async fn issue_cloud_token(&self) -> String {
		let res = op!([self.op_ctx] cloud_game_token_create {
			game_id: Some(self.game_id.into()),
		})
		.await
		.unwrap();

		res.token
	}

	async fn setup_region(ctx: &OperationContext<()>) -> (Uuid, String) {
		tracing::info!("setup region");

		let region_res = op!([ctx] faker_region {}).await.unwrap();
		let region_id = region_res.region_id.as_ref().unwrap().as_uuid();

		let get_res = op!([ctx] region_get {
			region_ids: vec![region_id.into()],
		})
		.await
		.unwrap();
		let region_data = get_res.regions.first().unwrap();

		(region_id, region_data.name_id.clone())
	}

	async fn setup_game(
		ctx: &OperationContext<()>,
		region_id: Uuid,
	) -> (
		Uuid,
		Uuid,
		Uuid,
		backend::matchmaker::VersionConfig,
		backend::matchmaker::VersionConfigMeta,
	) {
		use backend::db::{field::Type as FT, Field};

		let game_res = op!([ctx] faker_game {
			..Default::default()
		})
		.await
		.unwrap();

		let game_version_res = op!([ctx] faker_game_version {
			game_id: game_res.game_id,
			override_database: Some(faker::game_version::request::OverrideDbConfig {
				config: Some(backend::db::GameVersionConfig {
					database_name_id: "test".into(),
					schema: Some(backend::db::Schema {
						collections: vec![
							backend::db::Collection {
								name_id: "test".into(),
								fields: vec![
									Field {
										name_id: "my_int".into(),
										r#type: FT::Int.into(),
										optional: false,
									},
									Field {
										name_id: "my_float".into(),
										r#type: FT::Float.into(),
										optional: false,
									},
									Field {
										name_id: "my_bool".into(),
										r#type: FT::Bool.into(),
										optional: false,
									},
									Field {
										name_id: "my_string".into(),
										r#type: FT::String.into(),
										optional: false,
									},
								],
							},
						],
					}),
				})
			})
			..Default::default()
		})
		.await
		.unwrap();

		let namespace_res = op!([ctx] faker_game_namespace {
			game_id: game_res.game_id,
			version_id: game_version_res.version_id,
			..Default::default()
		})
		.await
		.unwrap();

		(
			game_res.game_id.as_ref().unwrap().as_uuid(),
			game_version_res.version_id.as_ref().unwrap().as_uuid(),
			namespace_res.namespace_id.as_ref().unwrap().as_uuid(),
			game_version_res.mm_config.clone().unwrap(),
			game_version_res.mm_config_meta.clone().unwrap(),
		)
	}
}

#[tokio::test(flavor = "multi_thread")]
async fn generic() {
	let ctx = Ctx::init().await;
	let cloud_token = ctx.issue_cloud_token().await;

	let insert_res = database_api::database_insert(
		&ctx.config(&cloud_token),
		"test".into(),
		models::DatabaseInsertRequest {
			database_id: None,
			entry: {
				let mut x = HashMap::new();
				x.insert("my_int".into(), json!(42));
				x.insert("my_float".into(), json!(1.23));
				x.insert("my_bool".into(), json!(true));
				x.insert("my_string".into(), json!("hello, world!"));
				x
			},
		},
	)
	.await
	.unwrap();
	let entry_id = insert_res.ids.first().unwrap();

	let fetch_res = database_api::database_fetch(
		&ctx.config(&cloud_token),
		"test".into(),
		models::DatabaseFetchRequest {
			database_id: None,
			filters: Some(vec![models::DatabaseFilter {
				field: "my_int".into(),
				eq: Some(Some(json!(42))),
			}]),
		},
	)
	.await
	.unwrap();
	assert_eq!(1, fetch_res.entries.len());
	assert_eq!(
		entry_id,
		fetch_res.entries.first().unwrap().get("_id").unwrap()
	)

	// TODO: Update
	// TODO: Delete
}

/// Issues a testing lobby token. We use this since we can't access the lobby token issued
/// on the lobby creation.
async fn lobby_token(ctx: &OperationContext<()>, lobby_id: &str) -> String {
	let token_res = op!([ctx] token_create {
		issuer: "test".into(),
		token_config: Some(token::create::request::TokenConfig {
			ttl: util::duration::days(365),
		}),
		refresh_token_config: None,
		client: None,
		kind: Some(token::create::request::Kind::New(token::create::request::KindNew {
			entitlements: vec![
				proto::claims::Entitlement {
					kind: Some(
						proto::claims::entitlement::Kind::MatchmakerLobby(proto::claims::entitlement::MatchmakerLobby {
							lobby_id: Some(Uuid::from_str(lobby_id).unwrap().into()),
						})
					)
				}
			],
		})),
		label: Some("lobby".into()),
		..Default::default()
	})
	.await
	.unwrap();

	token_res.token.as_ref().unwrap().token.clone()
}
