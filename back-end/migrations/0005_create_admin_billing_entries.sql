CREATE TYPE billing_condition AS ENUM ('less_than', 'more_than');

CREATE TABLE admin_billing_entries (
    id                     TEXT PRIMARY KEY,
    fans_count             INTEGER NOT NULL,
    condition              billing_condition NOT NULL,
    basic_plan_price       INTEGER,
    standard_plan_price    INTEGER,
    premium_plan_price     INTEGER,
    individual_plan_price  BOOLEAN NOT NULL DEFAULT FALSE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
