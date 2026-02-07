# 🚀 Godot PR Launcher

Desktop application to manage GitHub Pull Requests and launch Godot projects.

## ✨ Features (Phase 1)

- ✅ View all open PRs from `kinlly/ylbtm`
- ✅ Click a PR to launch Godot with the project
- ✅ Beautiful, modern UI with Tauri + React
- ✅ Lightweight (~5MB exe)

## 🛠️ Tech Stack

- **Frontend**: React 18 + Vite + TailwindCSS
- **Backend**: Rust + Tauri 1.5
- **APIs**: GitHub REST API v3

## 📦 Installation

### Prerequisites

**Windows:**
- Node.js 18+
- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Godot installed at `C:\repos\godot.exe`
- Project cloned at `C:\repos\ylbtm`

**Linux (for development):**
- Node.js 18+
- Rust 1.70+
- System dependencies:
  ```bash
  sudo apt-get update
  sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
  ```

### Setup

```bash
# Install JavaScript dependencies
npm install

# Run in development mode (requires Godot at C:\repos\godot.exe)
npm run tauri dev

# Build for production (Windows)
npm run tauri build
```

The built executable will be in `src-tauri/target/release/`

### Notes

- The application is designed primarily for **Windows** with hardcoded paths to Godot.
- On Linux, you can develop the UI but the Godot launcher will need path adjustments.

## 🎯 Usage

1. Double-click `godot-pr-launcher.exe`
2. View the list of open PRs
3. Click any PR to launch Godot with the ylbtm project
4. Godot runs in background while you review the PR

## 🔮 Roadmap (Phase 2)

- [ ] Comment on PRs directly from the app
- [ ] Merge PRs with one click
- [ ] GitHub Copilot CLI integration
- [ ] Chat interface for PR discussions

## 📄 License

MIT
