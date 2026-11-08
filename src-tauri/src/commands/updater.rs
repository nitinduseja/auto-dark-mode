use tauri::{AppHandle, Runtime};
use tracing::info;
use crate::windows::updater;

#[tauri::command]
pub async fn download_update<R: Runtime>(app: AppHandle<R>, url: String) -> Result<(), String> {
    info!("download_update command invoked with url: {}", url);
    tokio::task::spawn_blocking(move || {
        updater::download_update_binary(&app, &url)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

#[tauri::command]
pub async fn install_and_restart() -> Result<(), String> {
    info!("install_and_restart command invoked");
    tokio::task::spawn_blocking(move || {
        updater::install_update_and_restart()
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}
