CREATE TABLE users(
    id UUID NOT NULL,
    PRIMARY KEY(id),
    email TEXT NOT NULL UNIQUE,
    matrikelnummer integer NOT NULL UNIQUE,
    name TEXT NOT NULL,
    created_at timestamptz NOT NULL
);
