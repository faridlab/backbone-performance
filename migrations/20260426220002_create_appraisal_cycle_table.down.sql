-- Down: drop performance.appraisal_cycles table
DROP TABLE IF EXISTS performance.appraisal_cycles CASCADE;
DROP FUNCTION IF EXISTS performance.appraisal_cycles_audit_timestamp() CASCADE;
