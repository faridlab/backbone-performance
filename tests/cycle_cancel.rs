//! Cancelling an appraisal cycle over real Postgres, on the database shape the
//! module's own migrations build.
//!
//! A private scratch database per test with every migration in `migrations/`
//! applied in order, and a SKIP (not a failure) when no database is reachable.
//! Set `DATABASE_URL` to run it for real; `PERFORMANCE_REQUIRE_DB=1` turns a
//! skip into a failure.

use backbone_performance::application::service::{NewCycle, PerformanceWriteService};
use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

fn db_required() -> bool {
    std::env::var("PERFORMANCE_REQUIRE_DB").map(|v| v != "0").unwrap_or(false)
}

/// A fresh scratch database carrying the module's migrations.
async fn scratch(suffix: &str) -> Option<PgPool> {
    let url = std::env::var("DATABASE_URL").ok().or_else(|| {
        assert!(!db_required(), "a database is required");
        None
    })?;
    let (prefix, _) = url.trim_end_matches('/').rsplit_once('/')?;
    let admin = match PgPool::connect(&format!("{prefix}/postgres")).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("skip: no admin connection ({e})");
            assert!(!db_required(), "a database is required");
            return None;
        }
    };
    let name = format!("performance_cycle_cancel_{suffix}");
    let _ = sqlx::query(&format!(r#"DROP DATABASE IF EXISTS "{name}" WITH (FORCE)"#))
        .execute(&admin)
        .await;
    sqlx::query(&format!(r#"CREATE DATABASE "{name}""#))
        .execute(&admin)
        .await
        .expect("create scratch database");
    admin.close().await;
    let pool = PgPool::connect(&format!("{prefix}/{name}")).await.expect("connect scratch");

    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/migrations");
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .expect("read migrations")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.to_string_lossy().ends_with(".up.sql"))
        .collect();
    files.sort();
    for file in files {
        let sql = std::fs::read_to_string(&file).unwrap();
        sqlx::raw_sql(&sql)
            .execute(&pool)
            .await
            .unwrap_or_else(|e| panic!("apply {}: {e}", file.display()));
    }
    Some(pool)
}

async fn status_of(pool: &PgPool, id: Uuid) -> String {
    sqlx::query_scalar("SELECT status::text FROM performance.appraisal_cycles WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap()
}

/// A cycle can be cancelled from draft and from open, cancelling again is a
/// no-op, and a closed cycle refuses.
#[tokio::test]
async fn a_cycle_is_cancelled_before_it_closes() {
    let Some(pool) = scratch("db").await else { return };
    let svc = PerformanceWriteService::new(pool.clone());
    let new_cycle = |name: &str| NewCycle {
        name: name.into(),
        cycle_type: None,
        period_start: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
        period_end: NaiveDate::from_ymd_opt(2026, 6, 30).unwrap(),
    };

    let draft = svc.create_cycle(new_cycle("H1 draft")).await.expect("create");
    svc.cancel_cycle(draft).await.expect("cancel a draft cycle");
    assert_eq!(status_of(&pool, draft).await, "cancelled");
    svc.cancel_cycle(draft).await.expect("cancelling again is a no-op");

    let open = svc.create_cycle(new_cycle("H1 open")).await.expect("create");
    svc.open_cycle(open).await.expect("open");
    svc.cancel_cycle(open).await.expect("cancel an open cycle");
    assert_eq!(status_of(&pool, open).await, "cancelled");

    let closed = svc.create_cycle(new_cycle("H1 closed")).await.expect("create");
    sqlx::query("UPDATE performance.appraisal_cycles SET status = 'closed' WHERE id = $1")
        .bind(closed)
        .execute(&pool)
        .await
        .unwrap();
    assert!(svc.cancel_cycle(closed).await.is_err(), "a closed cycle is history");
}
