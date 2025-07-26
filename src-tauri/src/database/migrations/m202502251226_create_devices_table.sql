CREATE TABLE IF NOT EXISTS "devices" (
	"id"	uuid_text NOT NULL,
	"name"	varchar NOT NULL,
	"public_key"	blob NOT NULL,
	"config"	jsonb_text NOT NULL,
	"order"	integer NOT NULL DEFAULT 0,
	"profile_id"	integer NOT NULL,
	"folder_id"	integer NOT NULL,
	"created_at"	datetime_text NOT NULL,
	"last_connected_at"	datetime_text NOT NULL,
	PRIMARY KEY("id"),
	FOREIGN KEY("folder_id") REFERENCES "folders"("id") ON DELETE RESTRICT ON UPDATE CASCADE,
	FOREIGN KEY("profile_id") REFERENCES "profiles"("id") ON DELETE RESTRICT ON UPDATE CASCADE
);