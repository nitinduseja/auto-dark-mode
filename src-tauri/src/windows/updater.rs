use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tauri::{AppHandle, Emitter, Runtime};
use tracing::{info, warn};

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct DownloadProgressPayload {
    pub percentage: i32,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
}

pub fn get_update_temp_path() -> PathBuf {
    std::env::temp_dir().join("autodarkmode_update.exe")
}

/// Downloads the update binary from the provided URL, reporting byte-by-byte progress via Tauri events.
pub fn download_update_binary<R: Runtime>(app: &AppHandle<R>, url: &str) -> Result<(), String> {
    let target_path = get_update_temp_path();
    if target_path.exists() {
        let _ = fs::remove_file(&target_path);
    }

    let target_str = target_path.to_string_lossy().to_string();
    info!("Starting in-app download: {} -> {:?}", url, target_path);

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        let sanitized_url = url.replace('\'', "''");
        let sanitized_target = target_str.replace('\'', "''");

        // PowerShell streaming download script with System.Net.Http.HttpClient
        let script = format!(
            r#"[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 -bor [Net.SecurityProtocolType]::Tls13;
Add-Type -AssemblyName System.Net.Http;
$handler = [System.Net.Http.HttpClientHandler]::new();
$handler.AllowAutoRedirect = $true;
$client = [System.Net.Http.HttpClient]::new($handler);
$client.DefaultRequestHeaders.UserAgent.ParseAdd('AutoDarkMode-InAppUpdater');
try {{
    $resp = $client.GetAsync('{sanitized_url}', [System.Net.Http.HttpCompletionOption]::ResponseHeadersRead).GetAwaiter().GetResult();
    if (-not $resp.IsSuccessStatusCode) {{
        [Console]::Error.WriteLine("HTTP Error: " + $resp.StatusCode);
        exit 1;
    }}
    $total = $resp.Content.Headers.ContentLength;
    if ($null -eq $total) {{ $total = 0; }}
    $stream = $resp.Content.ReadAsStreamAsync().GetAwaiter().GetResult();
    $file = [System.IO.File]::Create('{sanitized_target}');
    $buf = New-Object byte[] 65536;
    $read = 0;
    $received = 0;
    $lastPct = -1;
    while (($read = $stream.Read($buf, 0, $buf.Length)) -gt 0) {{
        $file.Write($buf, 0, $read);
        $received += $read;
        if ($total -gt 0) {{
            $pct = [int](($received / $total) * 100);
            if ($pct -ne $lastPct) {{
                $lastPct = $pct;
                [Console]::WriteLine("PROGRESS:" + $pct + ":" + $received + ":" + $total);
            }}
        }} else {{
            [Console]::WriteLine("PROGRESS:-1:" + $received + ":0");
        }}
    }}
    $file.Close();
    $stream.Close();
    $client.Dispose();
    [Console]::WriteLine("COMPLETED");
}} catch {{
    [Console]::Error.WriteLine("Download exception: " + $_.Exception.Message);
    exit 2;
}}"#
        );

        let mut cmd = Command::new("powershell.exe");
        cmd.args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            &script,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(CREATE_NO_WINDOW);

        let mut child = cmd.spawn().map_err(|e| format!("Failed to launch updater downloader: {}", e))?;
        let stdout = child.stdout.take().ok_or("Failed to attach to downloader output")?;
        let reader = BufReader::new(stdout);

        let mut received_completed = false;

        for line in reader.lines().map_while(Result::ok) {
            let trimmed = line.trim();
            if trimmed.starts_with("PROGRESS:") {
                let parts: Vec<&str> = trimmed.split(':').collect();
                if parts.len() >= 4 {
                    let pct: i32 = parts[1].parse().unwrap_or(0);
                    let downloaded: u64 = parts[2].parse().unwrap_or(0);
                    let total: u64 = parts[3].parse().unwrap_or(0);

                    let payload = DownloadProgressPayload {
                        percentage: pct,
                        downloaded_bytes: downloaded,
                        total_bytes: total,
                    };
                    let _ = app.emit("update-download-progress", payload);
                }
            } else if trimmed == "COMPLETED" {
                received_completed = true;
            }
        }

        let status = child.wait().map_err(|e| format!("Downloader error: {}", e))?;

        if !status.success() || !received_completed || !target_path.exists() {
            warn!("PowerShell download did not complete successfully. Trying curl.exe fallback...");
            let mut curl = Command::new("curl.exe");
            curl.args(["-L", "-f", "-sS", "-o", &target_str, url])
                .creation_flags(CREATE_NO_WINDOW);

            let curl_res = curl.status().map_err(|e| format!("Curl execution failed: {}", e))?;
            if !curl_res.success() {
                let _ = app.emit("update-download-error", "Failed to download update binary via both PowerShell and curl.".to_string());
                return Err("Failed to download update binary via both PowerShell and curl".to_string());
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = (app, &target_str);
        return Err("In-app update is only supported on Windows".to_string());
    }

    // Verify downloaded binary has valid size (> 100KB)
    let file_size = target_path.metadata().map(|m| m.len()).unwrap_or(0);
    if file_size < 100_000 {
        let err_msg = format!("Downloaded update is corrupted or incomplete (size: {} bytes)", file_size);
        let _ = app.emit("update-download-error", err_msg.clone());
        return Err(err_msg);
    }

    info!("Update binary downloaded successfully ({} bytes). Emitting completion event.", file_size);
    let _ = app.emit("update-download-progress", DownloadProgressPayload {
        percentage: 100,
        downloaded_bytes: file_size,
        total_bytes: file_size,
    });
    let _ = app.emit("update-download-complete", ());

    Ok(())
}

/// Automatically replaces the running executable with the newly downloaded version and smoothly restarts.
pub fn install_update_and_restart() -> Result<(), String> {
    let current_exe = std::env::current_exe()
        .map_err(|e| format!("Unable to determine current executable path: {}", e))?;
    let temp_update = get_update_temp_path();

    if !temp_update.exists() {
        return Err(format!("Update binary not found at {:?}", temp_update));
    }

    let file_size = temp_update.metadata().map(|m| m.len()).unwrap_or(0);
    if file_size < 100_000 {
        return Err(format!("Update file at {:?} is incomplete ({} bytes)", temp_update, file_size));
    }

    info!("Replacing current executable {:?} with {:?}", current_exe, temp_update);

    // Strategy 1: Direct in-process NTFS rename
    // On Windows NTFS, a running executable can be renamed even while executing.
    let backup_exe = current_exe.with_extension("exe.old");
    if backup_exe.exists() {
        let _ = fs::remove_file(&backup_exe);
    }

    let mut direct_replaced = false;
    if fs::rename(&current_exe, &backup_exe).is_ok() {
        if fs::copy(&temp_update, &current_exe).is_ok() {
            direct_replaced = true;
            info!("Direct in-process executable replacement succeeded!");
        } else {
            // Restore backup if copy failed
            let _ = fs::rename(&backup_exe, &current_exe);
        }
    }

    if direct_replaced {
        info!("Spawning updated executable and exiting current process...");
        if let Ok(_) = Command::new(&current_exe).spawn() {
            // Clean up downloaded temp file
            let _ = fs::remove_file(&temp_update);
            std::process::exit(0);
        }
    }

    // Strategy 2: Detached supervisor helper
    // If direct in-process replacement was blocked, spawn a detached script that waits for current process exit,
    // overwrites current_exe with temp_update, launches current_exe, and cleans up.
    let current_str = current_exe.to_string_lossy().to_string();
    let temp_str = temp_update.to_string_lossy().to_string();
    let updater_script = std::env::temp_dir().join("autodarkmode_updater.bat");

    let script_content = format!(
        "@echo off\r\ntimeout /t 1 /nobreak > nul\r\n:retry\r\nmove /Y \"{}\" \"{}\" > nul 2>&1\r\nif errorlevel 1 (\r\n    timeout /t 1 /nobreak > nul\r\n    goto retry\r\n)\r\nstart \"\" \"{}\"\r\ndel \"%~f0\"\r\n",
        temp_str, current_str, current_str
    );

    fs::write(&updater_script, script_content)
        .map_err(|e| format!("Failed to create detached updater script: {}", e))?;

    let mut helper_cmd = Command::new("cmd.exe");
    helper_cmd.args(["/C", &updater_script.to_string_lossy()]);

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        const DETACHED_PROCESS: u32 = 0x00000008;
        helper_cmd.creation_flags(CREATE_NO_WINDOW | DETACHED_PROCESS);
    }

    helper_cmd.spawn()
        .map_err(|e| format!("Failed to launch helper updater process: {}", e))?;

    info!("Supervisor updater script launched. Terminating current process for replacement...");
    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_temp_path() {
        let path = get_update_temp_path();
        assert!(path.to_string_lossy().ends_with("autodarkmode_update.exe"));
    }

    #[test]
    fn test_download_progress_payload_serialization() {
        let payload = DownloadProgressPayload {
            percentage: 42,
            downloaded_bytes: 4200,
            total_bytes: 10000,
        };
        let json = serde_json::to_string(&payload).expect("Serialization failed");
        assert!(json.contains("\"percentage\":42"));
        assert!(json.contains("\"downloaded_bytes\":4200"));
        assert!(json.contains("\"total_bytes\":10000"));
    }
}
