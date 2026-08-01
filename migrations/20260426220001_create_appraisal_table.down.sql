-- Down: drop performance.appraisals table
DROP TABLE IF EXISTS performance.appraisals CASCADE;
DROP FUNCTION IF EXISTS performance.appraisals_audit_timestamp() CASCADE;
