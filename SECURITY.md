# Security Policy

## Supported Versions

Only the latest release of AutoDarkMode receives security updates and vulnerability patches.

| Version | Supported          |
| ------- | ------------------ |
| 1.1.x   | :white_check_mark: |
| 1.0.x   | :white_check_mark: |
| < 1.0.0 | :x:                |

## Reporting a Vulnerability

The AutoDarkMode team takes the security of our application and users seriously.

If you believe you have discovered a security vulnerability in AutoDarkMode, please report it privately:

1. **Do not create a public GitHub issue.**
2. Report security vulnerabilities via [GitHub Private Vulnerability Reporting](https://github.com/nitinduseja/auto-dark-mode/security/advisories/new) or contact the maintainers directly.
3. Provide detailed steps to reproduce the issue, including operating system version, reproduction scripts or configuration, and potential impact.

### Response Timeline

- **Initial Acknowledgement:** Within 48 hours.
- **Triage & Assessment:** Within 5 business days.
- **Fix & Public Advisory:** Once verified, a patch will be prepared and published alongside a coordinated security release.

## Security Architecture & Design Principles

- **Least Privilege:** Tauri IPC capabilities explicitly restrict window controls and event listening without granting arbitrary filesystem access or arbitrary process execution to the webview context.
- **Offline & Private:** AutoDarkMode operates 100% offline. No telemetry, user data, location queries, or diagnostic information are transmitted across the network.
- **Atomic Persistence:** Local configuration changes are written atomically via temporary swap files to prevent file corruption during power failures or system restarts.
