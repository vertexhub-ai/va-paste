CREATE TABLE IF NOT EXISTS pastes (
    id UUID PRIMARY KEY,
    content TEXT NOT NULL,
    title TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ
);
