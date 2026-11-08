# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.3.0] - 2026-11-08

### Added
- **Command-Line Interface (CLI)**: Added headless CLI operations to the main executable with parent console attachment (`AttachConsole(ATTACH_PARENT_PROCESS)`) supporting `--dark`, `--light`, `--toggle`, `--status`, `--postpone <minutes>`, `--resume`, `--version`, and `--help`.
- **Application Theme Synchronization**:
  - **Microsoft Office (2016/2019/2021/365)**: Synchronizes `HKCU\Software\Microsoft\Office\16.0\Common\UI Theme` between Black (4) and Colorful (0).
  - **Visual Studio Code**: Atomically updates `%APPDATA%\Code\User\settings.json` (`workbench.colorTheme`).
  - **Windows Terminal**: Synchronizes packaged and unpackaged `settings.json` (`theme: "dark" | "light"`).
- **Windows Grayscale Color Filter**: Integrated native `HKCU\Software\Microsoft\ColorFiltering` and DWM refresh, with dedicated UI toggle in Personalization.
- **Windows Touch Keyboard Theme**: Synchronizes `HKCU\Software\Microsoft\TabletTip\1.7\SelectedTheme` (`DefaultDark` / `DefaultLight`), with dedicated UI toggle in Personalization.
- **Native Action Center Notifications**: Added asynchronous Windows Toast / Balloon notification dispatch on theme transitions when `notify_on_switch` is enabled.
- **Custom Scripts Execution Engine**: Added background process execution for `.ps1`, `.bat`, `.cmd`, and `.exe` scripts with `CREATE_NO_WINDOW`.

---

## [1.2.0] - 2026-11-08

### Added
- **Real GitHub Releases Update Verification**: Replaced the static update stub with a full asynchronous update checking engine that queries GitHub's public Releases API (`api.github.com/repos/nitinduseja/auto-dark-mode/releases/latest`).
- **Semantic Version Parser & Comparator**: Integrated semver parsing and comparison to accurately detect newer production releases and compare against the current installation.
- **Direct Release & Asset Download UI**: Added an interactive release banner in Automation and Settings with direct buttons to view release notes and download release `.exe` binaries directly from GitHub.
- **Startup Background Update Verification**: When "Automatic Update Verification" is enabled, AutoDarkMode silently queries for updates in the background upon application launch.
- **Settings & About Integration**: Added real-time "Check for Updates" triggers and status display directly inside the About card on the Settings page.
- **Dynamic Version Synchronization**: Centralized version referencing so all UI components, status bars, tray flyouts, and diagnostics dynamically reflect the project's semantic release version without hardcoded strings.

### Fixed
- **System Tray Flyout Dark Mode**: Fixed missing window transparency (`.transparent(true)`), dropped Tailwind `/95` opacity utilities, and applied Win32 DWM immersive dark mode (`DWMWA_USE_IMMERSIVE_DARK_MODE`) to the tray popup HWND so the flyout matches Windows 11 dark theme.
- **Tray Flyout Theme Re-synchronization**: Added event dispatching and window focus listeners so the tray flyout immediately re-synchronizes theme state when toggled.

---

## [1.1.0] - 2026-11-06

### Added
- **Native File Picker for Wallpapers**: Integrated native Win32 `GetOpenFileNameW` dialog with image filters (`.jpg`, `.png`, `.jpeg`, `.bmp`, `.webp`) allowing one-click wallpaper selection.
- **Postpone Switch & Quick Resume**: Added flexible transition deferral (`postpone_switch`) and schedule resumption (`resume_automation`) directly accessible from the Overview dashboard and System Tray context menu.
- **Native System Tray Context Menu**: Implemented Win32 right-click tray context menu featuring Open Dashboard, Light/Dark toggles, Postpone 1 hour, Resume Automation, Settings, and Clean Exit.
- **DPI-Aware Multi-Monitor Tray Positioning**: Enhanced tray popups to inspect cursor coordinates and anchor accurately on multi-monitor setups using `MonitorFromPoint`.
- **Automatic Location Detection**: Added instant local timezone-based geographic coordinate resolver in Schedule settings.
- **Accent Color, Cursor, and Sound Synchronization**: Implemented Windows personalization registry hooks for DWM accent colors (`apply_accent_color`), mouse cursor schemes (`apply_cursor_scheme`), and sound schemes (`apply_sound_scheme`).
- **User Idle Detection**: Integrated `GetLastInputInfo` to prevent disruptive visual switches while actively interacting with the computer.
- **Full Mica Material Support**: Configured window transparency and translucent Fluent design tokens so Windows 11 DWM Mica material shines cleanly through the application surface.
- **Automated CI/CD Release Trigger**: Updated GitHub Actions release workflow to trigger on code pushes to `main` as well as tags, and added manual `workflow_dispatch` trigger.

### Fixed
- **Reversion Fighting Loop**: Fixed manual theme switches being overwritten within 2 seconds by introducing `ManualOverride` state in the background scheduler.
- **Battery Saver Inversion**: Corrected power conditions logic to force Dark Mode instead of Light Mode when battery saver is active.
- **Fullscreen Game Detection False Positives**: Replaced screen bounds heuristics with `SHQueryUserNotificationState` to avoid falsely flagging normal maximized windows as full-screen DirectX games.
- **Inverted Schedule Timeline Math**: Fixed percentage calculations and countdown display for inverted / night-shift schedules crossing midnight.
- **Fresh Checkout Compile Panic**: Committed `dist/.gitkeep` ensuring frontend asset directories exist on clean git checkouts.
- **Excessive Disk I/O & Polling**: Replaced aggressive disk-reading loops with an in-memory `AppState` cache (`Arc<RwLock<AppConfig>>`) and event-driven Svelte store reactions.
- **Real Environment Diagnostics**: Updated diagnostics to report authentic Windows 11 OS builds, process uptime, and system architecture.

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
