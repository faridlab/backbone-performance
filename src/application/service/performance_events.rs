//! The performance event port: what the module tells the world, and the seam
//! the composing service implements to deliver it (the #558 producer's arms
//! and the promotion validation).

use rust_decimal::Decimal;
use uuid::Uuid;

/// What the module announces.
#[derive(Debug, Clone)]
pub enum PerformanceEvent {
    /// An appraisal was individually finalised (rating immutable).
    AppraisalFinalised {
        appraisal_id: Uuid,
        employee_id: Uuid,
        rating: Option<Decimal>,
    },
    /// A cycle closed — every appraisal finalised (the strong predicate).
    CycleClosed {
        cycle_id: Uuid,
    },
}

/// The event sink port. The default logs.
pub trait PerformanceEventSink: Send + Sync {
    fn publish(&self, event: PerformanceEvent);
}

/// The default sink: logs, delivers nothing.
pub struct LoggingSink;

impl PerformanceEventSink for LoggingSink {
    fn publish(&self, event: PerformanceEvent) {
        match event {
            PerformanceEvent::AppraisalFinalised { appraisal_id, rating, .. } => {
                tracing::info!(
                    target: "performance",
                    appraisal_id = %appraisal_id,
                    ?rating,
                    "appraisal finalised (no event sink wired)"
                );
            }
            PerformanceEvent::CycleClosed { cycle_id } => {
                tracing::info!(
                    target: "performance",
                    cycle_id = %cycle_id,
                    "cycle closed (no event sink wired)"
                );
            }
        }
    }
}
