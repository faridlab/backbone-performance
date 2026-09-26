//! The performance write path (#560): cycles, goals, appraisals — with the
//! council's ONE-finalised-predicate (2026-09-26).
//!
//! The cycle machine (the skeleton's labels): `draft → open → calibration →
//! closed`, with `cancelled` reachable from ANY pre-close state (the council
//! extended cancel into calibration: a hard gate plus one stuck appraisal is
//! a deadlock, not a gate).
//!
//! The appraisal machine: `draft → self_review → submitted → calibrated →
//! finalized`. The employee step writes `self_review`; the manager step
//! (requires the self step done, sysparam `hr.appraisal.allow_manager_first`
//! at the host excepted) writes `manager_review` and moves to
//! `calibration`; the cycle's calibration rates (`overall_rating`); the
//! per-appraisal finalise stamps the row immutable.
//!
//! THE LOAD-BEARING RULE: a cycle CLOSES only when every appraisal is
//! FINALIZED — one durable predicate. The promotion validation port (host
//! side) checks cycle = closed, so a rating crossing that seam is finalised
//! by construction, and the promotion row snapshots it at filing (amendment
//! 4) so the weeks-long window between filing and effective can never audit
//! against a different number.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::performance_events::{PerformanceEvent, PerformanceEventSink, LoggingSink};

#[derive(Debug, thiserror::Error)]
pub enum PerformanceError {
    #[error("db: {0}")]
    Db(#[from] sqlx::Error),
    #[error("not found: {0}")]
    NotFound(&'static str),
    #[error("invalid state: {0}")]
    InvalidState(String),
    #[error("invalid input: {0}")]
    Invalid(String),
}

impl PerformanceError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Db(_) => "internal_error",
            Self::NotFound(_) => "not_found",
            Self::InvalidState(_) => "invalid_state",
            Self::Invalid(_) => "invalid_input",
        }
    }

    pub fn http_status(&self) -> u16 {
        match self {
            Self::Db(_) => 500,
            Self::NotFound(_) => 404,
            _ => 422,
        }
    }
}

/// A new cycle.
pub struct NewCycle {
    pub name: String,
    /// Free-form kind label (annual / quarterly / mid-year).
    pub cycle_type: Option<String>,
    pub period_start: chrono::NaiveDate,
    pub period_end: chrono::NaiveDate,
}

/// A new appraisal (an employee enrolled in an open cycle).
pub struct NewAppraisal {
    pub cycle_id: Uuid,
    pub employee_id: Uuid,
    pub reviewer_id: Uuid,
}

/// A new goal.
pub struct NewGoal {
    pub employee_id: Uuid,
    pub cycle_id: Option<Uuid>,
    pub title: String,
    pub weight: Decimal,
}

pub struct PerformanceWriteService {
    pool: PgPool,
    events: std::sync::RwLock<Arc<dyn PerformanceEventSink>>,
}

