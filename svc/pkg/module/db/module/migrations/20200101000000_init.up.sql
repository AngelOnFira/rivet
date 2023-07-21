-- Modules
CREATE TABLE modules (
	module_id UUID PRIMARY KEY,
	name_id STRING NOT NULL,
	team_id UUID NOT NULL,  -- References db-team.teams
	create_ts INT NOT NULL,
	publicity INT NOT NULL,
	UNIQUE INDEX (team_id, name_id ASC)
);

-- Version
CREATE TABLE versions (
	version_id UUID PRIMARY KEY,
	module_id UUID NOT NULL REFERENCES modules (module_id),
	create_ts INT NOT NULL,

	major INT NOT NULL,
	minor INT NOT NULL,
	patch INT NOT NULL,

	INDEX (module_id, create_ts ASC),
	INDEX (module_id, major ASC, minor ASC, patch ASC)
);

CREATE TABLE versions_image_docker (
	version_id UUID PRIMARY KEY REFERENCES versions (version_id),
	image_tag STRING NOT NULL
);

-- Function
CREATE TABLE functions (
	version_id UUID NOT NULL REFERENCES versions (version_id),
	name STRING NOT NULL,
	parameter_schema STRING NOT NULL,
	response_schema STRING NOT NULL,
	PRIMARY KEY (version_id, name)
);

CREATE TABLE functions_callable (
	version_id UUID NOT NULL REFERENCES versions (version_id),
	name STRING NOT NULL,
	PRIMARY KEY (version_id, name),
	FOREIGN KEY (version_id, name) REFERENCES functions (version_id, name)
);

-- Instance
CREATE TABLE instances (
	instance_id UUID PRIMARY KEY,
	version_id UUID NOT NULL REFERENCES versions (version_id),
	create_ts INT NOT NULL
);

CREATE TABLE instances_runtime_fly (
	instance_id UUID PRIMARY KEY REFERENCES instances (instance_id),
	vm_size STRING NOT NULL,
	app_id STRING NOT NULL
);

-- Game version
CREATE TABLE game_versions (
	version_id UUID PRIMARY KEY,  -- References db-game.versions
	config BYTES NOT NULL
);

CREATE TABLE game_version_module_instances (
	game_version_id UUID NOT NULL REFERENCES game_versions (version_id),
	instance_id UUID NOT NULL REFERENCES instances (instance_id),
	PRIMARY KEY (game_version_id, instance_id)
);

