CREATE TABLE activitypub_users (
	context				TEXT NOT NULL DEFAULT 'https://www.w3.org/ns/activitystreams',
	database_id			BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	id					TEXT NOT NULL,
	type_field				TEXT NOT NULL DEFAULT 'Person',
	preferred_username	TEXT NOT NULL,
	domain				TEXT NOT NULL,
	name				TEXT NULL,
	summary				TEXT NOT NULL DEFAULT '',
	inbox				TEXT NOT NULL,
	outbox				TEXT NOT NULL,
	followers			TEXT NOT NULL,
	following			TEXT NOT NULL,
	liked				TEXT NOT NULL,

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
	activitypub_actor	BIGINT NOT NULL REFERENCES activitypub_users(database_id) ON DELETE CASCADE,
	private_key		TEXT NOT NULL
);

-- CREATE TABLE activities (
-- 	database_id 		BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
-- 	owner_id			BIGINT NOT NULL REFERENCES activitypub_users(database_id) ON DELETE CASCADE,


-- 	id					TEXT NOT NULL,
-- 	type				TEXT NOT NULL,
-- 	actor				TEXT NOT NULL,
-- 	published			
-- );

