pub mod apps;
pub mod automation;
pub mod conditions;
pub mod dashboard;
pub mod personalization;
pub mod schedule;
pub mod settings;
pub mod shortcuts;
pub mod theme;
pub mod tray;
pub mod updater;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> OperationResult<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }
}
