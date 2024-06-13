CREATE TABLE activitypub_users (
	ap_user_id			BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	id					TEXT NOT NULL UNIQUE,
	type_field			TEXT NOT NULL DEFAULT 'Person',
	preferred_username	TEXT NOT NULL,
	domain				TEXT NOT NULL,
	name				TEXT NULL,
	summary				TEXT NOT NULL DEFAULT '',
	inbox				TEXT NOT NULL,
	outbox				TEXT NOT NULL,
	followers			TEXT NOT NULL,
	following			TEXT NOT NULL,
	liked				TEXT NULL,

	public_key			TEXT NOT NULL

	-- featured			TEXT,
	-- featuredTags		TEXT,
	-- url					TEXT,
	-- manuallyApprovesFollowers	BOOLEAN,
	-- discoverable		BOOLEAN,
	-- indexable			BOOLEAN,
	-- memorial			BOOLEAN

);

CREATE TABLE internal_users (
	uid 		BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	password	TEXT NOT NULL, --stored with argon2
	preferred_username	TEXT NOT NULL UNIQUE, --basically the username/login name
	activitypub_actor	BIGINT NOT NULL REFERENCES activitypub_users(ap_user_id) ON DELETE CASCADE,
	private_key		TEXT NOT NULL
);

CREATE TABLE activity_objects (
	obj_id		BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,

	type_field		TEXT NOT NULL DEFAULT 'Note',
	id				TEXT NOT NULL UNIQUE,

	name			TEXT NULL,
	attachment
	attributedTo	TEXT NULL REFERENCES activitypub_users(id) ON DELETE CASCADE,
	
	actor 			TEXT NULL REFERENCES activitypub_users(id) ON DELETE CASCADE,
	published		BIGINT

	content			TEXT
);

CREATE TABLE objects (
	id			TEXT PRIMARY KEY NOT NULL UNIQUE,
	type		TEXT NOT NULL,

	ap_user_id	BIGINT NULL REFERENCES activitypub_users(ap_user_id) ON DELETE CASCADE,
	obj_id	BIGINT NULL REFERENCES activity_objects(obj_id) ON DELETE CASCADE
)

