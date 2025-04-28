CREATE TABLE IF NOT EXISTS "tiles" (
	"id"	uuid_text NOT NULL,
	"config"	jsonb_text NOT NULL,
	"folder_id"	uuid_text NOT NULL,
	"row"	integer NOT NULL,
	"column"	integer NOT NULL,
	"properties"	jsonb_text NOT NULL DEFAULT '{}',
	"plugin_id"	TEXT NOT NULL,
	"action_id"	TEXT NOT NULL,
	PRIMARY KEY("id"),
	FOREIGN KEY("folder_id") REFERENCES "folders"("id") ON DELETE CASCADE ON UPDATE CASCADE
);