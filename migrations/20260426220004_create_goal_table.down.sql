-- Down: drop performance.goals table
DROP TABLE IF EXISTS performance.goals CASCADE;
DROP FUNCTION IF EXISTS performance.goals_audit_timestamp() CASCADE;
