use tauri::{command, AppHandle, Manager};
use tracing::info;

use super::OperationResult;
use crate::config::{AppState, ConfigManager};
use crate::state::{build_dashboard_state, DashboardState};

#[command]
pub async fn get_dashboard_state(app: AppHandle) -> OperationResult<DashboardState> {
    info!("Handling get_dashboard_state command");
    if let Some(state) = app.try_state::<AppState>() {
        let config = state.get_config();
        let dash_state = build_dashboard_state(&config);
        return OperationResult::ok(dash_state);
    }

    match ConfigManager::init_default() {
        Ok(manager) => match manager.load_or_create() {
            Ok(config) => {
                let state = build_dashboard_state(&config);
                OperationResult::ok(state)
            }
            Err(e) => OperationResult::err(format!("Failed to load configuration: {}", e)),
        },
        Err(e) => OperationResult::err(format!("Config manager initialization failed: {}", e)),
    }
}

#[command]
pub async fn sync_theme_state(app: AppHandle) -> OperationResult<DashboardState> {
    info!("Handling sync_theme_state command");
    get_dashboard_state(app).await
}
