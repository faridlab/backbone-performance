-- Down: drop enum types for performance module
DROP TYPE IF EXISTS reward_type CASCADE;
DROP TYPE IF EXISTS goal_status CASCADE;
DROP TYPE IF EXISTS feedback_relationship CASCADE;
DROP TYPE IF EXISTS cycle_status CASCADE;
DROP TYPE IF EXISTS appraisal_status CASCADE;
