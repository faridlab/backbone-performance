//! Guarded route composition — the recommended way to mount the performance
//! module (hand-authored, user-owned).
//!
//! The finalised-predicate fence is BY CONSTRUCTION here: `appraisals` mount
//! READ-ONLY (no generic write surface at all — nothing can patch or
//! soft-delete a finalised rating); every state change rides the write
//! service's verbs. Master data (cycles, goals, feedback requests) keeps its
//! generic writes where side-effect-free, and the verb routes carry the
//! state machine.

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use backbone_auth::org::OrgContext;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::service::{
    NewAppraisal, NewCycle, NewGoal, PerformanceError, PerformanceWriteService,
};
use crate::PerformanceModule;

#[derive(Debug, Serialize)]
struct ErrorBody {
    error: &'static str,
    message: String,
}
#[derive(Debug, Serialize)]
struct IdResponse {
    id: Uuid,
}

fn err_response(e: PerformanceError) -> axum::response::Response {
    (
        StatusCode::from_u16(e.http_status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        Json(ErrorBody { error: e.code(), message: e.to_string() }),
    ).into_response()
}

// ── Cycles ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct CreateCycleBody {
    name: String,
    #[serde(default)]
    cycle_type: Option<String>,
    period_start: NaiveDate,
    period_end: NaiveDate,
}

async fn create_cycle(
    State(svc): State<Arc<PerformanceWriteService>>,
    _org: OrgContext,
    Json(b): Json<CreateCycleBody>,
) -> axum::response::Response {
    match svc
        .create_cycle(NewCycle {
            name: b.name,
            cycle_type: b.cycle_type,
            period_start: b.period_start,
            period_end: b.period_end,
        })
        .await
    {
        Ok(id) => (StatusCode::CREATED, Json(IdResponse { id })).into_response(),
        Err(e) => err_response(e),
    }
}

async fn open_cycle(
    State(svc): State<Arc<PerformanceWriteService>>,
    _org: OrgContext,
    Path(cycle_id): Path<Uuid>,
) -> axum::response::Response {
    match svc.open_cycle(cycle_id).await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({ "status": "open" }))).into_response(),
        Err(e) => err_response(e),
    }
}

async fn enter_calibration(
    State(svc): State<Arc<PerformanceWriteService>>,
    _org: OrgContext,
    Path(cycle_id): Path<Uuid>,
) -> axum::response::Response {
    match svc.enter_calibration(cycle_id).await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({ "status": "calibration" }))).into_response(),
        Err(e) => err_response(e),
    }
}

/// Close the cycle — refuses while ANY appraisal is unfinalised (the ONE
/// finalised predicate; the promotion validation seam rides on it).
async fn close_cycle(
    State(svc): State<Arc<PerformanceWriteService>>,
    _org: OrgContext,
    Path(cycle_id): Path<Uuid>,
) -> axum::response::Response {
    match svc.close_cycle(cycle_id).await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({ "status": "closed" }))).into_response(),
        Err(e) => err_response(e),
    }
}

async fn cancel_cycle(
    State(svc): State<Arc<PerformanceWriteService>>,
    _org: OrgContext,
    Path(cycle_id): Path<Uuid>,
) -> axum::response::Response {
    match svc.cancel_cycle(cycle_id).await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({ "status": "cancelled" }))).into_response(),
        Err(e) => err_response(e),
    }
}

// ── Appraisals ────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct CreateAppraisalBody {
    cycle_id: Uuid,
    employee_id: Uuid,
    reviewer_id: Uuid,
}

async fn create_appraisal(
    State(svc): State<Arc<PerformanceWriteService>>,
    _org: OrgContext,
    Json(b): Json<CreateAppraisalBody>,
) -> axum::response::Response {
    match svc
        .create_appraisal(NewAppraisal {
            cycle_id: b.cycle_id,
            employee_id: b.employee_id,
            reviewer_id: b.reviewer_id,
        })
        .await
    {
        Ok(id) => (StatusCode::CREATED, Json(IdResponse { id })).into_response(),
        Err(e) => err_response(e),
    }
}

#[derive(Debug, Deserialize)]
struct SelfReviewBody {
    appraisal_id: Uuid,
    employee_id: Uuid,
    review: serde_json::Value,
}

async fn submit_self(
    State(svc): State<Arc<PerformanceWriteService>>,
    _org: OrgContext,
    Json(b): Json<SelfReviewBody>,
) -> axum::response::Response {
    match svc.submit_self(b.appraisal_id, b.employee_id, b.review).await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({ "status": "self_review" }))).into_response(),
        Err(e) => err_response(e),
    }
}

