# 🔐 GitHub Token Configuration

## Why You Need a Token

If your repository is **private**, GitHub requires authentication to access it. The application supports GitHub Personal Access Tokens for this purpose.

## How to Create a GitHub Token

1. Go to GitHub Settings: https://github.com/settings/tokens
2. Click **"Generate new token"** → **"Generate new token (classic)"**
3. Give it a name: `Godot PR Launcher`
4. Set expiration (recommended: 90 days or custom)
5. Select scopes:
   - ✅ **`repo`** (Full control of private repositories) - **REQUIRED**
6. Click **"Generate token"**
7. **Copy the token immediately** (you won't see it again!)

## Three Ways to Use Your Token

### Option 1: Environment Variable (Recommended for Development)

**Windows:**
```batch
# Set temporarily (current session only)
set GITHUB_TOKEN=ghp_your_token_here
godot-pr-launcher.exe

# OR set permanently (all sessions)
setx GITHUB_TOKEN "ghp_your_token_here"
```

**Linux/Mac:**
```bash
export GITHUB_TOKEN=ghp_your_token_here
./godot-pr-launcher
```

### Option 2: Configuration File (Recommended for Production)

Create a file named `.github-token` in one of these locations:

**Windows:**
```
# Option A: Next to the .exe file
C:\repos\godot-pr-launcher\.github-token

# Option B: In your home directory
C:\Users\YourUsername\.github-token
```

**Linux/Mac:**
```
# Option A: Next to the executable
~/.github-token

# Option B: In the app directory
/path/to/app/.github-token
```

**File contents:**
```
ghp_your_token_here
```

**Important:** 
- No quotes, no extra spaces
- Just the token on a single line
- Make sure the file has no extension (not `.github-token.txt`)

### Option 3: Batch File Launcher (Easiest for Windows)

Use the provided `launch-with-token.bat` file:

1. **Edit the .bat file:**
   ```batch
   # Open launch-with-token.bat in Notepad
   # Replace YOUR_GITHUB_TOKEN_HERE with your actual token
   SET GITHUB_TOKEN=ghp_your_actual_token_here
   ```

2. **Or pass token as parameter:**
   ```batch
   launch-with-token.bat ghp_your_token_here
   ```

3. **Double-click** `launch-with-token.bat` to run

## Security Best Practices

### ✅ DO:
- Keep your token secret
- Use short expiration times (30-90 days)
- Use minimal scopes (only `repo`)
- Store in `.github-token` file with restricted permissions
- Delete tokens you're no longer using

### ❌ DON'T:
- Share your token with anyone
- Commit tokens to git (`.gitignore` includes `.github-token`)
- Use tokens with unnecessary permissions
- Store tokens in public places

## Verifying Token Works

After setting up your token, run the application:
1. The app should now load PRs from your private repository
2. If you see an error, check:
   - Token is correctly copied (no extra spaces)
   - Token has `repo` scope
   - Token hasn't expired
   - Repository name is correct in the code

## Token Priority

The application checks for tokens in this order:
1. **GITHUB_TOKEN** environment variable
2. **.github-token** file in executable directory
3. **.github-token** file in home directory

## Troubleshooting

### "GitHub API error (401): Bad credentials"
- Token is incorrect or expired
- Create a new token

### "GitHub API error (403): Forbidden"
- Token doesn't have `repo` scope
- Recreate token with correct permissions

### "GitHub API error (404): Not Found"
- Repository doesn't exist
- Token doesn't have access to the repository
- Check repository name in code

### Still see "invalid type: map, expected a sequence"?
- Token might not be loaded
- Check token file location
- Verify no extra spaces or quotes in token
- Rebuild the application: `npm run tauri build`

## Example: Complete Setup (Windows)

```batch
# 1. Create token on GitHub (with 'repo' scope)

# 2. Create .github-token file
echo ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx > .github-token

# 3. Place file next to godot-pr-launcher.exe

# 4. Run the application
godot-pr-launcher.exe

# Done! Your private repo PRs should now load
```

## Need Help?

- Check `TROUBLESHOOTING.md` for common issues
- Verify token at: https://github.com/settings/tokens
- GitHub Token Guide: https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token
