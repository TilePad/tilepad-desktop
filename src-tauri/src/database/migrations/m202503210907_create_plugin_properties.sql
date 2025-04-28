CREATE TABLE IF NOT EXISTS "plugin_properties" (
	"plugin_id"	varchar NOT NULL,
	"properties"	jsonb_text NOT NULL,
	PRIMARY KEY("plugin_id")
);