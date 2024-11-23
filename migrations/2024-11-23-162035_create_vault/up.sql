-- Your SQL goes here
CREATE TABLE vaults (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR NOT NULL,
    tags TEXT[],
    iv VARCHAR,
    salt VARCHAR,
    username VARCHAR,
    password VARCHAR,
    setup_key VARCHAR
);
