use anyhow::*;
use ipnet::Ipv4Net;
use serde::Serialize;
use std::{
	collections::{HashMap, HashSet},
	net::Ipv4Addr,
};

use crate::context::ProjectContext;

use super::{net, pools::Pool, regions::Region};

#[derive(Serialize, Clone)]
pub struct Server {
	pub region_id: String,
	pool_id: String,
	version_id: String,
	index: usize,
	pub name: String,
	size: String,
	netnum: usize,
	pub vlan_ip: Ipv4Addr,
	volumes: HashMap<String, ServerVolume>,
	tags: Vec<String>,
	install_script: String,
}

#[derive(Serialize, Clone)]
pub struct ServerVolume {
	size: usize,
}

pub fn build_servers(
	ctx: &ProjectContext,
	regions: &HashMap<String, Region>,
	pools: &HashMap<String, Pool>,
) -> Result<HashMap<String, Server>> {
	let ns = ctx.ns_id();

	let mut servers = HashMap::<String, Server>::new();
	let mut used_netnums = HashSet::new();
	for pool in &ctx.ns().pools {
		let region_id = &pool.region;
		let pool_id = &pool.pool;
		let version_id = &pool.version;

		let region = regions
			.get(region_id)
			.expect(&format!("missing region: {region_id}"));
		let pool_config = pools
			.get(pool_id.as_str())
			.expect(&format!("missing pool: {pool_id}"));

		// Validate netnum is within range
		assert!(
			pool.netnum > 0,
			"netnum 0 is reserved for misc services and cannot be used by a pool"
		);

		// Validate netnum is unique
		let netnum_already_used = used_netnums.insert(pool.netnum);
		assert!(
			netnum_already_used,
			"netnum {} is already used",
			pool.netnum
		);

		for i in 0..pool.count {
			let name = format!("{ns}-{region_id}-{pool_id}-{version_id}-{i}");

			let volumes = pool
				.volumes
				.iter()
				.map(|(id, volume)| (id.clone(), ServerVolume { size: volume.size }))
				.collect::<HashMap<_, _>>();

			let vlan_ip = Ipv4Net::new(pool_config.vlan_address, pool_config.vlan_prefix_len)?
				.hosts()
				// Add 1 so we don't interfere with the net address
				.nth(i + 1)
				.unwrap();

			let mut server = Server {
				region_id: region_id.clone(),
				pool_id: pool_id.clone(),
				version_id: version_id.clone(),
				index: i,
				name: name.clone(),
				size: pool.size.clone(),
				netnum: pool.netnum,
				vlan_ip,
				volumes,

				// Tags that will be assigned to the servers.
				tags: vec![
					ns.to_string(),
					format!("{ns}-{region_id}"),
					format!("{ns}-{pool_id}"),
					format!("{ns}-{pool_id}-{version_id}"),
					format!("{ns}-{region_id}-{pool_id}"),
					format!("{ns}-{region_id}-{pool_id}-{version_id}"),
				],

				install_script: String::new(),
			};
			server.install_script = super::install_scripts::gen(&server)?;

			servers.insert(name.to_string(), server);
		}
	}

	Ok(servers)
}
