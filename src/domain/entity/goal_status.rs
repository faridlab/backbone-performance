use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "goal_status", rename_all = "snake_case")]
pub enum GoalStatus {
    Draft,
    Active,
    Achieved,
    Missed,
    Cancelled,
}

impl std::fmt::Display for GoalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Active => write!(f, "active"),
            Self::Achieved => write!(f, "achieved"),
            Self::Missed => write!(f, "missed"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl FromStr for GoalStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "draft" => Ok(Self::Draft),
            "active" => Ok(Self::Active),
            "achieved" => Ok(Self::Achieved),
            "missed" => Ok(Self::Missed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(format!("Unknown GoalStatus variant: {}", s)),
        }
    }
}

impl Default for GoalStatus {
    fn default() -> Self {
        Self::Draft
    }
}
