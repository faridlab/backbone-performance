//! The module's extension (hand-written; ADR-0031): what the module adds beside its
//! generated services. The generated module and builder carry `ModuleExt` and
//! `ModuleBuilderExt` and dereference to them, so their fields read as `module.field`.

#[allow(unused_imports)]
use super::*;

/// State the module adds; read through the generated module's `Deref`.
pub struct ModuleExt {
    /// The validated write engine: cycles, goals, appraisals. Generic CRUD
    /// on an appraisal row bypasses the finalised predicate — use this for
    /// every state change.
    pub performance_write_service: Arc<application::service::PerformanceWriteService>,
}

/// State the builder adds.
pub struct ModuleBuilderExt {
}

impl Default for ModuleBuilderExt {
    fn default() -> Self {
        Self {
        }
    }
}

impl ModuleBuilderExt {
    /// Build the extension's state from what the generated build made.
    #[allow(unused_variables, clippy::redundant_clone)]
    pub(crate) fn build(self, parts: &ModuleParts<'_>) -> anyhow::Result<ModuleExt> {
        let db_pool = parts.db_pool.clone();
        // The validated write engine, self-constructed from the pool.
        let performance_write_service =
            Arc::new(application::service::PerformanceWriteService::new(db_pool.clone()));
        Ok(ModuleExt {
            performance_write_service,
        })
    }
}
