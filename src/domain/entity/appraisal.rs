use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::AppraisalStatus;
use super::AuditMetadata;

/// Strongly-typed ID for Appraisal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AppraisalId(pub Uuid);

impl AppraisalId {
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

impl std::fmt::Display for AppraisalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for AppraisalId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for AppraisalId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<AppraisalId> for Uuid {
    fn from(id: AppraisalId) -> Self {
        id.0
    }
}

impl AsRef<Uuid> for AppraisalId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl std::ops::Deref for AppraisalId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Appraisal {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub cycle_id: Uuid,
    pub reviewer_id: Uuid,
    pub status: AppraisalStatus,
    pub overall_rating: Option<Decimal>,
    pub submitted_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl Appraisal {
    /// Create a builder for Appraisal
    pub fn builder() -> AppraisalBuilder {
        <AppraisalBuilder as Default>::default()
    }

    /// Create a new Appraisal with required fields
    pub fn new(
        employee_id: Uuid,
        cycle_id: Uuid,
        reviewer_id: Uuid,
        status: AppraisalStatus,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            employee_id,
            cycle_id,
            reviewer_id,
            status,
            overall_rating: None,
            submitted_at: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> AppraisalId {
        AppraisalId(self.id)
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
    pub fn status(&self) -> &AppraisalStatus {
        &self.status
    }

    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the overall_rating field (chainable)
    pub fn with_overall_rating(mut self, value: Decimal) -> Self {
        self.overall_rating = Some(value);
        self
    }

    /// Set the submitted_at field (chainable)
    pub fn with_submitted_at(mut self, value: DateTime<Utc>) -> Self {
        self.submitted_at = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "employee_id" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.employee_id = v;
                    }
                }
                "cycle_id" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.cycle_id = v;
                    }
                }
                "reviewer_id" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.reviewer_id = v;
                    }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.status = v;
                    }
                }
                "overall_rating" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.overall_rating = v;
                    }
                }
                "submitted_at" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.submitted_at = v;
                    }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for Appraisal {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "Appraisal"
    }
}

impl backbone_core::PersistentEntity for Appraisal {
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

impl backbone_orm::EntityRepoMeta for Appraisal {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("employee_id".to_string(), "uuid".to_string());
        m.insert("cycle_id".to_string(), "uuid".to_string());
        m.insert("reviewer_id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "appraisal_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for Appraisal entity
///
/// Provides a fluent API for constructing Appraisal instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct AppraisalBuilder {
    employee_id: Option<Uuid>,
    cycle_id: Option<Uuid>,
    reviewer_id: Option<Uuid>,
    status: Option<AppraisalStatus>,
    overall_rating: Option<Decimal>,
    submitted_at: Option<DateTime<Utc>>,
}

impl AppraisalBuilder {
    /// Set the employee_id field (required)
    pub fn employee_id(mut self, value: Uuid) -> Self {
        self.employee_id = Some(value);
        self
    }

    /// Set the cycle_id field (required)
    pub fn cycle_id(mut self, value: Uuid) -> Self {
        self.cycle_id = Some(value);
        self
    }

    /// Set the reviewer_id field (required)
    pub fn reviewer_id(mut self, value: Uuid) -> Self {
        self.reviewer_id = Some(value);
        self
    }

    /// Set the status field (default: `AppraisalStatus::default()`)
    pub fn status(mut self, value: AppraisalStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the overall_rating field (optional)
    pub fn overall_rating(mut self, value: Decimal) -> Self {
        self.overall_rating = Some(value);
        self
    }

    /// Set the submitted_at field (optional)
    pub fn submitted_at(mut self, value: DateTime<Utc>) -> Self {
        self.submitted_at = Some(value);
        self
    }

    /// Build the Appraisal entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<Appraisal, String> {
        let employee_id = self
            .employee_id
            .ok_or_else(|| "employee_id is required".to_string())?;
        let cycle_id = self
            .cycle_id
            .ok_or_else(|| "cycle_id is required".to_string())?;
        let reviewer_id = self
            .reviewer_id
            .ok_or_else(|| "reviewer_id is required".to_string())?;

        Ok(Appraisal {
            id: Uuid::new_v4(),
            employee_id,
            cycle_id,
            reviewer_id,
            status: self.status.unwrap_or_default(),
            overall_rating: self.overall_rating,
            submitted_at: self.submitted_at,
            metadata: AuditMetadata::default(),
        })
    }
}
