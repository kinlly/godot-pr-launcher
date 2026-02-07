# Debug Output Examples

## Example 1: Token Found and Working

```
=== GitHub API Debug Info ===
Repository: kinlly/ylbtm
API URL: https://api.github.com/repos/kinlly/ylbtm/pulls?state=open

✓ Token found in GITHUB_TOKEN env var: ghp_1a...xyz89
✓ Authorization header added to request
=============================

✓ Successfully loaded 5 pull request(s)
```

## Example 2: Token Not Found (Public API)

```
=== GitHub API Debug Info ===
Repository: kinlly/ylbtm
API URL: https://api.github.com/repos/kinlly/ylbtm/pulls?state=open

✗ GITHUB_TOKEN env var not set
Checking: C:\repos\godot-pr-launcher\.github-token
✗ .github-token file not found or not readable
Checking: C:\Users\YourName\.github-token
✗ .github-token file not found or not readable

⚠️  No GitHub token found. Using unauthenticated API (rate limited to 60 req/hour)
✗ No authorization header (using public API)
=============================
```

## Example 3: Token Found in Config File

```
=== GitHub API Debug Info ===
Repository: kinlly/ylbtm
API URL: https://api.github.com/repos/kinlly/ylbtm/pulls?state=open

✗ GITHUB_TOKEN env var not set
Checking: C:\repos\godot-pr-launcher\.github-token
✓ Token found in exe directory: ghp_Ab...De123
✓ Authorization header added to request
=============================
```

## Example 4: 404 Error with Debug Info

```
Error loading PRs

GitHub API error (404 Not Found):

Not Found

Debug Information:
✗ GITHUB_TOKEN env var not set
Checking: C:\repos\godot-pr-launcher\.github-token
✗ .github-token file not found or not readable
Checking: C:\Users\YourName\.github-token
✗ .github-token file not found or not readable

⚠️  No GitHub token found. Using unauthenticated API (rate limited to 60 req/hour)

Repository: kinlly/ylbtm

💡 Possible causes:
• Repository doesn't exist
• Repository name is incorrect
• Repository is private and token is missing/invalid
```

## Example 5: 401 Auth Error with Debug Info

```
Error loading PRs

GitHub API error (401 Unauthorized):

Bad credentials

Debug Information:
✓ Token found in GITHUB_TOKEN env var: ghp_old...token9
✓ Authorization header added to request

Repository: kinlly/ylbtm

💡 Authentication failed:
• Token is invalid or expired
• Create new token at: https://github.com/settings/tokens
```

## Example 6: Token Found in Home Directory

```
=== GitHub API Debug Info ===
Repository: kinlly/ylbtm
API URL: https://api.github.com/repos/kinlly/ylbtm/pulls?state=open

✗ GITHUB_TOKEN env var not set
Checking: C:\repos\godot-pr-launcher\.github-token
✗ .github-token file not found or not readable
Checking: C:\Users\YourName\.github-token
✓ Token found in home directory: ghp_xy...abc12
✓ Authorization header added to request
=============================
```

## How to View Debug Output

**Method 1: Command Line (Recommended)**
```bash
# Run from command prompt to see console output
cd C:\repos\godot-pr-launcher
godot-pr-launcher.exe
```

**Method 2: Development Mode**
```bash
# Debug info appears in terminal
npm run tauri dev
```

**Method 3: In Error Messages**
When an error occurs, the debug info is automatically included in the error message shown in the UI.

## What to Check

1. **Token Masking**: Verify the masked token matches your actual token
   - Example: If your token is `ghp_123456789abcdefghijklmnop`, it shows as `ghp_1...mnop`

2. **File Paths**: Check if the paths are correct
   - Windows: `C:\repos\godot-pr-launcher\.github-token`
   - Home dir: `C:\Users\YourName\.github-token`

3. **Authorization Status**: Confirm "Authorization header added" appears

4. **Error Hints**: Read the specific suggestions for your error code

## Troubleshooting

If you see:
- ✗ marks: That location was checked but token not found
- ✓ mark: Token successfully loaded from that location
- Token mask doesn't match: Check for extra spaces or wrong token
- No ✓ anywhere: Token file doesn't exist or is in wrong location
