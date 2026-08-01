use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use rust_decimal::Decimal;

use super::GoalStatus;
use super::AuditMetadata;

/// Strongly-typed ID for Goal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GoalId(pub Uuid);

impl GoalId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for GoalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for GoalId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for GoalId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<GoalId> for Uuid {
    fn from(id: GoalId) -> Self { id.0 }
}

impl AsRef<Uuid> for GoalId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for GoalId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Goal {
    pub id: Uuid,
    pub company_id: Uuid,
    pub employee_id: Uuid,
    pub cycle_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub weight: Option<Decimal>,
    pub progress: Option<Decimal>,
    pub parent_goal_id: Option<Uuid>,
    pub status: GoalStatus,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl Goal {
    /// Create a builder for Goal
    pub fn builder() -> GoalBuilder {
        GoalBuilder::default()
    }

    /// Create a new Goal with required fields
    pub fn new(company_id: Uuid, employee_id: Uuid, title: String, status: GoalStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            company_id,
            employee_id,
            cycle_id: None,
            title,
            description: None,
            weight: None,
            progress: None,
            parent_goal_id: None,
            status,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> GoalId {
        GoalId(self.id)
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
    pub fn status(&self) -> &GoalStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the cycle_id field (chainable)
    pub fn with_cycle_id(mut self, value: Uuid) -> Self {
        self.cycle_id = Some(value);
        self
    }

    /// Set the description field (chainable)
    pub fn with_description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the weight field (chainable)
    pub fn with_weight(mut self, value: Decimal) -> Self {
        self.weight = Some(value);
        self
    }

    /// Set the progress field (chainable)
    pub fn with_progress(mut self, value: Decimal) -> Self {
        self.progress = Some(value);
        self
    }

    /// Set the parent_goal_id field (chainable)
    pub fn with_parent_goal_id(mut self, value: Uuid) -> Self {
        self.parent_goal_id = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "company_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.company_id = v; }
                }
                "employee_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.employee_id = v; }
                }
                "cycle_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.cycle_id = v; }
                }
                "title" => {
                    if let Ok(v) = serde_json::from_value(value) { self.title = v; }
                }
                "description" => {
                    if let Ok(v) = serde_json::from_value(value) { self.description = v; }
                }
                "weight" => {
                    if let Ok(v) = serde_json::from_value(value) { self.weight = v; }
                }
                "progress" => {
                    if let Ok(v) = serde_json::from_value(value) { self.progress = v; }
                }
                "parent_goal_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.parent_goal_id = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for Goal {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "Goal"
    }
}

impl backbone_core::PersistentEntity for Goal {
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

impl backbone_orm::EntityRepoMeta for Goal {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("company_id".to_string(), "uuid".to_string());
        m.insert("employee_id".to_string(), "uuid".to_string());
        m.insert("cycle_id".to_string(), "uuid".to_string());
        m.insert("parent_goal_id".to_string(), "uuid".to_string());
        m.insert("status".to_string(), "goal_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["title"]
    }
    fn company_field() -> Option<&'static str> {
        Some("company_id")
    }
}

/// Builder for Goal entity
///
/// Provides a fluent API for constructing Goal instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct GoalBuilder {
    company_id: Option<Uuid>,
    employee_id: Option<Uuid>,
    cycle_id: Option<Uuid>,
    title: Option<String>,
    description: Option<String>,
    weight: Option<Decimal>,
    progress: Option<Decimal>,
    parent_goal_id: Option<Uuid>,
    status: Option<GoalStatus>,
}

impl GoalBuilder {
    /// Set the company_id field (required)
    pub fn company_id(mut self, value: Uuid) -> Self {
        self.company_id = Some(value);
        self
    }

    /// Set the employee_id field (required)
    pub fn employee_id(mut self, value: Uuid) -> Self {
        self.employee_id = Some(value);
        self
    }

    /// Set the cycle_id field (optional)
    pub fn cycle_id(mut self, value: Uuid) -> Self {
        self.cycle_id = Some(value);
        self
    }

    /// Set the title field (required)
    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    /// Set the description field (optional)
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the weight field (optional)
    pub fn weight(mut self, value: Decimal) -> Self {
        self.weight = Some(value);
        self
    }

    /// Set the progress field (optional)
    pub fn progress(mut self, value: Decimal) -> Self {
        self.progress = Some(value);
        self
    }

    /// Set the parent_goal_id field (optional)
    pub fn parent_goal_id(mut self, value: Uuid) -> Self {
        self.parent_goal_id = Some(value);
        self
    }

    /// Set the status field (default: `GoalStatus::default()`)
    pub fn status(mut self, value: GoalStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Build the Goal entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<Goal, String> {
        let company_id = self.company_id.ok_or_else(|| "company_id is required".to_string())?;
        let employee_id = self.employee_id.ok_or_else(|| "employee_id is required".to_string())?;
        let title = self.title.ok_or_else(|| "title is required".to_string())?;

        Ok(Goal {
            id: Uuid::new_v4(),
            company_id,
            employee_id,
            cycle_id: self.cycle_id,
            title,
            description: self.description,
            weight: self.weight,
            progress: self.progress,
            parent_goal_id: self.parent_goal_id,
            status: self.status.unwrap_or(GoalStatus::default()),
            metadata: AuditMetadata::default(),
        })
    }
}
