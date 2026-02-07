# 🚀 Setup Guide - Godot PR Launcher

## Project Overview

This Tauri desktop application is designed to run on **Windows** (Ally ROX) and provides a GUI to:
1. View open Pull Requests from `kinlly/ylbtm`
2. Launch Godot with the project when clicking on a PR

## ✅ What Has Been Created

All necessary files have been created and are ready to use:

### Frontend (React + Vite)
- ✅ `package.json` - Node.js dependencies
- ✅ `vite.config.js` - Vite configuration
- ✅ `tailwind.config.js` - TailwindCSS styling
- ✅ `postcss.config.js` - PostCSS configuration
- ✅ `index.html` - HTML entry point
- ✅ `src/main.jsx` - React entry point
- ✅ `src/App.jsx` - Main application component
- ✅ `src/index.css` - Global styles with Tailwind
- ✅ `src/components/PRList.jsx` - PR list component
- ✅ `src/components/LoadingSpinner.jsx` - Loading spinner component

### Backend (Rust + Tauri)
- ✅ `src-tauri/Cargo.toml` - Rust dependencies
- ✅ `src-tauri/tauri.conf.json` - Tauri configuration
- ✅ `src-tauri/build.rs` - Build script
- ✅ `src-tauri/src/main.rs` - Main Tauri application
- ✅ `src-tauri/src/github.rs` - GitHub API integration
- ✅ `src-tauri/src/godot.rs` - Godot launcher

### Documentation
- ✅ `README.md` - Project documentation
- ✅ `IMPLEMENTATION_PLAN.md` - Implementation roadmap
- ✅ `.gitignore` - Git ignore rules

## 🔧 Next Steps on Windows (Ally ROX)

### 1. Install Prerequisites

#### Install Rust
```powershell
# Download and run rustup-init.exe
# Or use winget:
winget install Rustlang.Rust.MSVC
```

#### Install Node.js
```powershell
winget install OpenJS.NodeJS.LTS
```

#### Install Microsoft C++ Build Tools
Download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/
- Select "Desktop development with C++"

### 2. Clone and Setup

```powershell
# Clone the repository
git clone https://github.com/kinlly/godot-pr-launcher.git
cd godot-pr-launcher

# Checkout the branch
git checkout copilot/create-desktop-app-using-tauri

# Install Node.js dependencies
npm install

# Install Tauri CLI (if not already installed)
npm install -g @tauri-apps/cli
```

### 3. Development Mode

```powershell
# Run in development mode
npm run tauri dev
```

This will:
1. Start the Vite dev server on http://localhost:1420
2. Launch the Tauri window with the app
3. Enable hot reload for frontend changes

### 4. Build for Production

```powershell
# Build the application
npm run tauri build
```

The executable will be created in:
- `src-tauri/target/release/godot-pr-launcher.exe`
- Installer: `src-tauri/target/release/bundle/msi/godot-pr-launcher_0.1.0_x64_en-US.msi`

### 5. Using the Application

1. Double-click `godot-pr-launcher.exe`
2. The app will load PRs from `kinlly/ylbtm`
3. Click any PR to launch Godot with: `C:\repos\godot.exe --path c:\repos\ylbtm`

## 📋 Configuration

### Customize Godot Path

Edit `src-tauri/src/godot.rs` to change the Godot executable path:

```rust
let output = Command::new("C:\\repos\\godot.exe")  // Change this path
    .args(&["--path", "c:\\repos\\ylbtm"])          // Change this path
    .spawn()
```

### Customize Repository

Edit `src-tauri/src/main.rs` to change the GitHub repository:

```rust
github::fetch_prs("kinlly", "ylbtm")  // Change owner and repo
```

## 🐛 Troubleshooting

### "Failed to fetch PRs"
- Check internet connection
- GitHub API has rate limits (60 requests/hour without authentication)
- Consider adding a GitHub token for higher limits

### "Failed to launch Godot"
- Verify Godot is installed at `C:\repos\godot.exe`
- Verify project exists at `C:\repos\ylbtm`
- Check Windows permissions

### Build Errors on Windows
- Ensure Visual Studio C++ Build Tools are installed
- Restart terminal after installing Rust/Node.js
- Run `cargo clean` in `src-tauri` folder if dependencies are corrupted

## 🔮 Future Enhancements (Phase 2)

See `IMPLEMENTATION_PLAN.md` for planned features:
- GitHub authentication
- Comment on PRs
- Merge PRs
- GitHub Copilot CLI integration
- PR chat interface

## 📄 License

MIT

---

## Development Notes

### Frontend Build Tested ✅
The React + Vite + TailwindCSS setup has been tested and builds successfully:
- Build output: `dist/` directory
- Bundle size: ~147KB (gzipped: 47KB)
- CSS: ~10KB (gzipped: 2.8KB)

### Backend (Rust) Notes
- The Tauri backend compiles on Windows with proper dependencies
- Linux compilation requires GTK/WebKit system libraries (for development only)
- The app is designed for Windows deployment

### Known Limitations
- Godot path is hardcoded to Windows (`C:\repos\godot.exe`)
- No authentication (uses public GitHub API)
- No PR checkout - just launches Godot with the main project
