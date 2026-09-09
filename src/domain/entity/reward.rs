use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::AuditMetadata;
use super::RewardType;

/// Strongly-typed ID for Reward
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RewardId(pub Uuid);

impl RewardId {
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

impl std::fmt::Display for RewardId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for RewardId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for RewardId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<RewardId> for Uuid {
    fn from(id: RewardId) -> Self {
        id.0
    }
}

impl AsRef<Uuid> for RewardId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl std::ops::Deref for RewardId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Reward {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub cycle_id: Option<Uuid>,
    pub reward_type: RewardType,
    pub title: String,
    pub description: Option<String>,
    pub amount: Option<Decimal>,
    pub awarded_by: Option<Uuid>,
    pub awarded_at: NaiveDate,
    pub payroll_component_id: Option<Uuid>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl Reward {
    /// Create a builder for Reward
    pub fn builder() -> RewardBuilder {
        <RewardBuilder as Default>::default()
    }

    /// Create a new Reward with required fields
    pub fn new(
        employee_id: Uuid,
        reward_type: RewardType,
        title: String,
        awarded_at: NaiveDate,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            employee_id,
            cycle_id: None,
            reward_type,
            title,
            description: None,
            amount: None,
            awarded_by: None,
            awarded_at,
            payroll_component_id: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> RewardId {
        RewardId(self.id)
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

    /// Set the description field (chainable)
    pub fn with_description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the amount field (chainable)
    pub fn with_amount(mut self, value: Decimal) -> Self {
        self.amount = Some(value);
        self
    }

    /// Set the awarded_by field (chainable)
    pub fn with_awarded_by(mut self, value: Uuid) -> Self {
        self.awarded_by = Some(value);
        self
    }

    /// Set the payroll_component_id field (chainable)
    pub fn with_payroll_component_id(mut self, value: Uuid) -> Self {
        self.payroll_component_id = Some(value);
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
                "reward_type" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.reward_type = v;
                    }
                }
                "title" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.title = v;
                    }
                }
                "description" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.description = v;
                    }
                }
                "amount" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.amount = v;
                    }
                }
                "awarded_by" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.awarded_by = v;
                    }
                }
                "awarded_at" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.awarded_at = v;
                    }
                }
                "payroll_component_id" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.payroll_component_id = v;
                    }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for Reward {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "Reward"
    }
}

impl backbone_core::PersistentEntity for Reward {
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

impl backbone_orm::EntityRepoMeta for Reward {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("employee_id".to_string(), "uuid".to_string());
        m.insert("cycle_id".to_string(), "uuid".to_string());
        m.insert("payroll_component_id".to_string(), "uuid".to_string());
        m.insert("reward_type".to_string(), "reward_type".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["title"]
    }
}

/// Builder for Reward entity
///
/// Provides a fluent API for constructing Reward instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct RewardBuilder {
    employee_id: Option<Uuid>,
    cycle_id: Option<Uuid>,
    reward_type: Option<RewardType>,
    title: Option<String>,
    description: Option<String>,
    amount: Option<Decimal>,
    awarded_by: Option<Uuid>,
    awarded_at: Option<NaiveDate>,
    payroll_component_id: Option<Uuid>,
}

impl RewardBuilder {
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

    /// Set the reward_type field (default: `RewardType::default()`)
    pub fn reward_type(mut self, value: RewardType) -> Self {
        self.reward_type = Some(value);
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

    /// Set the amount field (optional)
    pub fn amount(mut self, value: Decimal) -> Self {
        self.amount = Some(value);
        self
    }

    /// Set the awarded_by field (optional)
    pub fn awarded_by(mut self, value: Uuid) -> Self {
        self.awarded_by = Some(value);
        self
    }

    /// Set the awarded_at field (required)
    pub fn awarded_at(mut self, value: NaiveDate) -> Self {
        self.awarded_at = Some(value);
        self
    }

    /// Set the payroll_component_id field (optional)
    pub fn payroll_component_id(mut self, value: Uuid) -> Self {
        self.payroll_component_id = Some(value);
        self
    }

    /// Build the Reward entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<Reward, String> {
        let employee_id = self
            .employee_id
            .ok_or_else(|| "employee_id is required".to_string())?;
        let title = self.title.ok_or_else(|| "title is required".to_string())?;
        let awarded_at = self
            .awarded_at
            .ok_or_else(|| "awarded_at is required".to_string())?;

        Ok(Reward {
            id: Uuid::new_v4(),
            employee_id,
            cycle_id: self.cycle_id,
            reward_type: self.reward_type.unwrap_or_default(),
            title,
            description: self.description,
            amount: self.amount,
            awarded_by: self.awarded_by,
            awarded_at,
            payroll_component_id: self.payroll_component_id,
            metadata: AuditMetadata::default(),
        })
    }
}
