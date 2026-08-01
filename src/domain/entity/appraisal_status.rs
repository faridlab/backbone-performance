use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "appraisal_status", rename_all = "snake_case")]
pub enum AppraisalStatus {
    Draft,
    SelfReview,
    Submitted,
    Calibrated,
    Finalized,
}

impl std::fmt::Display for AppraisalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::SelfReview => write!(f, "self_review"),
            Self::Submitted => write!(f, "submitted"),
            Self::Calibrated => write!(f, "calibrated"),
            Self::Finalized => write!(f, "finalized"),
        }
    }
}

impl FromStr for AppraisalStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "draft" => Ok(Self::Draft),
            "self_review" => Ok(Self::SelfReview),
            "submitted" => Ok(Self::Submitted),
            "calibrated" => Ok(Self::Calibrated),
            "finalized" => Ok(Self::Finalized),
            _ => Err(format!("Unknown AppraisalStatus variant: {}", s)),
        }
    }
}

impl Default for AppraisalStatus {
    fn default() -> Self {
        Self::Draft
    }
}
