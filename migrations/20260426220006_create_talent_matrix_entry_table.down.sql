-- Down: drop performance.talent_matrix_entries table
DROP TABLE IF EXISTS performance.talent_matrix_entries CASCADE;
DROP FUNCTION IF EXISTS performance.talent_matrix_entries_audit_timestamp() CASCADE;
