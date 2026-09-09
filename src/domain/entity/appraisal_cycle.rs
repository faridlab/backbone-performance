use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::AuditMetadata;
use super::CycleStatus;

/// Strongly-typed ID for AppraisalCycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AppraisalCycleId(pub Uuid);

impl AppraisalCycleId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl std::fmt::Display for AppraisalCycleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for AppraisalCycleId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for AppraisalCycleId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<AppraisalCycleId> for Uuid {
    fn from(id: AppraisalCycleId) -> Self {
        id.0
    }
}

impl AsRef<Uuid> for AppraisalCycleId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl std::ops::Deref for AppraisalCycleId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AppraisalCycle {
    pub id: Uuid,
    pub name: String,
    pub cycle_type: Option<String>,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub status: CycleStatus,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl AppraisalCycle {
    /// Create a builder for AppraisalCycle
    pub fn builder() -> AppraisalCycleBuilder {
        <AppraisalCycleBuilder as Default>::default()
    }

    /// Create a new AppraisalCycle with required fields
    pub fn new(
        name: String,
        period_start: NaiveDate,
        period_end: NaiveDate,
        status: CycleStatus,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            cycle_type: None,
            period_start,
            period_end,
            status,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> AppraisalCycleId {
        AppraisalCycleId(self.id)
    }

    /// Get when this entity was created
    pub fn created_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.created_at.as_ref()
    }

    /// Get when this entity was last updated
    pub fn updated_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.updated_at.as_ref()
    }

    /// Check if this entity is soft deleted
    pub fn is_deleted(&self) -> bool {
        self.metadata.deleted_at.is_some()
    }

    /// Check if this entity is active (not deleted)
    pub fn is_active(&self) -> bool {
        self.metadata.deleted_at.is_none()
    }

    /// Get when this entity was deleted
    pub fn deleted_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.deleted_at.as_ref()
    }

    /// Get who created this entity
    pub fn created_by(&self) -> Option<&Uuid> {
        self.metadata.created_by.as_ref()
    }

    /// Get who last updated this entity
    pub fn updated_by(&self) -> Option<&Uuid> {
        self.metadata.updated_by.as_ref()
    }

    /// Get who deleted this entity
    pub fn deleted_by(&self) -> Option<&Uuid> {
        self.metadata.deleted_by.as_ref()
    }

    /// Get the current status
    pub fn status(&self) -> &CycleStatus {
        &self.status
    }

    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the cycle_type field (chainable)
    pub fn with_cycle_type(mut self, value: String) -> Self {
        self.cycle_type = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "name" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.name = v;
                    }
                }
                "cycle_type" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.cycle_type = v;
                    }
                }
                "period_start" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.period_start = v;
                    }
                }
                "period_end" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.period_end = v;
                    }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.status = v;
                    }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for AppraisalCycle {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "AppraisalCycle"
    }
}

impl backbone_core::PersistentEntity for AppraisalCycle {
    fn entity_id(&self) -> String {
        self.id.to_string()
    }
    fn set_entity_id(&mut self, id: String) {
        if let Ok(uuid) = uuid::Uuid::parse_str(&id) {
            self.id = uuid;
        }
    }
    fn created_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.created_at
    }
    fn set_created_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.created_at = Some(ts);
    }
    fn updated_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.updated_at
    }
    fn set_updated_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.updated_at = Some(ts);
    }
    fn deleted_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.deleted_at
    }
    fn set_deleted_at(&mut self, ts: Option<chrono::DateTime<chrono::Utc>>) {
        self.metadata.deleted_at = ts;
    }
}

impl backbone_orm::EntityRepoMeta for AppraisalCycle {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "cycle_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["name"]
    }
}

/// Builder for AppraisalCycle entity
///
/// Provides a fluent API for constructing AppraisalCycle instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct AppraisalCycleBuilder {
    name: Option<String>,
    cycle_type: Option<String>,
    period_start: Option<NaiveDate>,
    period_end: Option<NaiveDate>,
    status: Option<CycleStatus>,
}

impl AppraisalCycleBuilder {
    /// Set the name field (required)
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    /// Set the cycle_type field (optional)
    pub fn cycle_type(mut self, value: String) -> Self {
        self.cycle_type = Some(value);
        self
    }

    /// Set the period_start field (required)
    pub fn period_start(mut self, value: NaiveDate) -> Self {
        self.period_start = Some(value);
        self
    }

    /// Set the period_end field (required)
    pub fn period_end(mut self, value: NaiveDate) -> Self {
        self.period_end = Some(value);
        self
    }

    /// Set the status field (default: `CycleStatus::default()`)
    pub fn status(mut self, value: CycleStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Build the AppraisalCycle entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<AppraisalCycle, String> {
        let name = self.name.ok_or_else(|| "name is required".to_string())?;
        let period_start = self
            .period_start
            .ok_or_else(|| "period_start is required".to_string())?;
        let period_end = self
            .period_end
            .ok_or_else(|| "period_end is required".to_string())?;

        Ok(AppraisalCycle {
            id: Uuid::new_v4(),
            name,
            cycle_type: self.cycle_type,
            period_start,
            period_end,
            status: self.status.unwrap_or_default(),
            metadata: AuditMetadata::default(),
        })
    }
}
