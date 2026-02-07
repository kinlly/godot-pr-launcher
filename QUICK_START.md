# Quick Start - Private Repository Access

## TL;DR (Too Long; Didn't Read)

Your repo is private? Here's the fastest way to get it working:

### Windows (Easiest)

1. **Get your token:**
   - Go to https://github.com/settings/tokens
   - Click "Generate new token (classic)"
   - Check the `repo` box
   - Copy the token (starts with `ghp_`)

2. **Method A - Batch File (Recommended):**
   ```batch
   # Edit launch-with-token.bat
   # Replace YOUR_GITHUB_TOKEN_HERE with your token
   # Double-click to run!
   ```

3. **Method B - Quick File:**
   ```batch
   # Create file: .github-token (next to .exe)
   # Paste your token in it (just the token, nothing else)
   # Run godot-pr-launcher.exe normally
   ```

4. **Method C - Command Line:**
   ```batch
   set GITHUB_TOKEN=ghp_your_token_here
   godot-pr-launcher.exe
   ```

### Linux/Mac

1. **Get token** (same as above)

2. **Option 1 - File:**
   ```bash
   echo "ghp_your_token_here" > ~/.github-token
   ./godot-pr-launcher
   ```

3. **Option 2 - Environment:**
   ```bash
   export GITHUB_TOKEN=ghp_your_token_here
   ./godot-pr-launcher
   ```

## That's It!

The app will automatically:
- ✅ Find your token
- ✅ Use it to authenticate with GitHub
- ✅ Access your private repository
- ✅ Load all your private PRs

## Need More Details?

See [GITHUB_TOKEN_SETUP.md](GITHUB_TOKEN_SETUP.md) for:
- Security best practices
- Troubleshooting
- Advanced configuration
- Token scope details

## Still Not Working?

Check:
1. Token has `repo` scope (not just `public_repo`)
2. Token hasn't expired
3. No extra spaces in .github-token file
4. File is named `.github-token` not `.github-token.txt`
5. You rebuilt the app: `npm run tauri build`