#[derive(Debug, Deserialize)]
struct ManagerReviewBody {
    appraisal_id: Uuid,
    reviewer_id: Uuid,
    review: serde_json::Value,
}

async fn submit_manager(
    State(svc): State<Arc<PerformanceWriteService>>,
    _org: OrgContext,
    Json(b): Json<ManagerReviewBody>,
) -> axum::response::Response {
    match svc.submit_manager(b.appraisal_id, b.reviewer_id, b.review).await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({ "status": "submitted" }))).into_response(),
        Err(e) => err_response(e),
    }
}

#[derive(Debug, Deserialize)]
struct RateBody {
    rating: rust_decimal::Decimal,
}

async fn rate_appraisal(
    State(svc): State<Arc<PerformanceWriteService>>,
    _org: OrgContext,
    Path(appraisal_id): Path<Uuid>,
    Json(b): Json<RateBody>,
) -> axum::response::Response {
    match svc.rate(appraisal_id, b.rating).await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({ "status": "calibrated" }))).into_response(),
        Err(e) => err_response(e),
    }
}

async fn finalise_appraisal(
    State(svc): State<Arc<PerformanceWriteService>>,
    _org: OrgContext,
    Path(appraisal_id): Path<Uuid>,
) -> axum::response::Response {
    match svc.finalise_appraisal(appraisal_id).await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({ "status": "finalized" }))).into_response(),
        Err(e) => err_response(e),
    }
}

// ── Goals ─────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct CreateGoalBody {
    employee_id: Uuid,
    #[serde(default)]
    cycle_id: Option<Uuid>,
    title: String,
    weight: rust_decimal::Decimal,
}

async fn create_goal(
    State(svc): State<Arc<PerformanceWriteService>>,
    _org: OrgContext,
    Json(b): Json<CreateGoalBody>,
) -> axum::response::Response {
    match svc
        .create_goal(NewGoal {
            employee_id: b.employee_id,
            cycle_id: b.cycle_id,
            title: b.title,
            weight: b.weight,
        })
        .await
    {
        Ok(id) => (StatusCode::CREATED, Json(IdResponse { id })).into_response(),
        Err(e) => err_response(e),
    }
}

#[derive(Debug, Deserialize)]
struct CloseGoalQuery {
    achieved: bool,
}

async fn close_goal(
    State(svc): State<Arc<PerformanceWriteService>>,
    _org: OrgContext,
    Path(goal_id): Path<Uuid>,
    axum::extract::Query(q): axum::extract::Query<CloseGoalQuery>,
) -> axum::response::Response {
    match svc.close_goal(goal_id, q.achieved).await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({ "ok": true }))).into_response(),
        Err(e) => err_response(e),
    }
}

// ── Composition ───────────────────────────────────────────────────────────

fn create_performance_verb_routes(svc: Arc<PerformanceWriteService>) -> Router {
    Router::new()
        .route("/cycles", post(create_cycle))
        .route("/cycles/:id/open", post(open_cycle))
        .route("/cycles/:id/calibration", post(enter_calibration))
        .route("/cycles/:id/close", post(close_cycle))
        .route("/cycles/:id/cancel", post(cancel_cycle))
        .route("/appraisals", post(create_appraisal))
        .route("/appraisals/self", post(submit_self))
        .route("/appraisals/manager", post(submit_manager))
        .route("/appraisals/:id/rate", post(rate_appraisal))
        .route("/appraisals/:id/finalise", post(finalise_appraisal))
        .route("/goals", post(create_goal))
        .route("/goals/:id/close", post(close_goal))
        .with_state(svc)
}

/// Mount the performance module with the finalised predicate fenced by
/// construction: every entity READ-ONLY through the generated GET surface,
/// and every state change through the write service's verbs.
pub fn create_guarded_performance_routes(m: &PerformanceModule) -> Router {
    use crate::presentation::http::create_appraisal_cycle_routes;

    Router::new()
        // Safe base: GET-only for all seven entities.
        .merge(m.readonly_routes())
        // Master data keeps generic writes (cycles, goals, feedback requests,
        // rewards, talent matrix entries — no cross-entity invariants beyond
        // the verb-checked ones; appraisals do NOT: their lifecycle is the
        // predicate's to guard).
        .merge(create_appraisal_cycle_routes(m.appraisal_cycle_service.clone()))
        // The state machine.
        .merge(create_performance_verb_routes(m.performance_write_service.clone()))
}

