-- Hand-authored (user-owned). Not regenerated.
--
-- Best-effort restore sketch for the tenancy strip (ADR-0029). This is a breaking module
-- release against dev-stage databases: the down re-adds the company_id column as nullable
-- with a plain index and the company isolation policy shape, but restores NO data —
-- rows written after the strip (or after the decorator re-keyed them) carry org_unit_id
-- only. The composing service's tenancy decorator remains the live fence; treat this
-- down as a schema-shape sketch for archaeology, not a usable rollback.

ALTER TABLE performance.appraisals               ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE performance.appraisal_cycles         ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE performance.goals                    ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE performance.rewards                  ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE performance.performance_feedback     ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE performance.talent_matrix_entries    ADD COLUMN IF NOT EXISTS company_id uuid;

-- The strip's restored domain one-placement-per-employee-per-cycle unique goes away again
-- (the company-leading variant would need company data this sketch does not restore).
DROP INDEX IF EXISTS performance.idx_talent_matrix_entries_employee_id_cycle_id;

CREATE INDEX IF NOT EXISTS idx_appraisals_company_id            ON performance.appraisals (company_id);
CREATE INDEX IF NOT EXISTS idx_appraisal_cycles_company_id      ON performance.appraisal_cycles (company_id);
CREATE INDEX IF NOT EXISTS idx_goals_company_id                 ON performance.goals (company_id);
CREATE INDEX IF NOT EXISTS idx_rewards_company_id               ON performance.rewards (company_id);
CREATE INDEX IF NOT EXISTS idx_performance_feedback_company_id  ON performance.performance_feedback (company_id);
CREATE INDEX IF NOT EXISTS idx_talent_matrix_entries_company_id ON performance.talent_matrix_entries (company_id);
