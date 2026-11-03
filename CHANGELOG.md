# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.0] - 2026-11-02

### Added
- **Complete Windows 11 Navigation Suite:**
  - **Overview Dashboard:** Dynamic 24-hour solar timeline, current appearance status, next switch countdown, and quick toggle controls.
  - **Schedule View:** Astronomical sunrise/sunset daylight tracking with NOAA solar algorithms, geographic coordinates, manual custom time windows, and offset controls.
  - **Apps View:** Application-specific theme overrides and process rules (VS Code, Windows Terminal, etc.).
  - **Personalization View:** Day/Night desktop wallpaper switching (`SystemParametersInfoW`), accent color synchronization, and installed `.theme` file integration.
  - **Conditions View:** Intelligent switching inhibition during full-screen gaming (`SHQueryUserNotificationState`) and on-battery power saving (`GetSystemPowerStatus`).
  - **Automation View:** Native Windows startup registration (`HKCU\...\Run`), background daemon mode, and notification preferences.
  - **Shortcuts View:** Global customizable keyboard shortcuts for instant theme toggles and postponing transitions.
  - **Settings View:** Configuration import/export with schema validation, one-click reset to defaults, and diagnostic environment reporting.
- **Windows 11 Native Visuals:**
  - DWM Mica backdrop integration (`DWMWA_SYSTEMBACKDROP_TYPE`) with immersive dark titlebar support.
  - Windows 11 Fluent Nightfall / Bloom design token palette with Tailwind CSS.
- **Dual-Window & Tray Architecture:**
  - Modern Windows 11 / OneDrive-styled compact system tray flyout anchored above the taskbar.
  - Persistent background daemon minimizing to tray on window close.
  - Single-instance enforcement bringing the existing window forward when re-launched.
- **Robust Persistence:**
  - Atomic JSON configuration storage in `%APPDATA%\AutoDarkMode\config.json` with swap files and automatic recovery from corruption.
- **Open-Source Infrastructure:**
  - Standardized `src/` (Svelte 5) and `src-tauri/` (Rust) directory architecture.
  - MIT License and open-source contribution guidelines (`CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`).
  - Automated GitHub Actions CI workflow for Windows builds and unit testing.
