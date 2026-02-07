# 🔧 Troubleshooting Guide

## Common Issues and Solutions

### Error: "error decoding response body: invalid type: map, expected a sequence"

**What it means:**  
The application is receiving an unexpected response from GitHub's API. Instead of receiving a list of pull requests (an array), it's receiving a single object (usually an error message).

**Common Causes:**

1. **Repository doesn't exist**
   - The repository `kinlly/ylbtm` may not exist on GitHub
   - Check: https://github.com/kinlly/ylbtm

2. **Repository is private**
   - If the repository is private, the public API cannot access it
   - Solution: Make the repository public, or add GitHub authentication (planned for Phase 2)

3. **GitHub API Rate Limiting**
   - GitHub limits unauthenticated requests to 60 per hour
   - Wait an hour or implement GitHub authentication

4. **Network/Connection Issues**
   - Check your internet connection
   - Verify GitHub.com is accessible

5. **Repository name typo**
   - Current configuration: `kinlly/ylbtm`
   - Update in `src-tauri/src/main.rs` line 19 if different

**How to Fix:**

1. **Verify the repository exists and is public:**
   ```
   Visit: https://github.com/kinlly/ylbtm
   ```

2. **Change the repository** (if needed):
   Edit `src-tauri/src/main.rs`:
   ```rust
   github::fetch_prs("your-username", "your-repo")
   ```

3. **Check the actual error message:**
   - After the fix in commit ca46104, you should see a more descriptive error
   - Example: "GitHub API error (404): Not Found" means the repository doesn't exist

4. **Wait for rate limit reset:**
   - If rate limited, wait 1 hour or add GitHub authentication

### Error: "icons/icon.ico not found"

**Status:** ✅ Fixed in commit 5a8d6e1

All required icon files have been created in `src-tauri/icons/`:
- 32x32.png
- 128x128.png
- 128x128@2x.png
- icon.ico (Windows)
- icon.icns (macOS)

### Build fails on Windows

**Check:**
1. Rust is installed: `rustc --version`
2. Node.js is installed: `node --version`
3. Visual C++ Build Tools are installed
4. All dependencies installed: `npm install`

### Application window is blank

**Possible causes:**
1. Vite dev server not running (in dev mode)
2. Frontend build failed
3. Tauri configuration issue

**Solutions:**
1. Run `npm run dev` separately to check frontend
2. Check browser console in dev mode (F12)
3. Rebuild: `npm run build`

## Getting Help

If you continue experiencing issues:

1. Check the error message in the application UI
2. Look at the console/terminal output
3. Verify the repository at https://github.com/kinlly/ylbtm exists
4. Check GitHub API status: https://www.githubstatus.com/

## Phase 2 Features (Coming Soon)

- GitHub OAuth authentication (fixes rate limiting)
- Support for private repositories
- Better error handling and retry logic
- Offline mode with caching
