CREATE TABLE admin_finance_plan_entries (
    id          TEXT        PRIMARY KEY,
    period_date DATE        NOT NULL,
    income      INTEGER     NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
