-- Add migration script here

CREATE TABLE users (
    id               bigserial  PRIMARY KEY,
    name             text NOT NULL,
    email            text UNIQUE NOT NULL,
    password_hash    text NOT NULL,
    created_at       timestamp(0) with time zone NOT NULL DEFAULT NOW()
);


CREATE TABLE snippets (
	id           bigserial PRIMARY KEY,
	title        text  NOT NULL,
	content      text  NOT NULL,
        created_at   timestamp(0) with time zone NOT NULL DEFAULT NOW(),
        expired_at   timestamp(0) with time zone NOT NULL
);

