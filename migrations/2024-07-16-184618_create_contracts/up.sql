-- Your SQL goes here
CREATE TABLE contracts (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    order_id VARCHAR NOT NULL,
    status INTEGER NOT NULL,
    address VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
