# Contributing to AutoDarkMode

Thank you for your interest in contributing to AutoDarkMode! This project is an open-source, lightweight Windows 11 theme switcher built using Tauri 2, Svelte 5, and Rust.

Please take a moment to review this guide before submitting issues or pull requests.

---

## 1. Code of Conduct

By participating in this project, you agree to abide by the [Code of Conduct](CODE_OF_CONDUCT.md).

---

## 2. Prerequisites & Development Environment

To build and run AutoDarkMode locally, ensure you have:

- **Operating System:** Windows 10 (version 1809+) or Windows 11 x64 (Windows 11 22H2+ recommended for native Mica support)
- **Rust:** Stable toolchain (`rustup default stable`)
- **Node.js:** v18 or v20+ with npm
- **Visual Studio C++ Build Tools:** Required by the Rust Windows MSVC toolchain
- **WebView2 Runtime:** Pre-installed on Windows 11; install the evergreen runtime if on older Windows 10 builds

---

## 3. Project Architecture

The repository maintains a strict separation between frontend and backend:

```text
AutoDarkMode/
├── docs/                        # Project documentation & screenshots
│   └── screenshots/             # Application UI screenshots
│
├── src/                         # Frontend (Svelte 5 runes, TypeScript, Tailwind CSS)
│   ├── components/              # UI components (dashboard, schedule, apps, personalization, etc.)
│   ├── services/                # Typed Tauri IPC service wrapper with browser fallbacks
│   ├── stores/                  # Svelte 5 reactive stores ($state classes)
│   ├── styles/                  # Tailwind CSS & Fluent theme design tokens
│   ├── types/                   # Shared TypeScript interfaces
│   ├── App.svelte               # Root component (handles main window & tray flyout views)
│   └── main.ts                  # Frontend entry point
│
├── src-tauri/                   # Backend (Tauri 2 & Rust)
│   ├── capabilities/            # Tauri 2 security capabilities (least-privilege IPC access)
│   ├── icons/                   # Multi-resolution application & tray icons
│   ├── src/                     # Rust source code
│   │   ├── commands/            # Tauri IPC command handlers
│   │   ├── config/              # Atomic persistent JSON configuration manager
│   │   ├── scheduler/           # Background theme evaluation daemon
│   │   ├── state/               # Astronomical solar math & timeline computations
│   │   ├── tray/                # Windows notification area / system tray integration
│   │   ├── windows/             # Win32 APIs (Registry, DWM Mica, Power, Gaming, Themes)
│   │   ├── lib.rs               # Application lifecycle & command registration
│   │   └── main.rs              # Native Windows entry point
│   ├── Cargo.toml               # Rust dependencies & optimization profiles
│   ├── build.rs                 # Tauri build hook
│   └── tauri.conf.json          # Window configuration, bundles, and metadata
│
├── .github/                     # Workflows and templates
├── package.json                 # Frontend dependencies and scripts
└── tsconfig.json                # Strict TypeScript configuration
```

---

## 4. Getting Started

### Clone the Repository

```powershell
git clone https://github.com/nitinduseja/auto-dark-mode.git
cd auto-dark-mode
```

### Install Dependencies

```powershell
npm install
```

### Run Frontend in Vite Development Server

For rapid UI design iteration with browser fallbacks:

```powershell
npm run dev
```

### Run Full Desktop Application with Tauri

To test native Windows Registry changes, Mica backdrop, and system tray:

```powershell
npm run tauri dev
```

---

## 5. Verification & Testing

Before submitting a pull request, verify that all automated checks pass:

```powershell
# 1. Typecheck Svelte 5 and TypeScript
npm run check

# 2. Build frontend production bundles
npm run build

# 3. Run backend unit tests
cargo test --manifest-path src-tauri/Cargo.toml --features custom-protocol
```

---

## 6. Coding Guidelines

### Frontend (Svelte 5 & TypeScript)
- Use Svelte 5 runes (`$state`, `$derived`, `$props`) instead of legacy Svelte 3/4 reactive declarations (`let`, `$:`) or writable stores.
- Isolate all Tauri IPC calls inside `src/services/tauri.ts` with browser-compatible fallback data.
- Use `@lucide/svelte` for icons.
- Ensure strict TypeScript typing with zero `any` declarations in production code.

### Backend (Rust & Tauri 2)
- Follow the principle of least privilege in `src-tauri/capabilities/default.json`.
- Preserve the existing IPC command contracts and names.
- Write unit tests for business logic (solar calculations, configuration migrations, time scheduling).
- Ensure all OS registry operations verify success and gracefully handle access restrictions.

---

## 7. Submitting Pull Requests

1. Create a feature branch from `main`:
   ```powershell
   git checkout -b feat/your-feature-name
   ```
2. Commit your changes following [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat: add sunrise offset preview`
   - `fix: prevent tray flyout clipping on high-DPI displays`
   - `docs: update build instructions in README`
3. Push your branch and open a Pull Request targeting `main`.
4. Ensure all CI checks pass.
