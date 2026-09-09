use super::AuditMetadata;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Strongly-typed ID for TalentMatrixEntry
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TalentMatrixEntryId(pub Uuid);

impl TalentMatrixEntryId {
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

impl std::fmt::Display for TalentMatrixEntryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for TalentMatrixEntryId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for TalentMatrixEntryId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<TalentMatrixEntryId> for Uuid {
    fn from(id: TalentMatrixEntryId) -> Self {
        id.0
    }
}

impl AsRef<Uuid> for TalentMatrixEntryId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl std::ops::Deref for TalentMatrixEntryId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TalentMatrixEntry {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub cycle_id: Uuid,
    pub performance_score: i32,
    pub potential_score: i32,
    pub box_label: Option<String>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl TalentMatrixEntry {
    /// Create a builder for TalentMatrixEntry
    pub fn builder() -> TalentMatrixEntryBuilder {
        <TalentMatrixEntryBuilder as Default>::default()
    }

    /// Create a new TalentMatrixEntry with required fields
    pub fn new(
        employee_id: Uuid,
        cycle_id: Uuid,
        performance_score: i32,
        potential_score: i32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            employee_id,
            cycle_id,
            performance_score,
            potential_score,
            box_label: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> TalentMatrixEntryId {
        TalentMatrixEntryId(self.id)
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

    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the box_label field (chainable)
    pub fn with_box_label(mut self, value: String) -> Self {
        self.box_label = Some(value);
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
                "performance_score" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.performance_score = v;
                    }
                }
                "potential_score" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.potential_score = v;
                    }
                }
                "box_label" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.box_label = v;
                    }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for TalentMatrixEntry {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "TalentMatrixEntry"
    }
}

impl backbone_core::PersistentEntity for TalentMatrixEntry {
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

impl backbone_orm::EntityRepoMeta for TalentMatrixEntry {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("employee_id".to_string(), "uuid".to_string());
        m.insert("cycle_id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for TalentMatrixEntry entity
///
/// Provides a fluent API for constructing TalentMatrixEntry instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct TalentMatrixEntryBuilder {
    employee_id: Option<Uuid>,
    cycle_id: Option<Uuid>,
    performance_score: Option<i32>,
    potential_score: Option<i32>,
    box_label: Option<String>,
}

impl TalentMatrixEntryBuilder {
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

    /// Set the performance_score field (required)
    pub fn performance_score(mut self, value: i32) -> Self {
        self.performance_score = Some(value);
        self
    }

    /// Set the potential_score field (required)
    pub fn potential_score(mut self, value: i32) -> Self {
        self.potential_score = Some(value);
        self
    }

    /// Set the box_label field (optional)
    pub fn box_label(mut self, value: String) -> Self {
        self.box_label = Some(value);
        self
    }

    /// Build the TalentMatrixEntry entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<TalentMatrixEntry, String> {
        let employee_id = self
            .employee_id
            .ok_or_else(|| "employee_id is required".to_string())?;
        let cycle_id = self
            .cycle_id
            .ok_or_else(|| "cycle_id is required".to_string())?;
        let performance_score = self
            .performance_score
            .ok_or_else(|| "performance_score is required".to_string())?;
        let potential_score = self
            .potential_score
            .ok_or_else(|| "potential_score is required".to_string())?;

        Ok(TalentMatrixEntry {
            id: Uuid::new_v4(),
            employee_id,
            cycle_id,
            performance_score,
            potential_score,
            box_label: self.box_label,
            metadata: AuditMetadata::default(),
        })
    }
}
