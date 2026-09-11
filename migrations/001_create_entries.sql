CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    description TEXT,
    started_at TIMESTAMP NOT NULL DEFAULT now(),
    stopped_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);
