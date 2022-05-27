CREATE TABLE todo_user (
	id UUID PRIMARY KEY,
	email TEXT UNIQUE NOT NULL,
	password_hash TEXT NOT NULL
);