impl PerformanceWriteService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            events: std::sync::RwLock::new(Arc::new(LoggingSink)),
        }
    }

    pub fn set_event_sink(&self, sink: Arc<dyn PerformanceEventSink>) {
        *self.events.write().expect("performance events lock poisoned") = sink;
    }

    fn events(&self) -> Arc<dyn PerformanceEventSink> {
        self.events.read().expect("performance events lock poisoned").clone()
    }

    async fn bind_ambient(tx: &mut sqlx::PgConnection) -> Result<(), sqlx::Error> {
        if let Some(scope) = backbone_orm::org_scope::current_org_scope() {
            backbone_orm::org_scope::bind_org_scope_on(tx, &scope).await?;
        }
        Ok(())
    }

    // ── Cycles ───────────────────────────────────────────────────────────

    /// Create a draft cycle.
    pub async fn create_cycle(&self, n: NewCycle) -> Result<Uuid, PerformanceError> {
        if n.name.trim().is_empty() {
            return Err(PerformanceError::Invalid("a cycle needs a name".into()));
        }
        if n.period_end <= n.period_start {
            return Err(PerformanceError::Invalid("period_end must be after period_start".into()));
        }
        let id = Uuid::new_v4();
        let mut tx = self.pool.begin().await?;
        Self::bind_ambient(&mut tx).await?;
        sqlx::query(
            r#"INSERT INTO performance.appraisal_cycles
                   (id, name, cycle_type, period_start, period_end, status)
               VALUES ($1, $2, $3, $4, $5, 'draft')"#,
        )
        .bind(id)
        .bind(&n.name)
        .bind(&n.cycle_type)
        .bind(n.period_start)
        .bind(n.period_end)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(id)
    }

    /// Open a draft cycle (appraisals may enroll).
    pub async fn open_cycle(&self, cycle_id: Uuid) -> Result<(), PerformanceError> {
        self.move_cycle(cycle_id, "draft", "open").await
    }

    /// Move an open cycle into calibration.
    pub async fn enter_calibration(&self, cycle_id: Uuid) -> Result<(), PerformanceError> {
        self.move_cycle(cycle_id, "open", "calibration").await
    }

    /// CLOSE the cycle — the ONE finalised predicate: refuses while ANY
    /// appraisal is not per-appraisal FINALIZED (the council's amendment;
    /// the draft's weaker "not calibrated" gate let unstamped ratings cross
    /// the promotion seam).
    pub async fn close_cycle(&self, cycle_id: Uuid) -> Result<(), PerformanceError> {
        let mut tx = self.pool.begin().await?;
        Self::bind_ambient(&mut tx).await?;
        let status: Option<String> = sqlx::query_scalar(
            "SELECT status::text FROM performance.appraisal_cycles WHERE id = $1 FOR UPDATE",
        )
        .bind(cycle_id)
        .fetch_optional(&mut *tx)
        .await?;
        match status.as_deref() {
            None => return Err(PerformanceError::NotFound("appraisal cycle")),
            Some("closed") => {
                tx.rollback().await?;
                return Ok(()); // idempotent
            }
            Some("calibration") => {}
            Some(other) => {
                return Err(PerformanceError::InvalidState(format!(
                    "the cycle is {other}; only a calibration cycle closes"
                )))
            }
        }
        let unfinalised: i64 = sqlx::query_scalar(
            r#"SELECT COUNT(*) FROM performance.appraisals
                WHERE cycle_id = $1 AND status <> 'finalized'
                  AND (metadata->>'deleted_at') IS NULL"#,
        )
        .bind(cycle_id)
        .fetch_one(&mut *tx)
        .await?;
        if unfinalised > 0 {
            return Err(PerformanceError::InvalidState(format!(
                "{unfinalised} appraisal(s) are not finalized — finalize each before closing the cycle"
            )));
        }
        sqlx::query("UPDATE performance.appraisal_cycles SET status = 'closed' WHERE id = $1")
            .bind(cycle_id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        self.events().publish(PerformanceEvent::CycleClosed { cycle_id });
        Ok(())
    }

    /// Cancel a cycle — reachable from ANY pre-close state (the council's
    /// deadlock breaker).
    pub async fn cancel_cycle(&self, cycle_id: Uuid) -> Result<(), PerformanceError> {
        let mut tx = self.pool.begin().await?;
        Self::bind_ambient(&mut tx).await?;
        let status: Option<String> = sqlx::query_scalar(
            "SELECT status::text FROM performance.appraisal_cycles WHERE id = $1 FOR UPDATE",
        )
        .bind(cycle_id)
        .fetch_optional(&mut *tx)
        .await?;
        match status.as_deref() {
            None => return Err(PerformanceError::NotFound("appraisal cycle")),
            Some("closed") => {
                return Err(PerformanceError::InvalidState("a closed cycle is history — it cannot be cancelled".into()))
            }
            Some("cancelled") => {
                tx.rollback().await?;
                return Ok(());
            }
            _ => {}
        }
        sqlx::query("UPDATE performance.appraisal_cycles SET status = 'cancelled' WHERE id = $1")
            .bind(cycle_id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn move_cycle(&self, cycle_id: Uuid, from: &str, to: &str) -> Result<(), PerformanceError> {
        let mut tx = self.pool.begin().await?;
        Self::bind_ambient(&mut tx).await?;
        let status: Option<String> = sqlx::query_scalar(
            "SELECT status::text FROM performance.appraisal_cycles WHERE id = $1",
        )
        .bind(cycle_id)
        .fetch_optional(&mut *tx)
        .await?;
        match status.as_deref() {
            None => return Err(PerformanceError::NotFound("appraisal cycle")),
            Some(s) if s == from => {}
            Some(other) => {
                return Err(PerformanceError::InvalidState(format!(
                    "the cycle is {other}, not {from}"
                )))
            }
        }
        sqlx::query(&format!(
            "UPDATE performance.appraisal_cycles SET status = '{to}' WHERE id = $1"
        ))
        .bind(cycle_id)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }

    // ── Appraisals ───────────────────────────────────────────────────────

    /// Enroll an employee in an open cycle.
    pub async fn create_appraisal(&self, n: NewAppraisal) -> Result<Uuid, PerformanceError> {
        let id = Uuid::new_v4();
        let mut tx = self.pool.begin().await?;
        Self::bind_ambient(&mut tx).await?;
        let moved = sqlx::query(
            r#"INSERT INTO performance.appraisals (id, cycle_id, employee_id, reviewer_id, status)
               SELECT $1, $2, $3, $4, 'draft'
                WHERE EXISTS (SELECT 1 FROM performance.appraisal_cycles
                               WHERE id = $2 AND status = 'open'
                                 AND (metadata->>'deleted_at') IS NULL)
                   AND NOT EXISTS (SELECT 1 FROM performance.appraisals
                                    WHERE cycle_id = $2 AND employee_id = $3
                                      AND (metadata->>'deleted_at') IS NULL)
               RETURNING 1"#,
        )
        .bind(id)
        .bind(n.cycle_id)
        .bind(n.employee_id)
        .bind(n.reviewer_id)
        .fetch_optional(&mut *tx)
        .await?;
        if moved.is_none() {
            // Distinguish: cycle not open vs already enrolled.
            let cycle: Option<String> = sqlx::query_scalar(
                "SELECT status::text FROM performance.appraisal_cycles WHERE id = $1",
            )
            .bind(n.cycle_id)
            .fetch_optional(&mut *tx)
            .await?;
            tx.rollback().await?;
            return Err(match cycle.as_deref() {
                None => PerformanceError::NotFound("appraisal cycle"),
                Some("open") => PerformanceError::Invalid(
                    "this employee already has an appraisal in this cycle".into(),
                ),
                Some(other) => PerformanceError::InvalidState(format!(
                    "the cycle is {other}; appraisals enroll only while it is open"
                )),
            });
        }
        tx.commit().await?;
        Ok(id)
    }

    /// The employee's self step: writes the structured self-review and moves
    /// draft → self_review.
    pub async fn submit_self(
        &self,
        appraisal_id: Uuid,
        employee_id: Uuid,
        review: serde_json::Value,
    ) -> Result<(), PerformanceError> {
        let mut tx = self.pool.begin().await?;
        Self::bind_ambient(&mut tx).await?;
        let moved = sqlx::query(
            r#"UPDATE performance.appraisals
                  SET self_review = $3, submitted_at = now(), status = 'self_review'
                WHERE id = $1 AND employee_id = $2 AND status = 'draft'
                  AND (metadata->>'deleted_at') IS NULL"#,
        )
        .bind(appraisal_id)
        .bind(employee_id)
        .bind(review)
        .execute(&mut *tx)
        .await?
        .rows_affected();
        tx.commit().await?;
        if moved != 1 {
            return Err(PerformanceError::InvalidState(
                "only your own draft appraisal can take a self review".into(),
            ));
        }
        Ok(())
    }

    /// The reviewer's step: requires the self step done (the host's
    /// allow_manager_first sysparam is enforced by the caller not calling
    /// early); writes the manager review and moves to calibration.
    pub async fn submit_manager(
        &self,
        appraisal_id: Uuid,
        reviewer_id: Uuid,
        review: serde_json::Value,
    ) -> Result<(), PerformanceError> {
        let mut tx = self.pool.begin().await?;
        Self::bind_ambient(&mut tx).await?;
        let moved = sqlx::query(
            r#"UPDATE performance.appraisals
                  SET manager_review = $3, status = 'submitted'
                WHERE id = $1 AND reviewer_id = $2 AND status = 'self_review'
                  AND (metadata->>'deleted_at') IS NULL"#,
        )
        .bind(appraisal_id)
        .bind(reviewer_id)
        .bind(review)
        .execute(&mut *tx)
        .await?
        .rows_affected();
        tx.commit().await?;
        if moved != 1 {
            return Err(PerformanceError::InvalidState(
                "only your assigned appraisal, after the self step, takes a manager review".into(),
            ));
        }
        Ok(())
    }

    /// The calibration rating (the cycle must be in calibration).
    pub async fn rate(
        &self,
        appraisal_id: Uuid,
        rating: Decimal,
    ) -> Result<(), PerformanceError> {
        if rating < Decimal::ZERO {
            return Err(PerformanceError::Invalid("the rating must be non-negative".into()));
        }
        let mut tx = self.pool.begin().await?;
        Self::bind_ambient(&mut tx).await?;
        let moved = sqlx::query(
            r#"UPDATE performance.appraisals a
                  SET overall_rating = $2, rated_at = now(), status = 'calibrated'
                WHERE a.id = $1 AND a.status = 'submitted'
                  AND EXISTS (SELECT 1 FROM performance.appraisal_cycles c
                               WHERE c.id = a.cycle_id AND c.status = 'calibration')
                  AND (a.metadata->>'deleted_at') IS NULL"#,
        )
        .bind(appraisal_id)
        .bind(rating)
        .execute(&mut *tx)
        .await?
        .rows_affected();
        tx.commit().await?;
        if moved != 1 {
            return Err(PerformanceError::InvalidState(
                "only a submitted appraisal, in a calibration cycle, takes a rating".into(),
            ));
        }
        Ok(())
    }

    /// The per-appraisal finalise: stamps the row immutable. The cycle gate
    /// (close) requires THIS state on every appraisal.
    pub async fn finalise_appraisal(&self, appraisal_id: Uuid) -> Result<(), PerformanceError> {
        let mut tx = self.pool.begin().await?;
        Self::bind_ambient(&mut tx).await?;
        use sqlx::Row;
        let row = sqlx::query(
            r#"SELECT employee_id, overall_rating, status::text FROM performance.appraisals
                WHERE id = $1 AND (metadata->>'deleted_at') IS NULL FOR UPDATE"#,
        )
        .bind(appraisal_id)
        .fetch_optional(&mut *tx)
        .await?;
        let row = match row {
            Some(r) => r,
            None => return Err(PerformanceError::NotFound("appraisal")),
        };
        let status: String = row.try_get("status")?;
        if status == "finalized" {
            tx.rollback().await?;
            return Ok(()); // idempotent
        }
        if status != "calibrated" {
            return Err(PerformanceError::InvalidState(
                "only a calibrated appraisal finalises".into(),
            ));
        }
        let employee_id: Uuid = row.try_get("employee_id")?;
        let rating: Option<Decimal> = row.try_get("overall_rating")?;
        sqlx::query("UPDATE performance.appraisals SET status = 'finalized' WHERE id = $1")
            .bind(appraisal_id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        self.events().publish(PerformanceEvent::AppraisalFinalised {
            appraisal_id,
            employee_id,
            rating,
        });
        Ok(())
    }

    // ── Goals ────────────────────────────────────────────────────────────

    /// Create a goal. Weights are validated as a SUM at the read model; the
    /// cap (≤ 100 per employee per cycle) is enforced in the verb's tx.
    pub async fn create_goal(&self, n: NewGoal) -> Result<Uuid, PerformanceError> {
        if n.title.trim().is_empty() {
            return Err(PerformanceError::Invalid("a goal needs a title".into()));
        }
        if n.weight <= Decimal::ZERO {
            return Err(PerformanceError::Invalid("a goal's weight must be positive".into()));
        }
        let id = Uuid::new_v4();
        let mut tx = self.pool.begin().await?;
        Self::bind_ambient(&mut tx).await?;
        // The weight cap: the employee's live goals for the same cycle must
        // stay within 100 once this goal joins (scoped tx, per the council).
        let total: Decimal = sqlx::query_scalar(
            r#"SELECT COALESCE(SUM(weight), 0) FROM performance.goals
                WHERE employee_id = $1
                  AND cycle_id IS NOT DISTINCT FROM $2
                  AND status IN ('draft', 'active')
                  AND (metadata->>'deleted_at') IS NULL"#,
        )
        .bind(n.employee_id)
        .bind(n.cycle_id)
        .fetch_one(&mut *tx)
        .await?;
        if total + n.weight > Decimal::from(100) {
            tx.rollback().await?;
            return Err(PerformanceError::Invalid(format!(
                "the employee's goal weights would sum to {} — the cap is 100",
                total + n.weight
            )));
        }
        sqlx::query(
            r#"INSERT INTO performance.goals (id, employee_id, cycle_id, title, weight, status)
               VALUES ($1, $2, $3, $4, $5, 'active')"#,
        )
        .bind(id)
        .bind(n.employee_id)
        .bind(n.cycle_id)
        .bind(&n.title)
        .bind(n.weight)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(id)
    }

    /// Close a goal (achieved or missed).
    pub async fn close_goal(&self, goal_id: Uuid, achieved: bool) -> Result<(), PerformanceError> {
        let mut tx = self.pool.begin().await?;
        Self::bind_ambient(&mut tx).await?;
        let moved = sqlx::query(
            r#"UPDATE performance.goals
                  SET status = CASE WHEN $2 THEN 'achieved' ELSE 'missed' END
                WHERE id = $1 AND status IN ('draft', 'active')
                  AND (metadata->>'deleted_at') IS NULL"#,
        )
        .bind(goal_id)
        .bind(achieved)
        .execute(&mut *tx)
        .await?
        .rows_affected();
        tx.commit().await?;
        if moved != 1 {
            return Err(PerformanceError::InvalidState(
                "only a draft or active goal closes".into(),
            ));
        }
        Ok(())
    }
}
