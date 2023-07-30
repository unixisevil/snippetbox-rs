-- Add migration script here


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
