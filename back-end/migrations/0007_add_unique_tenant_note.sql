ALTER TABLE admin_finance_tenant_notes
    ADD CONSTRAINT admin_finance_tenant_notes_mongo_id_unique UNIQUE (mongo_id);
