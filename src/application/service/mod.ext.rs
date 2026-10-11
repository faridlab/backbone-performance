// This directory's extension (hand-written; ADR-0031). The generated mod.rs (lib.rs for
// src/) pulls it in with include!, so its lines resolve against this directory.

// The validated write path: cycles, goals, appraisals with the ONE
// finalised predicate.
pub mod performance_write_service;
pub mod performance_events;
pub use performance_write_service::{
    NewAppraisal, NewCycle, NewGoal, PerformanceError, PerformanceWriteService,
};
pub use performance_events::{LoggingSink as PerformanceLoggingSink, PerformanceEvent, PerformanceEventSink};
