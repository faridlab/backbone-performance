-- Down: drop performance.performance_feedback table
DROP TABLE IF EXISTS performance.performance_feedback CASCADE;
DROP FUNCTION IF EXISTS performance.performance_feedback_audit_timestamp() CASCADE;
