-- Down: drop the declared foreign keys for performance module
ALTER TABLE IF EXISTS performance.talent_matrix_entries DROP CONSTRAINT IF EXISTS fk_talent_matrix_entries_cycle_id;
ALTER TABLE IF EXISTS performance.rewards DROP CONSTRAINT IF EXISTS fk_rewards_cycle_id;
ALTER TABLE IF EXISTS performance.goals DROP CONSTRAINT IF EXISTS fk_goals_parent_goal_id;
ALTER TABLE IF EXISTS performance.goals DROP CONSTRAINT IF EXISTS fk_goals_cycle_id;
ALTER TABLE IF EXISTS performance.performance_feedback DROP CONSTRAINT IF EXISTS fk_performance_feedback_cycle_id;
ALTER TABLE IF EXISTS performance.appraisals DROP CONSTRAINT IF EXISTS fk_appraisals_cycle_id;
