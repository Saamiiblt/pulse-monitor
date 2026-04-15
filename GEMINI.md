# GEMINI.md - PulseMonitor

## Project Overview
PulseMonitor is a lightweight desktop application built with **Tauri** that provides real-time monitoring of system resources (CPU and RAM usage). It features a "RAM Optimization" tool that attempts to free up memory by clearing the working sets of active processes on Windows.

- **Frontend:** Vanilla JavaScript, HTML, and CSS.
- **Backend:** Rust (Tauri), using the `sysinfo` crate for hardware statistics.
- **Target OS:** Primarily Windows (due to the use of PowerShell for RAM optimization).

## Architecture
The application follows the standard Tauri pattern:
- **Frontend (`src/`):**
  - `index.html`: UI structure.
  - `main.js`: Handles periodic UI updates (every 1s) and invokes Rust commands.
  - `style.css`: Minimalist, dark-themed styling.
- **Backend (`src-tauri/`):**
  - `main.rs`: Optimized to only refresh CPU and Memory data (selective refresh).
  - `Cargo.toml`: Configured with release profiles (LTO, opt-level "s", panic "abort") for minimal binary size.
  - `tauri.conf.json`: Security-hardened with a custom CSP and minimal API allowlist.

## Key Commands (Rust/JS Bridge)
- `get_stats`: Returns current CPU usage (%), used RAM (MB), and total RAM (MB).
- `optimize_ram`: Executes a PowerShell command to call `.EmptyWorkingSet()` on processes with a working set > 5MB.

## Building and Running

### Prerequisites
- Node.js and npm
- Rust toolchain (Cargo)
- Tauri CLI (`npm install -g @tauri-apps/cli` or use `npm run tauri`)

### Development
```bash
npm run dev
# or
npm run tauri dev
```

### Production Build
```bash
npm run build
# or
npm run tauri build
```

## Development Conventions
- **UI Constraints:** The window is set to 350x400 pixels, non-resizable, and supports transparency (as configured in `tauri.conf.json`).
- **State Management:** The Rust backend maintains a `sysinfo::System` instance in a Mutex-protected `AppState` to efficiently query system stats.
- **Error Handling:** Frontend uses `try...catch` for IPC calls to gracefully handle potential communication errors with the Rust core.
