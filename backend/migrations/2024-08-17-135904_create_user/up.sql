-- Your SQL goes here
CREATE TABLE "user"(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"birthdate" TIMESTAMPTZ NOT NULL
);

