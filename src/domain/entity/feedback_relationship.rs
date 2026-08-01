use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "feedback_relationship", rename_all = "snake_case")]
pub enum FeedbackRelationship {
    Peer,
    Manager,
    DirectReport,
    CrossFunctional,
}

impl std::fmt::Display for FeedbackRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Peer => write!(f, "peer"),
            Self::Manager => write!(f, "manager"),
            Self::DirectReport => write!(f, "direct_report"),
            Self::CrossFunctional => write!(f, "cross_functional"),
        }
    }
}

impl FromStr for FeedbackRelationship {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "peer" => Ok(Self::Peer),
            "manager" => Ok(Self::Manager),
            "direct_report" => Ok(Self::DirectReport),
            "cross_functional" => Ok(Self::CrossFunctional),
            _ => Err(format!("Unknown FeedbackRelationship variant: {}", s)),
        }
    }
}

impl Default for FeedbackRelationship {
    fn default() -> Self {
        Self::Peer
    }
}
