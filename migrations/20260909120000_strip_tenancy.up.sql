-- Hand-authored (user-owned). Not regenerated.
--
-- Strip every company-fence artifact from the performance tables (ADR-0029): the module is
-- tenant-agnostic; org scoping is installed by the COMPOSING service's tenancy decorator,
-- never by the module. Dropped here, per table: the company-leading indexes, the
-- <table>_company_isolation RLS policy, and the company_id column itself.
--
-- Ordering guard (the decorator must run FIRST on any database with data): the module
-- never moves tenancy data. A table is safe to strip when EITHER
--   a) it carries org_unit_id with no NULLs — the decorator backfilled it from company_id —
--      or b) it is empty (a fresh database: the earlier chain files created it empty).
-- Otherwise the strip RAISEs, naming the decorator step, rather than dropping a column
-- that still holds the only tenancy key. The file is re-runnable (every drop is IF EXISTS
-- and the tracker has no checksums), so a failed run retries cleanly after the decorator
-- lands.
--
-- RLS enable/force flags are deliberately NOT touched: the decorator owns those now.

DO $$
DECLARE
    t text;
    has_org boolean;
    org_nulls bigint;
    total bigint;
    offenders text := '';
BEGIN
    FOREACH t IN ARRAY ARRAY['appraisals', 'appraisal_cycles', 'goals', 'rewards', 'performance_feedback', 'talent_matrix_entries']
    LOOP
        IF to_regclass(format('performance.%I', t)) IS NULL THEN
            CONTINUE; -- chain not fully applied on this database; nothing to strip
        END IF;

        SELECT EXISTS (
                   SELECT 1 FROM information_schema.columns
                   WHERE table_schema = 'performance' AND table_name = t AND column_name = 'org_unit_id'
               )
        INTO has_org;

        EXECUTE format('SELECT count(*) FROM performance.%I', t) INTO total;

        IF has_org THEN
            EXECUTE format(
                'SELECT count(*) FROM performance.%I WHERE org_unit_id IS NULL', t)
            INTO org_nulls;
        ELSE
            org_nulls := total; -- no org column: every row's only tenancy key is company_id
        END IF;

        IF has_org AND org_nulls = 0 THEN
            CONTINUE; -- decorator backfilled: safe
        END IF;
        IF total = 0 THEN
            CONTINUE; -- empty table (fresh database): safe
        END IF;
        offenders := offenders || format(' performance.%s (%s rows, %s rows not covered by org_unit_id);', t, total, org_nulls);
    END LOOP;

    IF offenders <> '' THEN
        RAISE EXCEPTION 'refusing to strip company_id — these tables are not yet covered by the tenancy decorator:%. Apply the composing service''s tenancy decorator (it backfills org_unit_id from company_id) and re-run; it is the only step that moves tenancy data.', offenders;
    END IF;
END $$;

-- ── appraisals ────────────────────────────────────────────────────────────────
DROP INDEX IF EXISTS performance.idx_appraisals_company_id_employee_id_cycle_id;
DROP POLICY IF EXISTS appraisals_company_isolation ON performance.appraisals;
ALTER TABLE performance.appraisals DROP COLUMN IF EXISTS company_id;

-- ── appraisal_cycles ──────────────────────────────────────────────────────────
DROP INDEX IF EXISTS performance.idx_appraisal_cycles_company_id_status;
DROP POLICY IF EXISTS appraisal_cycles_company_isolation ON performance.appraisal_cycles;
ALTER TABLE performance.appraisal_cycles DROP COLUMN IF EXISTS company_id;

-- ── goals ─────────────────────────────────────────────────────────────────────
DROP INDEX IF EXISTS performance.idx_goals_company_id_employee_id;
DROP POLICY IF EXISTS goals_company_isolation ON performance.goals;
ALTER TABLE performance.goals DROP COLUMN IF EXISTS company_id;

-- ── rewards ───────────────────────────────────────────────────────────────────
DROP INDEX IF EXISTS performance.idx_rewards_company_id_employee_id_awarded_at;
DROP POLICY IF EXISTS rewards_company_isolation ON performance.rewards;
ALTER TABLE performance.rewards DROP COLUMN IF EXISTS company_id;

-- ── performance_feedback ──────────────────────────────────────────────────────
DROP INDEX IF EXISTS performance.idx_performance_feedback_company_id_to_employee_id;
DROP POLICY IF EXISTS performance_feedback_company_isolation ON performance.performance_feedback;
ALTER TABLE performance.performance_feedback DROP COLUMN IF EXISTS company_id;

-- ── talent_matrix_entries ─────────────────────────────────────────────────────
DROP INDEX IF EXISTS performance.idx_talent_matrix_entries_company_id_employee_id_cycle_id;
DROP POLICY IF EXISTS talent_matrix_entries_company_isolation ON performance.talent_matrix_entries;
ALTER TABLE performance.talent_matrix_entries DROP COLUMN IF EXISTS company_id;

-- ── Restore the domain one-placement-per-employee-per-cycle unique (tenant-free) ──
-- One matrix placement per employee per cycle is a DOMAIN invariant, not a tenancy
-- posture: an employee belongs to exactly one unit under any deployment, so the
-- per-employee-per-cycle unique needs no tenant column. This carries the exact
-- pre-fence predicate. The per-unit scope-leading uniques, when wanted, are POSTURE
-- and are owned by the composing service's tenancy decorator — they are intentionally
-- NOT created here (the pre-fence global form already permits two units of one tenant
-- to keep their own placements because employees never share ids across tenants).
CREATE UNIQUE INDEX IF NOT EXISTS idx_talent_matrix_entries_employee_id_cycle_id
    ON performance.talent_matrix_entries (employee_id, cycle_id) WHERE (metadata ->> 'deleted_at') IS NULL;
