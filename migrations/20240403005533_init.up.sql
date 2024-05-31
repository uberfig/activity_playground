-- Add migration script here
CREATE TABLE internal_users (
	uid 		BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	password	TEXT NOT NULL, --stored with argon2
	preferredUsername	TEXT NOT NULL UNIQUE --basically the username/login name
	type				TEXT NOT NULL DEFAULT 'Person',
	name				TEXT,
	summary				TEXT NOT NULL DEFAULT '',
	inbox				TEXT NOT NULL,
	outbox				TEXT NOT NULL,
	followers			TEXT NOT NULL,
	following			TEXT NOT NULL,
	liked				TEXT NOT NULL
);

CREATE TABLE activitypub_users (
	uid 				BIGSERIAL PRIMARY KEY NOT NULL UNIQUE,
	type				TEXT NOT NULL,
	name				TEXT,
	preferredUsername	TEXT NOT NULL UNIQUE, --basically the username/login name
	summary				TEXT NOT NULL DEFAULT '',
	inbox				TEXT NOT NULL,
	outbox				TEXT NOT NULL,
	followers			TEXT NOT NULL,
	following			TEXT NOT NULL,
	liked				TEXT NOT NULL
);