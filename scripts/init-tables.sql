create table if not exists users (
    id               bigserial  primary key,
    name             text not null,
    email            text unique not null,
    password_hash    text not null,
    created_at       timestamp(0) with time zone not null default now()
);


create table if not exists snippets (
	id           bigserial primary key,
	title        text  not null,
	content      text  not null,
        created_at   timestamp(0) with time zone not null default now(),
        expired_at   timestamp(0) with time zone not null
);


grant all privileges on table users to boxuser;
grant all privileges on table snippets to  boxuser;

/* seed some snippets */
INSERT INTO snippets (title, content, created_at, expired_at) VALUES (
	'An old silent pond',
	'An old silent pond...\nA frog jumps into the pond,\nsplash! Silence again.\n\n– Matsuo Bashō',
	now(),
	now() + interval  '365 day'
);

INSERT INTO snippets (title, content, created_at, expired_at) VALUES (
	'Over the wintry forest',
	'Over the wintry\nforest, winds howl in rage\nwith no leaves to blow.\n\n– Natsume Soseki',
	now(),
	now() + interval  '30 day'
);

INSERT INTO snippets (title, content, created_at, expired_at) VALUES (
	'First autumn morning',
	'First autumn morning\nthe mirror I stare into\nshows my father''s face.\n\n– Murakami Kijo',
	now(),
	now() + interval  '7 day'
);

