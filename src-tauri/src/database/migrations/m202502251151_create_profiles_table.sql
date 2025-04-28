CREATE TABLE IF NOT EXISTS "profiles" (
	"id"	uuid_text NOT NULL,
	"name"	varchar NOT NULL,
	"default"	boolean NOT NULL,
	"config"	jsonb_text NOT NULL,
	"order"	integer NOT NULL DEFAULT 0,
	PRIMARY KEY("id")
);