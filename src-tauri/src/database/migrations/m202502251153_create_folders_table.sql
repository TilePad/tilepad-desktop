CREATE TABLE IF NOT EXISTS  "folders" (
	"id"	uuid_text NOT NULL,
	"name"	varchar NOT NULL,
	"config"	jsonb_text NOT NULL,
	"profile_id"	uuid_text NOT NULL,
	"default"	boolean NOT NULL,
	"order"	integer NOT NULL,
	PRIMARY KEY("id"),
	FOREIGN KEY("profile_id") REFERENCES "profiles"("id") ON DELETE CASCADE ON UPDATE CASCADE
);