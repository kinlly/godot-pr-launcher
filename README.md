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

- Node.js 18+
- Rust 1.70+
- Godot installed at `C:\repos\godot.exe`
- Project cloned at `C:\repos\ylbtm`

### Setup

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

The built executable will be in `src-tauri/target/release/`

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
