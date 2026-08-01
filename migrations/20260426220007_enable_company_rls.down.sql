-- Down: remove the company RLS fence for performance module

-- Reverse the company RLS fence for performance.appraisals
DROP POLICY IF EXISTS appraisals_company_isolation ON performance.appraisals;
ALTER TABLE performance.appraisals NO FORCE ROW LEVEL SECURITY;
ALTER TABLE performance.appraisals DISABLE ROW LEVEL SECURITY;

-- Reverse the company RLS fence for performance.appraisal_cycles
DROP POLICY IF EXISTS appraisal_cycles_company_isolation ON performance.appraisal_cycles;
ALTER TABLE performance.appraisal_cycles NO FORCE ROW LEVEL SECURITY;
ALTER TABLE performance.appraisal_cycles DISABLE ROW LEVEL SECURITY;

-- Reverse the company RLS fence for performance.performance_feedback
DROP POLICY IF EXISTS performance_feedback_company_isolation ON performance.performance_feedback;
ALTER TABLE performance.performance_feedback NO FORCE ROW LEVEL SECURITY;
ALTER TABLE performance.performance_feedback DISABLE ROW LEVEL SECURITY;

-- Reverse the company RLS fence for performance.goals
DROP POLICY IF EXISTS goals_company_isolation ON performance.goals;
ALTER TABLE performance.goals NO FORCE ROW LEVEL SECURITY;
ALTER TABLE performance.goals DISABLE ROW LEVEL SECURITY;

-- Reverse the company RLS fence for performance.rewards
DROP POLICY IF EXISTS rewards_company_isolation ON performance.rewards;
ALTER TABLE performance.rewards NO FORCE ROW LEVEL SECURITY;
ALTER TABLE performance.rewards DISABLE ROW LEVEL SECURITY;

-- Reverse the company RLS fence for performance.talent_matrix_entries
DROP POLICY IF EXISTS talent_matrix_entries_company_isolation ON performance.talent_matrix_entries;
ALTER TABLE performance.talent_matrix_entries NO FORCE ROW LEVEL SECURITY;
ALTER TABLE performance.talent_matrix_entries DISABLE ROW LEVEL SECURITY;

