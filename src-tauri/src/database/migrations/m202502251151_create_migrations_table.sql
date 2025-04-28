CREATE TABLE IF NOT EXISTS "migrations" (
	"name"	uuid_text NOT NULL,
	"applied_at"	datetime_text NOT NULL,
	PRIMARY KEY("name")
);