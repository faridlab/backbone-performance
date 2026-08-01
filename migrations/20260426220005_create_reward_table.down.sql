-- Down: drop performance.rewards table
DROP TABLE IF EXISTS performance.rewards CASCADE;
DROP FUNCTION IF EXISTS performance.rewards_audit_timestamp() CASCADE;
