# PulseMonitor ⚡

PulseMonitor is a sleek, lightweight system resource monitor designed for Windows. Built with **Tauri** and **Rust**, it provides real-time insights into your system's performance with a minimalist, premium-feel interface.

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![Platform](https://img.shields.io/badge/platform-Windows-brightgreen)
![License](https://img.shields.io/badge/license-MIT-orange)

## ✨ Features

- **Real-time Monitoring**: Track CPU usage and RAM consumption with sub-second accuracy.
- **RAM Optimization**: A one-click "Booster" tool that clears the working sets of active processes, freeing up physical memory.
- **Glassmorphism Design**: A beautiful, transparent dark-themed UI that feels modern and unobtrusive.
- **Lightweight & Fast**: Extremely low memory footprint thanks to the Rust backend and minimal app size.

## 🛠️ Tech Stack

- **Backend**: [Rust](https://www.rust-lang.org/) (Tauri Framework)
- **Frontend**: Vanilla JavaScript, HTML5, CSS3
- **System Stats**: `sysinfo` crate for high-performance hardware querying.
- **Styles**: Custom CSS with modern gradients and micro-animations.

## 🚀 Getting Started

### Installation
You can download the latest installer from the [Releases](https://github.com/Saamiiblt/pulse-monitor/releases) page.

1. Download `PulseMonitor_0.1.0_x64-setup.exe`.
2. Run the installer.
3. Launch **PulseMonitor** from your desktop or start menu.

### Development
If you want to build the project from source:

1. **Prerequisites**:
   - [Node.js](https://nodejs.org/)
   - [Rust](https://rustup.rs/)
   - Tauri CLI: `npm install -g @tauri-apps/cli`

2. **Run Dev Mode**:
   ```bash
   npm run dev
   ```

3. **Build Release**:
   ```bash
   npm run build
   ```

## 🏗️ Architecture

- **`src-tauri/`**: The Rust core. It manages system state and executes low-level PowerShell commands for memory optimization.
- **`src/`**: The web-based frontend. It communicates with the Rust backend via asynchronous IPC (Inter-Process Communication).
- **Security**: Hardened with custom Content Security Policy (CSP) and restricted API access.

---
Built for performance, designed for aesthetics.
