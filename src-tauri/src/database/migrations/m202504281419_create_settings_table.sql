CREATE TABLE IF NOT EXISTS "settings" (
	"id"	integer NOT NULL,
	"config"	jsonb_text NOT NULL,
	PRIMARY KEY("id")
);