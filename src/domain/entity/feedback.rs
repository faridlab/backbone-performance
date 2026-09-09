use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::AuditMetadata;
use super::FeedbackRelationship;

/// Strongly-typed ID for Feedback
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FeedbackId(pub Uuid);

impl FeedbackId {
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

impl std::fmt::Display for FeedbackId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for FeedbackId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for FeedbackId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<FeedbackId> for Uuid {
    fn from(id: FeedbackId) -> Self {
        id.0
    }
}

impl AsRef<Uuid> for FeedbackId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl std::ops::Deref for FeedbackId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Feedback {
    pub id: Uuid,
    pub cycle_id: Option<Uuid>,
    pub from_employee_id: Uuid,
    pub to_employee_id: Uuid,
    pub content: String,
    pub is_anonymous: bool,
    pub relationship: FeedbackRelationship,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl Feedback {
    /// Create a builder for Feedback
    pub fn builder() -> FeedbackBuilder {
        <FeedbackBuilder as Default>::default()
    }

    /// Create a new Feedback with required fields
    pub fn new(
        from_employee_id: Uuid,
        to_employee_id: Uuid,
        content: String,
        is_anonymous: bool,
        relationship: FeedbackRelationship,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            cycle_id: None,
            from_employee_id,
            to_employee_id,
            content,
            is_anonymous,
            relationship,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> FeedbackId {
        FeedbackId(self.id)
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

    /// Set the cycle_id field (chainable)
    pub fn with_cycle_id(mut self, value: Uuid) -> Self {
        self.cycle_id = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "cycle_id" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.cycle_id = v;
                    }
                }
                "from_employee_id" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.from_employee_id = v;
                    }
                }
                "to_employee_id" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.to_employee_id = v;
                    }
                }
                "content" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.content = v;
                    }
                }
                "is_anonymous" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.is_anonymous = v;
                    }
                }
                "relationship" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.relationship = v;
                    }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for Feedback {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "Feedback"
    }
}

impl backbone_core::PersistentEntity for Feedback {
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

impl backbone_orm::EntityRepoMeta for Feedback {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("cycle_id".to_string(), "uuid".to_string());
        m.insert("from_employee_id".to_string(), "uuid".to_string());
        m.insert("to_employee_id".to_string(), "uuid".to_string());
        m.insert(
            "relationship".to_string(),
            "feedback_relationship".to_string(),
        );
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["content"]
    }
}

/// Builder for Feedback entity
///
/// Provides a fluent API for constructing Feedback instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct FeedbackBuilder {
    cycle_id: Option<Uuid>,
    from_employee_id: Option<Uuid>,
    to_employee_id: Option<Uuid>,
    content: Option<String>,
    is_anonymous: Option<bool>,
    relationship: Option<FeedbackRelationship>,
}

impl FeedbackBuilder {
    /// Set the cycle_id field (optional)
    pub fn cycle_id(mut self, value: Uuid) -> Self {
        self.cycle_id = Some(value);
        self
    }

    /// Set the from_employee_id field (required)
    pub fn from_employee_id(mut self, value: Uuid) -> Self {
        self.from_employee_id = Some(value);
        self
    }

    /// Set the to_employee_id field (required)
    pub fn to_employee_id(mut self, value: Uuid) -> Self {
        self.to_employee_id = Some(value);
        self
    }

    /// Set the content field (required)
    pub fn content(mut self, value: String) -> Self {
        self.content = Some(value);
        self
    }

    /// Set the is_anonymous field (default: `false`)
    pub fn is_anonymous(mut self, value: bool) -> Self {
        self.is_anonymous = Some(value);
        self
    }

    /// Set the relationship field (default: `FeedbackRelationship::default()`)
    pub fn relationship(mut self, value: FeedbackRelationship) -> Self {
        self.relationship = Some(value);
        self
    }

    /// Build the Feedback entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<Feedback, String> {
        let from_employee_id = self
            .from_employee_id
            .ok_or_else(|| "from_employee_id is required".to_string())?;
        let to_employee_id = self
            .to_employee_id
            .ok_or_else(|| "to_employee_id is required".to_string())?;
        let content = self
            .content
            .ok_or_else(|| "content is required".to_string())?;

        Ok(Feedback {
            id: Uuid::new_v4(),
            cycle_id: self.cycle_id,
            from_employee_id,
            to_employee_id,
            content,
            is_anonymous: self.is_anonymous.unwrap_or(false),
            relationship: self.relationship.unwrap_or_default(),
            metadata: AuditMetadata::default(),
        })
    }
}
