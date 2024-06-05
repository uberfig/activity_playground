-- Add migration script here
CREATE TABLE internal_users (
	uid 		BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	password	TEXT NOT NULL, --stored with argon2
	preferredUsername	TEXT NOT NULL UNIQUE --basically the username/login name
);

CREATE TABLE activitypub_users (
	id 				BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	internal			BIGINT NULL REFERENCES internal_users(uid) DEFAULT NULL,
	type				TEXT NOT NULL DEFAULT 'Person',
	preferredUsername	TEXT NOT NULL UNIQUE,
	domain				TEXT NOT NULL,
	name				TEXT NOT NULL,
	summary				TEXT NOT NULL DEFAULT '',
	inbox				TEXT NOT NULL,
	outbox				TEXT NOT NULL,
	followers			TEXT NOT NULL,
	following			TEXT NOT NULL,
	liked				TEXT NOT NULL
);