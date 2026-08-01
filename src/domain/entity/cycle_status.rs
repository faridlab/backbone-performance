use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "cycle_status", rename_all = "snake_case")]
pub enum CycleStatus {
    Draft,
    Open,
    Calibration,
    Closed,
}

impl std::fmt::Display for CycleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Open => write!(f, "open"),
            Self::Calibration => write!(f, "calibration"),
            Self::Closed => write!(f, "closed"),
        }
    }
}

impl FromStr for CycleStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "draft" => Ok(Self::Draft),
            "open" => Ok(Self::Open),
            "calibration" => Ok(Self::Calibration),
            "closed" => Ok(Self::Closed),
            _ => Err(format!("Unknown CycleStatus variant: {}", s)),
        }
    }
}

impl Default for CycleStatus {
    fn default() -> Self {
        Self::Draft
    }
}
