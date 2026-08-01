use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "reward_type", rename_all = "snake_case")]
pub enum RewardType {
    Recognition,
    Bonus,
    Gift,
    LeaveCredit,
    Certificate,
}

impl std::fmt::Display for RewardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Recognition => write!(f, "recognition"),
            Self::Bonus => write!(f, "bonus"),
            Self::Gift => write!(f, "gift"),
            Self::LeaveCredit => write!(f, "leave_credit"),
            Self::Certificate => write!(f, "certificate"),
        }
    }
}

impl FromStr for RewardType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "recognition" => Ok(Self::Recognition),
            "bonus" => Ok(Self::Bonus),
            "gift" => Ok(Self::Gift),
            "leave_credit" => Ok(Self::LeaveCredit),
            "certificate" => Ok(Self::Certificate),
            _ => Err(format!("Unknown RewardType variant: {}", s)),
        }
    }
}

impl Default for RewardType {
    fn default() -> Self {
        Self::Recognition
    }
}
