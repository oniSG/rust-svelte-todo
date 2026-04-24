CREATE TABLE admin_finance_tenant_notes (
    id                     TEXT PRIMARY KEY,
    mongo_id               TEXT NOT NULL,
    note                   TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
